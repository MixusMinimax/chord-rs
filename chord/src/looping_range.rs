/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use std::ops::{Bound, RangeBounds};

pub trait LoopingRange<T: ?Sized> {
    fn contains_looping<U>(&self, value: &U) -> bool
    where
        T: PartialOrd<T>,
        T: PartialOrd<U>,
        U: ?Sized + PartialOrd<T>;
}

impl<T: ?Sized, R: RangeBounds<T>> LoopingRange<T> for R {
    fn contains_looping<U>(&self, item: &U) -> bool
    where
        T: PartialOrd<T>,
        T: PartialOrd<U>,
        U: ?Sized + PartialOrd<T>,
    {
        let start = self.start_bound();
        let end = self.end_bound();
        let is_reverse = match (start, end) {
            (Bound::Included(start), Bound::Included(end)) => start > end,
            (Bound::Included(start), Bound::Excluded(end))
            | (Bound::Excluded(start), Bound::Included(end))
            | (Bound::Excluded(start), Bound::Excluded(end)) => start >= end,
            _ => false,
        };
        if is_reverse {
            (start, Bound::Unbounded).contains(item) || (Bound::Unbounded, end).contains(item)
        } else {
            self.contains(item)
        }
    }
}

#[cfg(test)]
#[allow(clippy::reversed_empty_ranges)]
mod tests {
    use super::*;

    #[test]
    fn test_contains_looping() {
        let range = 1..=5;
        assert!(!range.contains_looping(&0));
        assert!(range.contains_looping(&1));
        assert!(range.contains_looping(&2));
        assert!(range.contains_looping(&3));
        assert!(range.contains_looping(&4));
        assert!(range.contains_looping(&5));
        assert!(!range.contains_looping(&6));

        let range = 5..=1;
        assert!(range.contains_looping(&0));
        assert!(range.contains_looping(&1));
        assert!(!range.contains_looping(&2));
        assert!(!range.contains_looping(&3));
        assert!(!range.contains_looping(&4));
        assert!(range.contains_looping(&5));
        assert!(range.contains_looping(&6));

        let range = 1..5;
        assert!(!range.contains_looping(&0));
        assert!(range.contains_looping(&1));
        assert!(range.contains_looping(&2));
        assert!(range.contains_looping(&3));
        assert!(range.contains_looping(&4));
        assert!(!range.contains_looping(&5));
        assert!(!range.contains_looping(&6));

        let range = 5..1;
        assert!(range.contains_looping(&0));
        assert!(!range.contains_looping(&1));
        assert!(!range.contains_looping(&2));
        assert!(!range.contains_looping(&3));
        assert!(!range.contains_looping(&4));
        assert!(range.contains_looping(&5));
        assert!(range.contains_looping(&6));
    }
}
