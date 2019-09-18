use std::ops;

#[inline]
pub(in crate) fn bl_range<R: ops::RangeBounds<usize>>(range: R) -> ffi::BLRange {
    ffi::BLRange {
        start: match range.start_bound() {
            ops::Bound::Included(n) => *n,
            ops::Bound::Excluded(_) => {
                unreachable!("start_bound of a range cannot be an excluded bound")
            },
            ops::Bound::Unbounded => 0,
        },
        end: match range.end_bound() {
            ops::Bound::Included(n) => *n,
            ops::Bound::Excluded(n) => *n - 1,
            ops::Bound::Unbounded => 0,
        },
    }
}

#[inline]
pub(in crate) fn range_to_tuple<F, R>(range: R, len_func: F) -> (usize, usize)
where
    F: FnOnce() -> usize,
    R: ops::RangeBounds<usize>,
{
    (
        match range.start_bound() {
            ops::Bound::Included(n) => *n,
            ops::Bound::Excluded(_) => {
                unreachable!("start_bound of a range cannot be an excluded bound")
            },
            ops::Bound::Unbounded => 0,
        },
        match range.end_bound() {
            ops::Bound::Included(n) => *n,
            ops::Bound::Excluded(n) => *n - 1,
            ops::Bound::Unbounded => len_func(),
        },
    )
}
