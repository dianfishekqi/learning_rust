pub trait Sorter {
    fn sort<T>(self, slice: &mut [T])
    where
        T: Ord;
}

pub mod bubblesort;
pub mod insertionsort;
pub mod selectionsort;

// Smart sorts
pub mod quicksort;

#[cfg(test)]
mod tests {
    use super::*;
    struct StandardSorter;
    impl Sorter for StandardSorter {
        fn sort<T>(self, slice: &mut [T])
        where
            T: Ord,
        {
            slice.sort()
        }
    }

    #[test]
    fn sanity_check() {
        let mut slice = vec![2, 1];
        StandardSorter.sort(&mut slice);
        assert_eq!(slice, [1, 2]);
    }
}
