//! The high-level interface to an optimiser.

use crate::compile::{CompilationError, j2::hir::*};
use index_vec::IndexVec;

pub(super) mod noopt;
#[allow(clippy::module_inception)]
pub(super) mod opt;

/// An optimiser. By definition this operates on one [Block] at a time, so it is both [ModLikeT]
/// and [BlockLikeT].
pub(super) trait OptT: ModLikeT + BlockLikeT {
    /// The block is now complete and the optimiser should turn it into a [Block] and a set of
    /// types (suitable for putting in a [Mod]).
    fn build(self: Box<Self>) -> (Block, IndexVec<TyIdx, Ty>);

    #[allow(dead_code)]
    fn peel(self) -> (Block, Block);

    /// What does this optimiser currently map `iidx` to? By definition this will return `iidx` or
    /// a smaller [InstIdx].
    ///
    /// Note that the value returned may vary as the optimiser receives more instructions. For
    /// example consider an input trace along the lines of:
    ///
    /// ```text
    /// %0: i32 = arg
    /// %1: i32 = arg
    /// %2: i32 = add %0, %1
    /// %3: i32 = 0
    /// %4: i1 = eq %0, %3
    /// guard true, %4, [...]
    /// %6: i32 = sub %0, %1
    /// ```
    ///
    /// From instructions 1..=5 `map_iidx(InstIdx::from(0))` will return `0`; from that point
    /// onwards it may (depending on the optimiser!) return `3` because the `guard` proves that it
    /// is equivalent.
    ///
    /// # Panics
    ///
    /// If `iidx` is greater than the number of instructions the optimiser currently holds.
    fn map_iidx(&self, iidx: InstIdx) -> InstIdx;

    /// Push an instruction and return an [InstIdx]. The returned [InstIdx] may refer to a
    /// previously inserted instruction, as an optimiser might prove that `inst` is unneeded.
    /// That previously inserted instruction may not even be of the same kind as `inst`!
    fn push_inst(&mut self, inst: Inst) -> Result<InstIdx, CompilationError>;

    /// Push a type [ty]. This type may be cached, and thus the [TyIdx] returned may not
    /// monotonically increase.
    fn push_ty(&mut self, ty: Ty) -> Result<TyIdx, CompilationError>;
}
