//! An LLVM stackmap parser.
//!
//! Parses stackmaps generated by LLVM during compile time. For more information about the stackmap
//! format see https://llvm.org/docs/StackMaps.html#stackmap-format.
//!
//! Note that LLVM currently only supports stackmaps for 64 bit architectures. Once they support
//! others we will need to either make this parser more dynamic or create a new one for each
//! architecture.
#[cfg(not(target_arch = "x86_64"))]
compile_error!("The stackmap parser currently only supports x64.");

use std::error;

struct Function {
    /// The address in the binary where this function is located.
    addr: u64,
    /// Number of stackmap records in this function.
    record_count: u64,
    /// The size of this functions stack. Does not include the return address of this or children
    /// stack frames.
    stack_size: u64,
}

/// Information recorded by a stackmap instruction.
pub struct Record {
    /// The unique identifier of this record.
    pub id: u64,
    /// The absolute offset in bytes of this record in the binary.
    pub offset: u64,
    /// The list of live variables recorded at this point.
    pub live_vars: Vec<LiveVar>,
    /// The stack size of the function this record is contained in.
    pub size: u64,
}

impl Record {
    pub fn empty() -> Record {
        Record {
            id: 0,
            offset: 0,
            live_vars: Vec::new(),
            size: 0,
        }
    }
}

/// Describes where live variables are stored at specific times during execution.
#[derive(Clone, Debug)]
pub enum Location {
    /// The live variable is stored in a register. Note, that LLVM's stackmap only stores one
    /// location per live variable, which is enough for reading them out. For deoptimisation
    /// however, we need to restore live variables, and the compiler often puts them in multiple
    /// places, e.g. during spilling. Thus the fields describe three different locations in total:
    /// * `u16`: Dwarf register number
    /// * `u16`: size of the value
    /// * `i32`: additional location on the stack (offset in relation to the base pointer)
    /// * `u16`: additional location in a register (dwarf register number)
    ///
    /// FIXME: We may need more additional locations in the future, which however will require
    /// rewriting the stackmap format (until now we managed to get by with two extra locations).
    Register(u16, u16, i32, Vec<i16>),
    /// The live variable is a pointer into the stack. To avoid unnecessary spilling and
    /// dereferencing LLVM just records the value as an (offset, register) pair where the register
    /// is typically the base pointer:
    /// * `u16`: Dwarf register number
    /// * `i32`: offset
    /// * `u16`: size of the value
    Direct(u16, i32, u16),
    /// The live variable lives on the stack. Similar to the `Direct` location, it's recorded as an
    /// (offset, register) pair where the register is typically the base pointer. However, to read
    /// out the value the address described by the (offset, register) pair needs to be dereferenced
    /// first.
    Indirect(u16, i32, u16),
    /// The live variable is a constant and has been directly inlined into the stackmap.
    Constant(u32),
    /// The live variable is a large constant and was stored in a vector as part of a record. This
    /// variant describes the index where the constant is stored.
    LargeConstant(u64),
}

/// A live variable.
#[derive(Debug)]
pub struct LiveVar {
    /// The location where this variable is stored (or needs to be written to during
    /// deoptimsation). Typically, this vector only has a single entry, though it is possible for
    /// variables to be stored across multiple locations (e.g. 128bit values).
    locs: Vec<Location>,
}

impl LiveVar {
    pub fn len(&self) -> usize {
        self.locs.len()
    }

    pub fn is_empty(&self) -> bool {
        self.locs.is_empty()
    }

    pub fn get(&self, idx: usize) -> Option<&Location> {
        self.locs.get(idx)
    }
}

/// Information about a functions's prologue.
pub struct PrologueInfo {
    /// Describes whether or not the function uses a base pointer.
    pub hasfp: bool,
    /// A list of callee-saved registers, saved as (register, index) pairs, where:
    /// * register: the original register where a value was stored
    /// * index: the spill index (can be turned into an offset by multiplying with the register
    ///   size)
    pub csrs: Vec<(u16, i32)>,
}

/// Collect the prologue info and records of a function into a single struct.
pub struct SMEntry {
    pub pinfo: PrologueInfo,
    pub records: Vec<Record>,
}

/// Parses LLVM stackmaps version 3 from a given address. Provides a way to query relevant
/// locations given the return address of a `__llvm_deoptimize` function.
pub struct StackMapParser<'a> {
    data: &'a [u8],
    offset: usize,
}

impl StackMapParser<'_> {
    /// Parse LLVM stackmaps and return them as a list of `SMEntry`s, each of which describes a
    /// single function, i.e. it's prologue and all records contained in it.
    pub fn parse(data: &[u8]) -> (Vec<PrologueInfo>, Vec<(Record, usize)>) {
        let mut smp = StackMapParser { data, offset: 0 };
        smp.read().unwrap()
    }

    /// Parse LLVM's stackmap format.
    #[allow(clippy::type_complexity)]
    fn read(&mut self) -> Result<(Vec<PrologueInfo>, Vec<(Record, usize)>), Box<dyn error::Error>> {
        // Read version number.
        if self.read_u8() != 3 {
            return Err("Only stackmap format version 3 is supported.".into());
        }

        // Reserved
        assert_eq!(self.read_u8(), 0);
        assert_eq!(self.read_u16(), 0);

        let num_funcs = self.read_u32();
        let num_consts = self.read_u32();
        let num_recs = self.read_u32();

        let funcs = self.read_functions(num_funcs);
        let consts = self.read_consts(num_consts);

        // Check that the records match the sum of the expected records per function.
        assert_eq!(
            funcs.iter().map(|f| f.record_count).sum::<u64>(),
            u64::from(num_recs)
        );

        // Since records ids are used for indexing we need to prepare the vector first so we can
        // insert records at their right index.
        let totalsize: u64 = funcs.iter().map(|f| f.record_count).sum();
        let mut all_records = Vec::new();
        all_records.resize_with(usize::try_from(totalsize).unwrap(), || (Record::empty(), 0));

        // Parse and collect records.
        for (i, f) in funcs.iter().enumerate() {
            let mut records = self.read_records(f.record_count, &consts);
            for mut r in records.drain(..) {
                // Calculate the absolute offset for this record in the binary.
                r.offset += f.addr;
                r.size = f.stack_size;
                let idx = usize::try_from(r.id).unwrap();
                all_records[idx] = (r, i);
            }
        }

        // Read prologue info.
        let prologue_info = self.read_prologue(num_funcs);

        Ok((prologue_info, all_records))
    }

    fn read_functions(&mut self, num: u32) -> Vec<Function> {
        let mut v = Vec::new();
        for _ in 0..num {
            let addr = self.read_u64();
            let stack_size = self.read_u64();
            let record_count = self.read_u64();
            v.push(Function {
                addr,
                record_count,
                stack_size,
            });
        }
        v
    }

    fn read_consts(&mut self, num: u32) -> Vec<u64> {
        let mut v = Vec::new();
        for _ in 0..num {
            v.push(self.read_u64());
        }
        v
    }

    fn read_records(&mut self, num: u64, consts: &[u64]) -> Vec<Record> {
        let mut v = Vec::new();
        for _ in 0..num {
            let id = self.read_u64();
            let offset = u64::from(self.read_u32());
            self.read_u16();
            let num_live_vars = self.read_u16();
            let live_vars = self.read_live_vars(num_live_vars, consts);
            // Padding
            self.align_8();
            self.read_u16();
            let num_liveouts = self.read_u16();
            self.read_liveouts(num_liveouts);
            self.align_8();
            v.push(Record {
                id,
                offset,
                live_vars,
                size: 0,
            });
        }
        v
    }

    fn read_live_vars(&mut self, num: u16, consts: &[u64]) -> Vec<LiveVar> {
        let mut v = Vec::new();
        for _ in 0..num {
            let num_locs = self.read_u8();
            v.push(LiveVar {
                locs: self.read_locations(num_locs, consts),
            });
        }
        v
    }

    fn read_locations(&mut self, num: u8, consts: &[u64]) -> Vec<Location> {
        let mut v = Vec::new();
        for _ in 0..num {
            let kind = self.read_u8();
            self.read_u8();
            let size = self.read_u16();
            let dwreg = self.read_u16();
            self.read_u16();
            let mut extras = Vec::new();
            for _ in 0..self.read_u16() {
                extras.push(self.read_i16());
            }

            let location = match kind {
                0x01 => {
                    let offset = self.read_i32();
                    Location::Register(dwreg, size, offset, extras)
                }
                0x02 => {
                    let offset = self.read_i32();
                    Location::Direct(dwreg, offset, size)
                }
                0x03 => {
                    let offset = self.read_i32();
                    Location::Indirect(dwreg, offset, size)
                }
                0x04 => {
                    let offset = self.read_u32();
                    Location::Constant(offset)
                }
                0x05 => {
                    let offset = self.read_i32();
                    Location::LargeConstant(consts[usize::try_from(offset).unwrap()])
                }
                _ => unreachable!(),
            };

            v.push(location)
        }
        v
    }

    fn read_liveouts(&mut self, num: u16) {
        for _ in 0..num {
            let _dwreg = self.read_u16();
            let _size = self.read_u8();
        }
    }

    fn read_prologue(&mut self, num_funcs: u32) -> Vec<PrologueInfo> {
        let mut pis = Vec::new();
        for _ in 0..num_funcs {
            let hasfptr = self.read_u8();
            assert!(hasfptr == 0 || hasfptr == 1);
            self.read_u8(); // Padding
            let numspills = self.read_u32();

            let mut v = Vec::new();
            for _ in 0..numspills {
                let reg = self.read_u16();
                self.read_u16(); // Padding
                let off = self.read_i32();
                v.push((reg, off));
            }
            let pi = PrologueInfo {
                hasfp: hasfptr != 0,
                csrs: v,
            };
            pis.push(pi);
        }
        pis
    }

    fn align_8(&mut self) {
        self.offset += (8 - (self.offset % 8)) % 8;
    }

    fn read_u8(&mut self) -> u8 {
        let d = u8::from_ne_bytes(self.data[self.offset..self.offset + 1].try_into().unwrap());
        self.offset += 1;
        d
    }

    fn read_u16(&mut self) -> u16 {
        let d = u16::from_ne_bytes(self.data[self.offset..self.offset + 2].try_into().unwrap());
        self.offset += 2;
        d
    }

    fn read_i16(&mut self) -> i16 {
        let d = i16::from_ne_bytes(self.data[self.offset..self.offset + 2].try_into().unwrap());
        self.offset += 2;
        d
    }

    fn read_u32(&mut self) -> u32 {
        let d = u32::from_ne_bytes(self.data[self.offset..self.offset + 4].try_into().unwrap());
        self.offset += 4;
        d
    }

    fn read_i32(&mut self) -> i32 {
        let d = i32::from_ne_bytes(self.data[self.offset..self.offset + 4].try_into().unwrap());
        self.offset += 4;
        d
    }

    fn read_u64(&mut self) -> u64 {
        let d = u64::from_ne_bytes(self.data[self.offset..self.offset + 8].try_into().unwrap());
        self.offset += 8;
        d
    }
}
