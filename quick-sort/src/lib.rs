

fn partition(mut list: &mut Vec<i32>) -> usize{

    let len = list.len();
    let pivot = list[len - 1];
    let mut previous_index = 0;
    let mut current_index = 0;

    while current_index < len - 1 {
        if list[current_index] <= pivot {
            list.swap(previous_index, current_index);
            previous_index += 1;
        }
        current_index += 1;
    }
    list.swap(previous_index, len - 1);

    return previous_index;
}

fn quick_sort(mut list: &mut Vec<i32>, left: usize, right: usize) {
    if left < right {
        let partition_index = partition(list);
        quick_sort(list, left, partition_index - 1);
        quick_sort(list, partition_index + 1, right);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn partition_works1() {
        let mut list = Vec::from([1]);
        let expected_partition_index: usize = 0;
        let partition_index = partition(&mut list);

        assert_eq!(expected_partition_index, partition_index);
    }
    #[test]
    fn partition_works2() {
        let mut list = Vec::from([2,1]);
        let expected_partition_index: usize = 0;
        let partition_index = partition(&mut list);

        assert_eq!(expected_partition_index, partition_index);
    }
    #[test]
    fn partition_works3() {
        let mut list = Vec::from([2,5,13]);
        let expected_partition_index: usize = 2;
        let partition_index = partition(&mut list);

        assert_eq!(expected_partition_index, partition_index);
    }
    #[test]
    fn partition_works4() {
        let mut list = Vec::from([2,5,13,8]);
        let expected_partition_index: usize = 2;
        let partition_index = partition(&mut list);

        assert_eq!(expected_partition_index, partition_index);
    }
    #[test]
    fn partition_works5() {
        let mut list = Vec::from([2,7,4,9]);
        let expected_partition_index: usize = 3;
        let partition_index = partition(&mut list);

        assert_eq!(expected_partition_index, partition_index);
    }
    #[test]
    fn partition_works10() {
        let mut list = Vec::from([3,2,5,0,1,8,6,9,4]);
        let expected_partition_index: usize = 4;
        let partition_index = partition(&mut list);

        assert_eq!(expected_partition_index, partition_index);
    }

    #[test]
    fn everything_works() {
        let mut list = Vec::from([100,1,50,16]);
        let left = 0;
        let right = list.len() - 1;
        quick_sort(&mut list, left, right);
        assert_eq!(list, Vec::from([1,16,50,100]));
    }
}
