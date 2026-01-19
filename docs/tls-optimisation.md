# TLS Optimisation: Global Tracing Counter

## The Problem

Every promotion function (e.g., `__yk_idempotent_promote_i32`) needs to check if the current thread is tracing. Previously, this check used TLS (Thread-Local Storage):

```rust
// Before: Every call does a TLS lookup
if MTThread::is_tracing() {  // ← Calls __tls_get_addr (~30 cycles)
    // record promotion
}
```

In shared libraries on Linux, accessing TLS variables requires calling `__tls_get_addr`, which is expensive (~30 cycles vs ~4 cycles for a simple memory read). This was causing **~12% overhead** in the LuLPeg benchmark.

### Perf Profile (Before Optimisation)

| Function | Self % | Notes |
|----------|--------|-------|
| `luaV_execute` | 41.83% | Main Lua interpreter loop |
| `__tls_get_addr` | 9.11% | TLS lookups |
| `__yk_idempotent_promote_i32` | 8.35% | Promotion function |
| `__yk_idempotent_promote_i64` | 6.00% | Promotion function |
| `__yk_promote_ptr` | 4.89% | Promotion function |
| `__tls_get_addr$plt` | 2.87% | PLT overhead for TLS |

**Total TLS overhead: ~12%**

## The Solution

We added a **global atomic counter** (not TLS) that tracks how many threads are currently tracing:

```
┌─────────────────────────────────────────────────────────────────┐
│                    TRACING_THREAD_COUNT                         │
│                    (global atomic u32)                          │
│                                                                 │
│   Value: 0 = no thread tracing (fast path)                      │
│   Value: N = N threads currently tracing                        │
└─────────────────────────────────────────────────────────────────┘
```

The promotion functions now do a **two-level check**:

```rust
// After: Fast path avoids TLS entirely
if MTThread::any_thread_tracing()  // ← Simple atomic load (~4 cycles)
    && MTThread::is_tracing()      // ← TLS lookup (only if first check passes)
{
    // record promotion
}
```

## How It Works

### When a Thread Starts Tracing

```
set_tracing(IsTracing::Loop)
  └─→ TRACING_THREAD_COUNT += 1
```

### When a Thread Stops Tracing

```
set_tracing(IsTracing::None)
  └─→ TRACING_THREAD_COUNT -= 1
```

### On Every Promotion Call

```
any_thread_tracing()?
  ├─ NO (count == 0)  → Return immediately (no TLS lookup!)
  │
  └─ YES (count > 0)  → Check is_tracing() (TLS lookup)
                         ├─ This thread tracing → Record promotion
                         └─ Not this thread    → Return
```

## Performance Impact

| Scenario | Before | After |
|----------|--------|-------|
| **Converged** (no tracing) | `__tls_get_addr` call | 1 atomic load |
| **During tracing** | `__tls_get_addr` call | 1 atomic load + `__tls_get_addr` |

**Expected improvement**: In the converged state (when traces are compiled and executing), promotion functions should be **~7× faster** because they avoid all TLS lookups entirely.

The key insight is that after convergence, `TRACING_THREAD_COUNT == 0`, so every promotion call returns immediately after a single cheap memory read.

## Implementation Details

### Files Changed

1. **`ykrt/src/mt.rs`**:
   - Added `TRACING_THREAD_COUNT: AtomicU32` global counter
   - Added `MTThread::any_thread_tracing()` fast check function
   - Modified `MTThread::set_tracing()` to update the counter

2. **`ykrt/src/promote.rs`**:
   - All promotion functions now use the two-level check pattern

### Code References

Global counter declaration:

```rust
/// Global counter of threads currently tracing. This is NOT TLS - it's a simple atomic counter.
/// When this is 0, no thread is tracing, and promotion functions can skip TLS lookups entirely.
/// This optimisation avoids `__tls_get_addr` overhead on the fast path (not tracing).
static TRACING_THREAD_COUNT: AtomicU32 = AtomicU32::new(0);
```

Fast check function:

```rust
/// Fast check: is ANY thread currently tracing?
///
/// This is a simple atomic load (no TLS lookup), making it suitable for the hot path.
/// Returns `false` when no thread is tracing, allowing callers to skip TLS lookups entirely.
#[inline(always)]
pub fn any_thread_tracing() -> bool {
    TRACING_THREAD_COUNT.load(Ordering::Relaxed) > 0
}
```

## Why Not Use `#[thread_local]` Directly?

We initially attempted to replace `thread_local!` with `#[thread_local]` (native TLS) for the `THREAD_MTTHREAD` variable. This approach failed because:

1. **Destructor semantics**: Tests rely on `Arc::strong_count()` to detect when threads have exited and released their TLS-held `Arc`s. The `thread_local!` macro provides deterministic destructor behaviour that the tests depend on.

2. **`-Ztls-model=initial-exec` flag**: This Rust compiler flag would force Initial Exec TLS model (avoiding `__tls_get_addr`), but it broke the build by affecting all dependencies including `dynasmrt`.

The global counter approach achieves the same performance benefit without these compatibility issues.

## Future Optimisations

If further TLS optimisation is needed:

1. **Platform-specific inline assembly**: Directly access the FS segment register to bypass TLS machinery entirely.

2. **Cached TLS pointer**: When tracing starts, cache the `MTThread*` in a native TLS variable for use during the tracing phase.

3. **Compile with `-ftls-model=initial-exec`**: Apply this to the C code in `ykcapi` if it uses TLS.

