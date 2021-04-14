/// A cache-aware search function.
pub fn find<T>(collection: &Vec<T>, item: &T) -> Option<usize>
where T: Ord
{
    // TODO: Research cache sizes and what we need here. Line size? Total cache size?
    let cache_size = 0;
    let jump_size = get_optimal_jump_size(collection);
    // TODO-Q: Boundary analyze? Should this be <= or <?
    if cache_size <= jump_size {
        return match collection.binary_search(item) {
            Ok(index) => Some(index),
            Err(_) => None,
        }
    } else {
        return find_jump_with_size(collection, item, jump_size);
    }
}

/// Public wrapper function for the Jump Search implementation of find.
pub fn find_jump<T>(collection: &Vec<T>, item: &T) -> Option<usize>
where T: Ord
{
    find_jump_with_size(collection, item, get_optimal_jump_size(collection))
}

/// Core jump search algorithm.
fn find_jump_with_size<T>(collection: &Vec<T>, item: &T, jump_size: usize) -> Option<usize>
where T: Ord
{
    let mut i = jump_size;
    while i < collection.len() {
        if collection[i] == *item {
            return Some(i);
        }
        if collection[i] > *item {
            if let Some(idx) = linear_search(collection, item, i - jump_size, i) {
                return Some(idx);
            }
        }
        i += jump_size;
    }
    linear_search(collection, item, i - jump_size, collection.len())
}

/// Helper function for jump search that linearly searches through an interval.
fn linear_search<T>(collection: &Vec<T>, item: &T, left: usize, right: usize) -> Option<usize>
where T: Ord
{
    match collection[left..right].iter().position(|v| v == item) {
        Some(idx) => Some(left + idx),
        None => None,
    }
}

/// Returns the square root of the collection size, which is mathematically proven to be the optimal jump size for Jump Search.
fn get_optimal_jump_size<T>(collection: &Vec<T>) -> usize {
    ((collection.len() as f64).sqrt()) as usize
}

mod tests {
    // TODO-Q: Why do we have to do this?
    #[allow(unused_imports)]
    use super::find_jump;

    #[test]
    fn test_find_jump() {
        let vec = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987, 1597, 2584, 4181, 6765, 10946, 17711, 28657, 46368, 75025, 121393, 196418, 317811];
        // TODO: Find a way to make it so that you don't have to specify a borrow on literals.
        assert_eq!(Some(0), find_jump(&vec, &0));
        assert_eq!(Some(4), find_jump(&vec, &3));
        assert_eq!(Some(5), find_jump(&vec, &5));
        assert_eq!(Some(6), find_jump(&vec, &8));
        assert_eq!(Some(9), find_jump(&vec, &34));
        assert_eq!(Some(10), find_jump(&vec, &55));
        assert_eq!(Some(11), find_jump(&vec, &89));
        assert_eq!(Some(28), find_jump(&vec, &317811));
        assert_eq!(None, find_jump(&vec, &500));
    }
}