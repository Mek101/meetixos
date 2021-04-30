/*! # HAL Frames
 *
 * Implements a wrapper for the [`PhysAddr`] and the [`VirtAddr`] that
 * always ensure [`PAGE_SIZE`] alignment and consequently give a size to the
 * wrapped virtual/physical addresses
 *
 * [`PhysAddr`]: /hal/addr/struct.PhysAddr.html
 * [`VirtAddr`]: /hal/addr/struct.VirtAddr.html
 * [`PAGE_SIZE`]: /hal/paging/constant.PAGE_SIZE.html
 */

use core::{
    fmt,
    iter::Step,
    marker::PhantomData,
    ops::{
        Add,
        AddAssign,
        Range,
        RangeInclusive,
        Sub,
        SubAssign
    }
};

use crate::{
    addr::{
        Address,
        PhysAddr,
        VirtAddr
    },
    mem::paging::{
        Page1GiB,
        Page2MiB,
        Page4KiB,
        PageSize,
        PageTableIndex,
        PageTableLevel
    }
};

/** # Virtual Memory Frame
 *
 * [`Frame`]<[`VirtAddr`]> alias.
 *
 * Applies the [`Frame`] logic to virtual addresses, it's expected to be
 * used as allocation unit for virtual page allocators
 *
 * [`Frame`]: /hal/paging/struct.Frame.html
 * [`VirtAddr`]: /hal/addr/struct.VirtAddr.html
 */
pub type VirtFrame<S> = Frame<VirtAddr, S>;
pub type VirtFrameRange<S> = Range<VirtFrame<S>>;
pub type VirtFrameRangeIncl<S> = RangeInclusive<VirtFrame<S>>;

/** # Physical Memory Frame
 *
 * [`Frame`]<[`PhysAddr`]> alias.
 *
 * Applies the [`Frame`] logic to physical addresses, it's expected to be
 * used as allocation unit for physical page allocators
 *
 * [`Frame`]: /hal/paging/struct.Frame.html
 * [`PhysAddr`]: /hal/addr/struct.PhysAddr.html
 */
pub type PhysFrame<S> = Frame<PhysAddr, S>;
pub type PhysFrameRange<S> = Range<PhysFrame<S>>;
pub type PhysFrameRangeIncl<S> = RangeInclusive<PhysFrame<S>>;

/** # Memory Frame
 *
 * Represents a generic frame of memory, that is simply an [`Address`] based
 * pointer which is surely aligned with [`PAGE_SIZE`] and have the same
 * size.
 *
 * It is used as unit for physical and virtual allocation to have a type
 * assurance of the alignment and the size and not on the allocator
 * implementation
 *
 * [`Address`]: /hal/addr/trait.Address.html
 * [`PAGE_SIZE`]: /hal/paging/constant.PAGE_SIZE.html
 */
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Frame<T, S>
    where T: Address,
          S: PageSize {
    m_addr_impl: T,
    _unused: PhantomData<S>
}

impl<T, S> Frame<T, S>
    where T: Address,
          S: PageSize
{
    /** # Constructs an unchecked `Frame`
     *
     * This function is used only internally by the module
     */
    fn new_unchecked(addr: T) -> Self {
        Self { m_addr_impl: addr,
               _unused: PhantomData::default() }
    }

    /** # Constructs a `Frame`
     *
     * The returned instance will be valid only if aligned with [`S::SIZE`]
     *
     * [`S::SIZE`]: /hal/paging/trait.PageSize.html#associatedconstant.SIZE
     */
    pub fn new(addr: T) -> Result<Self, FrameNotAlignedErr> {
        if addr.is_aligned(S::SIZE) {
            Ok(Self::new_unchecked(addr))
        } else {
            Err(FrameNotAlignedErr)
        }
    }

    /** # Constructs `addr`'s containing `Frame`
     *
     * The returned instance is down-aligned to the nearest [`S::SIZE`]
     *
     * [`S::SIZE`]: /hal/paging/trait.PageSize.html#associatedconstant.SIZE
     */
    pub fn of_addr(addr: T) -> Self {
        Self::new_unchecked(addr.align_down(S::SIZE).unwrap())
    }

    /** Returns the starting [`Address`] of this `Frame`
     *
     * [`Address`]: /hal/addr/trait.Address.html
     */
    pub fn start_addr(&self) -> T {
        self.m_addr_impl
    }

    /** Returns the size of this `Frame`
     */
    pub fn size(&self) -> usize {
        S::SIZE
    }

    /** # Converts the `Frame` to a generic `PageSize`
     *
     * Useful when must return a concrete `Frame` from a generic sized
     * method
     */
    pub fn into_generic_sized_frame<ST>(self) -> Frame<T, ST>
        where ST: PageSize {
        Frame { m_addr_impl: self.m_addr_impl,
                _unused: Default::default() }
    }

    /** # Converts the `Frame` into a `Range` of `Frame`s
     *
     * The custom [`PageSize`] must be a multiple of the current page size
     *
     * [`PageSize`]: /hal/paging/trait.PageSize.html
     */
    pub fn into_range_of<ST>(self) -> Range<Frame<T, ST>>
        where ST: PageSize {
        assert_eq!(S::SIZE % ST::SIZE, 0);
        let new_sized_frame = self.into_generic_sized_frame();
        Range { start: new_sized_frame,
                end: new_sized_frame + S::SIZE / ST::SIZE }
    }

    /** # Constructs a `Range` of `Frame`s
     *
     * Returns an end exclusive [`Range`] that starts from the given
     * `start_frame` and steps until the previous `end_frame`
     *
     * [`Range`]: https://doc.rust-lang.org/std/ops/struct.Range.html
     */
    pub fn range_of(start_frame: Frame<T, S>,
                    end_frame: Frame<T, S>)
                    -> Range<Frame<T, S>> {
        Range { start: start_frame,
                end: end_frame }
    }

    /** # Constructs a `Range` of `Frame`s
     *
     * Returns an end exclusive [`Range`] that starts from the given
     * `start_frame` and steps for `count` frames
     *
     * [`Range`]: https://doc.rust-lang.org/std/ops/struct.Range.html
     */
    pub fn range_of_count(start_frame: Frame<T, S>, count: usize) -> Range<Frame<T, S>> {
        Self::range_of(start_frame, start_frame + count)
    }

    /** # Constructs a `RangeInclusive` of `Frame`
     *
     * Returns an end exclusive [`RangeInclusive`] that starts from the
     * given `start_frame` and steps until the given `end_frame`
     *
     * [`RangeInclusive`]: https://doc.rust-lang.org/std/ops/struct.RangeInclusive.html
     */
    pub fn range_incl_of(start_frame: Frame<T, S>,
                         end_frame: Frame<T, S>)
                         -> RangeInclusive<Frame<T, S>> {
        RangeInclusive::new(start_frame, end_frame)
    }

    /** # Constructs a `RangeInclusive` of `Frame`
     *
     * Returns an end exclusive [`RangeInclusive`] that starts from the
     * given `start_frame` and steps for `count` frames
     *
     * [`RangeInclusive`]: https://doc.rust-lang.org/std/ops/struct.RangeInclusive.html
     */
    pub fn range_incl_of_count(start_frame: Frame<T, S>,
                               count: usize)
                               -> RangeInclusive<Frame<T, S>> {
        Self::range_incl_of(start_frame, start_frame + count)
    }
}

impl<S> Frame<VirtAddr, S> where S: PageSize {
    /** Returns the fourth level [`PageTableIndex`]
     *
     * [`PageTableIndex`]: /hal/paging/struct.PageTableIndex.html
     */
    pub fn level_4_index(&self) -> PageTableIndex {
        self.m_addr_impl.level_4_index()
    }

    /** Returns the third level [`PageTableIndex`]
     *
     * [`PageTableIndex`]: /hal/paging/struct.PageTableIndex.html
     */
    pub fn level_3_index(&self) -> PageTableIndex {
        self.m_addr_impl.level_3_index()
    }

    /** Returns the second level [`PageTableIndex`]
     *
     * [`PageTableIndex`]: /hal/paging/struct.PageTableIndex.html
     */
    pub fn level_2_index(&self) -> PageTableIndex {
        self.m_addr_impl.level_2_index()
    }

    /** Returns the first level [`PageTableIndex`]
     *
     * [`PageTableIndex`]: /hal/paging/struct.PageTableIndex.html
     */
    pub fn level_1_index(&self) -> PageTableIndex {
        self.m_addr_impl.level_1_index()
    }

    /** Returns the [`PageTableLevel`]th [`PageTableIndex`]
     *
     * [`PageTableLevel`]: /hal/paging/enum.PageTableLevel.html
     * [`PageTableIndex`]: /hal/paging/struct.PageTableIndex.html
     */
    pub fn index_for_level(&self, pt_level: PageTableLevel) -> PageTableIndex {
        match pt_level {
            PageTableLevel::Level4 => self.level_4_index(),
            PageTableLevel::Level3 => self.level_3_index(),
            PageTableLevel::Level2 => self.level_2_index(),
            PageTableLevel::Level1 => self.level_1_index()
        }
    }
}

impl Frame<VirtAddr, Page4KiB> {
    /** # Constructs the `VirtFrame` from the indices
     *
     * The returned instance is the down-aligned virtual frame of the
     * composed one
     */
    pub fn from_table_indexes(l4_index: PageTableIndex,
                              l3_index: PageTableIndex,
                              l2_index: PageTableIndex,
                              l1_index: PageTableIndex)
                              -> Self {
        Self::of_addr(VirtAddr::from_4kib_indices(l4_index, l3_index, l2_index, l1_index))
    }
}

impl Frame<VirtAddr, Page2MiB> {
    /** # Constructs the `VirtFrame` from the indices
     *
     * The returned instance is the down-aligned virtual frame of the
     * composed one
     */
    pub fn from_table_indexes(l4_index: PageTableIndex,
                              l3_index: PageTableIndex,
                              l2_index: PageTableIndex)
                              -> Self {
        Self::of_addr(VirtAddr::from_2mib_indices(l4_index, l3_index, l2_index))
    }
}

impl Frame<VirtAddr, Page1GiB> {
    /** # Constructs the `VirtFrame` from the indices
     *
     * The returned instance is the down-aligned virtual frame of the
     * composed one
     */
    pub fn from_table_indexes(l4_index: PageTableIndex,
                              l3_index: PageTableIndex)
                              -> Self {
        Self::of_addr(VirtAddr::from_1gib_indices(l4_index, l3_index))
    }
}

impl<T, S> fmt::Debug for Frame<T, S>
    where T: Address,
          S: PageSize
{
    /** Formats the value using the given formatter
     */
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self.m_addr_impl)
    }
}

unsafe impl<T, S> Step for Frame<T, S>
    where T: Address,
          S: PageSize
{
    /** Returns the number of *successor* steps required to get from `start`
     * to `end`.
     */
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        Some((end.start_addr().as_usize() - start.start_addr().as_usize()) / S::SIZE)
    }

    /** Returns the value that would be obtained by taking the *successor* of
     * `self` `count` times.
     */
    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        Some(start + count)
    }

    /** Returns the value that would be obtained by taking the *predecessor*
     * of `self` `count` times.
     */
    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        Some(start - count)
    }
}

impl<T, S> Add<usize> for Frame<T, S>
    where T: Address,
          S: PageSize
{
    /** The resulting type after applying the `+` operator.
     */
    type Output = Self;

    /** Performs the `+` operation.
     */
    fn add(self, rhs: usize) -> Self::Output {
        Self::of_addr(self.start_addr() + rhs * S::SIZE)
    }
}

impl<T, S> AddAssign<usize> for Frame<T, S>
    where T: Address,
          S: PageSize
{
    /** Performs the `+=` operation.
     */
    fn add_assign(&mut self, rhs: usize) {
        *self = self.add(rhs);
    }
}

impl<T, S> Sub<usize> for Frame<T, S>
    where T: Address,
          S: PageSize
{
    /** The resulting type after applying the `-` operator.
     */
    type Output = Self;

    /** Performs the `-` operation.
     */
    fn sub(self, rhs: usize) -> Self::Output {
        Self::of_addr(self.start_addr() - rhs * S::SIZE)
    }
}

impl<T, S> SubAssign<usize> for Frame<T, S>
    where T: Address,
          S: PageSize
{
    /** Performs the `-=` operation.
     */
    fn sub_assign(&mut self, rhs: usize) {
        *self = self.sub(rhs);
    }
}

/** # Frame Alignment Error
 *
 * Represents a [`Frame`] creation error
 *
 * [`Frame`]: /hal/paging/struct.Frame.html
 */
#[derive(Debug)]
pub struct FrameNotAlignedErr;

impl fmt::Display for FrameNotAlignedErr {
    /** Formats the value using the given formatter.
     */
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "The given frame address was not properly aligned")
    }
}
