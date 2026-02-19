pub fn find<T: PartialOrd>(array: &[T], key: T) -> Option<usize> {
    if array.len() == 0 {
        return None;
    }
    let middle_index = array.len() / 2;
    if array[middle_index] < key {
        let (_, right) = array.split_at(middle_index + 1);
        find(right, key).map(|f| middle_index + f + 1)
    } else if array[middle_index] > key {
        let (left, _) = array.split_at(middle_index);
        find(left, key).map(|f| middle_index - left.len() + f)
    } else {
        Some(middle_index)
    }
}
