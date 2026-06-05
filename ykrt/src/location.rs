//! Trace location: track the state of a program location (counting, tracing, compiled, etc).

use std::{
    mem,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
};

use crate::{
    compile::CompiledTrace,
    mt::{HotThreshold, MT, TraceCompilationErrorThreshold, TraceId},
};
use parking_lot::Mutex;

#[cfg(target_pointer_width = "64")]
const STATE_TAG_MASK: usize = 0b11; // Bits 0-1: state kind (null / not-hot / hot).
#[cfg(target_pointer_width = "64")]
// Bit 2: method-entry flag, only meaningful when STATE_NOT_HOT. Arc<Mutex<HotLocation>> is at
// least 8-byte aligned, so bits 0-2 of the pointer are always 0 and safe to use as tag bits.
const STATE_ENTRY_BIT: usize = 0b100;
#[cfg(target_pointer_width = "64")]
// Combined mask for extracting a pointer stored in a hot Location (clear bits 0-2).
const STATE_FULL_MASK: usize = STATE_TAG_MASK | STATE_ENTRY_BIT;
#[cfg(target_pointer_width = "64")]
// Count / pointer payload begins at bit 3 (one extra bit vs. the old layout).
const STATE_NUM_BITS: usize = 3;

const STATE_NULL: usize = 0b00;
/// The tag value for a not-yet-hot [Location]. Because null [Location]s have an inner value of 0,
/// this value *must* be non-zero. To derive the count of a not-yet-hot [Location], we have to do
/// `(inner & !STATE_TAG_MASK) >> STATE_NUM_BITS` to derive the count.
const STATE_NOT_HOT: usize = 0b01;
/// The tag value for a hot [Location]; its [HotLocation] address will be contained in the non-tag
/// bits.
const STATE_HOT: usize = 0b10;

/// A `Location` stores state that the meta-tracer needs to identify hot loops and run associated
/// machine code.
///
/// Each position in the end user's program that may be a control point (i.e. the possible start of
/// a trace) must have an associated `Location`. The `Location` does not need to be at a stable
/// address in memory and can be freely moved.
///
/// Program positions that can't be control points don't need an associated `Location`. For
/// interpreters that can't (or don't want) to be as selective, a simple (if moderately wasteful)
/// mechanism is for every bytecode or AST node to have its own `Location` (even for bytecodes or
/// nodes that can't be control points).
#[repr(C)]
#[derive(Debug)]
pub struct Location {
    /// A Location is a state machine. "Null" locations are always "null". Non-"null" locations
    /// operate operate as follows (where Counting is the start state):
    ///
    /// ```text
    ///                                           ★
    ///                                           │
    ///                                           │
    ///                                           ▼
    ///                                         ┌──────────────────────────────────────────────────────────────────────┐   increment count
    ///                                         │                                                                      │ ──────────────────┐
    ///                                         │                               Counting                               │                   │
    ///                                         │                                                                      │ ◀─────────────────┘
    ///                                         └──────────────────────────────────────────────────────────────────────┘
    ///                                           │                 ▲                          ▲
    ///                                           │ start tracing   │ failed below threshold   │ failed below threshold
    ///                                           ▼                 │                          │
    /// ┌───────────┐  failed above threshold   ┌────────────────┐  │                          │
    /// │ DontTrace │ ◀──────────────────────── │    Tracing     │ ─┘                          │
    /// └───────────┘                           └────────────────┘                             │
    ///   ▲                                       │                                            │
    ///   │                                       │                                            │
    ///   │                                       ▼                                            │
    ///   │           failed above threshold    ┌────────────────┐                             │
    ///   └──────────────────────────────────── │   Compiling    │ ────────────────────────────┘
    ///                                         └────────────────┘
    ///                                           │
    ///                                           │
    ///                                           ▼
    ///                                         ┌────────────────┐
    ///                                         │    Compiled    │
    ///                                         └────────────────┘
    /// ```
    ///
    /// This diagram was created with [this tool](https://dot-to-ascii.ggerganov.com/) using this
    /// GraphViz input:
    ///
    /// ```text
    /// digraph {
    ///   init [label="", shape=point];
    ///   init -> Counting;
    ///   Counting -> Counting [label="increment count"];
    ///   Counting -> Tracing [label="start tracing"];
    ///   Tracing -> Compiling;
    ///   Tracing -> Counting [label="failed below threshold"];
    ///   Tracing -> DontTrace [label="failed above threshold"];
    ///   Compiling -> Compiled;
    ///   Compiling -> Counting [label="failed below threshold"];
    ///   Compiling -> DontTrace [label="failed above threshold"];
    /// }
    /// ```
    ///
    /// We hope that a Location soon reaches the `Compiled` state (aka "the happy state") and stays
    /// there. However, many Locations will not be used frequently enough to reach such a state, so
    /// we don't want to waste resources on them.
    ///
    /// We therefore encode a Location as a tagged integer: when initialised, no memory is
    /// allocated; if the location is used frequently enough it becomes hot, memory is allocated
    /// for it, and a pointer stored instead of an integer. Note that once memory for a hot
    /// location is allocated, it can only be (scheduled for) deallocation when a Location is
    /// dropped, as the Location may have handed out `&` references to that allocated memory. That
    /// means that the `Counting` state is encoded in two separate ways: both with and without
    /// allocated memory.
    ///
    /// The layout of a Location is as follows:
    ///   bits 0-1 = state kind (STATE_NULL / STATE_NOT_HOT / STATE_HOT)
    ///   bit  2   = method-entry flag (only meaningful when STATE_NOT_HOT)
    ///   bits 3.. = payload: count (STATE_NOT_HOT) or pointer (STATE_HOT)
    /// In the `STATE_HOT` state, bits 0-2 are 0 in the pointer (8-byte alignment).
    inner: AtomicUsize,
}

impl Location {
    /// Create a new location.
    pub fn new() -> Self {
        // Locations start in the counting state with a count of 0.
        debug_assert_ne!(STATE_NOT_HOT, 0);
        Self {
            inner: AtomicUsize::new(STATE_NOT_HOT),
        }
    }

    /// Create a new method-entry location. Behaves like [Self::new] but marks the location as a
    /// method-entry anchor. When the tracer encounters an inner loop while tracing from a
    /// method-entry anchor it cools the anchor down fully (rather than re-queuing it immediately),
    /// allowing the inner loop to compile first and the method-entry to later become a coupler.
    pub fn new_method_entry() -> Self {
        debug_assert_ne!(STATE_NOT_HOT, 0);
        Self {
            inner: AtomicUsize::new(STATE_NOT_HOT | STATE_ENTRY_BIT),
        }
    }

    /// Create a new "null" location, denoting a point in a program which can never contribute to a
    /// trace.
    pub fn null() -> Self {
        Self {
            inner: AtomicUsize::new(STATE_NULL),
        }
    }

    /// Returns true if this is a "null" location.
    pub fn is_null(&self) -> bool {
        self.inner.load(Ordering::Relaxed) == STATE_NULL
    }

    /// If `self` is:
    ///   1. in the `Counting` state and
    ///   2. has not had a `HotLocation` allocated for it
    ///
    /// then increment and return its count, or `None` otherwise.
    pub(crate) fn inc_count(&self) -> Option<HotThreshold> {
        let x = self.inner.load(Ordering::Relaxed);
        if x & STATE_TAG_MASK == STATE_NOT_HOT {
            // `HotThreshold` must be unsigned
            debug_assert_eq!(HotThreshold::MIN, 0);
            // For the `as` to be safe, `HotThreshold` can't be bigger than `usize`
            debug_assert!(mem::size_of::<HotThreshold>() <= mem::size_of::<usize>());
            let entry_bit = x & STATE_ENTRY_BIT; // preserve across the CAS
            let old = (x >> STATE_NUM_BITS) as HotThreshold;
            // The particular value of `new` must fit in the bits we have available.
            let new = old + 1;
            debug_assert!(
                (new as usize)
                    .checked_shl(u32::try_from(STATE_NUM_BITS).unwrap())
                    .is_some()
            );

            self.inner
                .compare_exchange_weak(
                    ((old as usize) << STATE_NUM_BITS) | entry_bit | STATE_NOT_HOT,
                    ((new as usize) << STATE_NUM_BITS) | entry_bit | STATE_NOT_HOT,
                    Ordering::Relaxed,
                    Ordering::Relaxed,
                )
                .ok()
                .map(|_| new)
        } else {
            None
        }
    }

    /// If `self` is:
    ///   1. in the `Counting` state and
    ///   2. has not had a `HotLocation` allocated for it
    ///
    /// return its count, or `None` otherwise
    pub(crate) fn count(&self) -> Option<HotThreshold> {
        let x = self.inner.load(Ordering::Relaxed);
        if x & STATE_TAG_MASK == STATE_NOT_HOT {
            // `HotThreshold` must be unsigned
            debug_assert_eq!(HotThreshold::MIN, 0);
            Some((x >> STATE_NUM_BITS) as HotThreshold)
        } else {
            None
        }
    }

    /// Change `self` to be a [HotLocation] `hl` if: `self` is in the `Counting` state; and the
    /// count is `old`. If the transition is successful, return a clone of the [Arc] that now
    /// wraps the [HotLocation].
    pub(crate) fn count_to_hot_location(
        &self,
        old: HotThreshold,
        mut hl: HotLocation,
    ) -> Option<Arc<Mutex<HotLocation>>> {
        // Read the entry bit from the current atomic value so we can include it in the CAS
        // expected value and propagate is_method_entry into the HotLocation.
        let cur = self.inner.load(Ordering::Relaxed);
        let entry_bit = cur & STATE_ENTRY_BIT;
        hl.is_method_entry = entry_bit != 0;

        let hl = Arc::new(Mutex::new(hl));
        let cl: *const Mutex<HotLocation> = Arc::into_raw(Arc::clone(&hl));
        // Arc<Mutex<HotLocation>> is at least 8-byte aligned, so bits 0-2 are always 0.
        debug_assert_eq!((cl as usize) & !STATE_FULL_MASK, cl as usize);
        match self.inner.compare_exchange(
            ((old as usize) << STATE_NUM_BITS) | entry_bit | STATE_NOT_HOT,
            (cl as usize) | STATE_HOT,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => Some(hl),
            Err(_) => {
                unsafe {
                    Arc::from_raw(cl);
                }
                None
            }
        }
    }

    pub fn set_hl_debug_str(&self, s: String) {
        match self.hot_location() {
            Some(hl) => {
                hl.lock().debug_str = Some(s);
            }
            None => {
                let Some(count) = self.count() else {
                    // We clashed with another thread.
                    todo!();
                };
                let hl = HotLocation {
                    kind: HotLocationKind::Counting(count),
                    tracecompilation_errors: 0,
                    debug_str: Some(s),
                    is_method_entry: false, // set by count_to_hot_location from the entry bit
                };
                if self.count_to_hot_location(count, hl).is_none() {
                    // We clashed with another thread.
                    todo!();
                }
            }
        }
    }

    /// If `self` has a [HotLocation] return a reference to the `Mutex` that directly wraps it, or
    /// `None` otherwise.
    pub(crate) fn hot_location(&self) -> Option<&Mutex<HotLocation>> {
        let x = self.inner.load(Ordering::Relaxed);
        if x & STATE_TAG_MASK == STATE_HOT {
            // `Arc::into_raw::<Mutex<T>>` returns `*mut Mutex<T>` so the address we're wrapping is
            // a pointer to the `Mutex` itself. By returning a `&` reference we ensure that the
            // reference to the `Mutex` can't outlive this `Location`.
            Some(unsafe { &*((x & !STATE_FULL_MASK) as *const _) })
        } else {
            None
        }
    }

    /// If `self` has a [HotLocation] return a clone of the [Arc] that wraps it, or `None`
    /// otherwise.
    pub(crate) fn hot_location_arc_clone(&self) -> Option<Arc<Mutex<HotLocation>>> {
        let x = self.inner.load(Ordering::Relaxed);
        if x & STATE_TAG_MASK == STATE_HOT {
            let raw = unsafe { Arc::from_raw((x & !STATE_FULL_MASK) as *mut _) };
            let cl = Arc::clone(&raw);
            mem::forget(raw);
            Some(cl)
        } else {
            None
        }
    }
}

impl Default for Location {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Location {
    fn drop(&mut self) {
        let x = self.inner.load(Ordering::Relaxed);
        if x & STATE_TAG_MASK == STATE_HOT {
            drop(unsafe { Arc::from_raw((x & !STATE_FULL_MASK) as *mut Mutex<HotLocation>) });
        }
    }
}

#[derive(Debug)]
pub(crate) struct HotLocation {
    pub(crate) kind: HotLocationKind,
    /// How often has tracing or compilation starting directly from this hot location led to an
    /// error?
    pub(crate) tracecompilation_errors: TraceCompilationErrorThreshold,
    /// An optional debug string for this hot location.
    pub(crate) debug_str: Option<String>,
    /// True if this location was created via [Location::new_method_entry]. When the tracer detects
    /// an "unrolled inner loop" while this location is the trace start, it cools the location down
    /// fully (count back to 0) instead of re-queueing it immediately, giving the inner loop time
    /// to compile and allowing this location to later form a coupler trace into it.
    pub(crate) is_method_entry: bool,
}

impl HotLocation {
    /// A trace, or the compilation of a trace, starting at this [HotLocation] led to an error. The
    /// return value indicates whether further traces for this location should be generated or not.
    pub(crate) fn tracecompilation_error(&mut self, mt: &Arc<MT>) -> TraceFailed {
        if self.tracecompilation_errors < mt.trace_failure_threshold() {
            self.tracecompilation_errors += 1;
            TraceFailed::KeepTrying
        } else {
            TraceFailed::DontTrace
        }
    }
}

/// A `Location`'s non-counting states.
pub(crate) enum HotLocationKind {
    /// Points to executable machine code that can be executed instead of the interpreter for this
    /// HotLocation.
    Compiled(Arc<dyn CompiledTrace>),
    /// A trace for this HotLocation is being compiled in another trace. When compilation is
    /// complete, the compiling thread will update the state of this HotLocation.
    Compiling(TraceId),
    /// Because of a failure in compiling / tracing, we have reentered the `Counting` state. This
    /// can be seen as a way of implementing back-off in the face of errors.
    Counting(HotThreshold),
    /// This HotLocation has encountered problems (e.g. traces which are too long) and shouldn't be
    /// traced again.
    DontTrace,
    /// This HotLocation started a trace which is ongoing.
    Tracing(TraceId),
}

impl std::fmt::Debug for HotLocationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Compiled(_) => write!(f, "Compiled"),
            Self::Compiling(_) => write!(f, "Compiling"),
            Self::Counting(_) => write!(f, "Counting"),
            Self::DontTrace => write!(f, "DontTrace"),
            Self::Tracing(_) => write!(f, "Tracing"),
        }
    }
}

/// Track [HotLocation]s seen while tracing to determine if unrolling has occurred.
pub(super) struct SeenHotLocations {
    seen: Vec<Arc<Mutex<HotLocation>>>,
}

impl SeenHotLocations {
    /// Create a new [SeenHotLocations] starting from `initial`.
    pub(super) fn new(initial: Arc<Mutex<HotLocation>>) -> Self {
        Self {
            seen: vec![initial],
        }
    }

    /// Record that `hl` has been encountered during tracing. Return `true` if a loop has been
    /// formed in the trace. Note: "a loop has been formed" includes both "the entire trace is a
    /// loop" and "an inner loop has been unrolled".
    pub(super) fn push_and_check_any_loop_closed(&mut self, hl: Arc<Mutex<HotLocation>>) -> bool {
        let seen = self.seen.iter().any(|x| Arc::ptr_eq(x, &hl));
        self.seen.push(hl);
        seen
    }

    /// Return true if [Self] forms a "pure" loop: that is, that the first and last [HotLocation]s
    /// are equivalent.
    ///
    /// # Panics
    ///
    /// If there are not at least two [HotLocation]s in `Self`.
    pub(super) fn is_loop(&self) -> bool {
        assert!(self.seen.len() > 1);
        Arc::ptr_eq(self.seen.first().unwrap(), self.seen.last().unwrap())
    }
}

/// When a [HotLocation] has failed to compile a valid trace, should the [HotLocation] be tried
/// again or not?
#[derive(Debug)]
pub(crate) enum TraceFailed {
    KeepTrying,
    DontTrace,
}
