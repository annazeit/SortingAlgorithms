pub fn insertion_sort (mut list: Vec<i32>) -> Vec<i32> {
    let mut previous_index = 0;
    let mut current_index = 1;
    // let mut previous_value =  list[previous_index];
    // let mut current_value =  list[current_index];

    // for i in 1..list.len()  {
    //     if current_value < previous_value {
    //         &mut list.swap(previous_index, current_index);
    //
    //         previous_index += 1;
    //         current_index += 1;
    //         if current_index >= list.len() {
    //             current_index -= 1;
    //         }
    //         current_value = list[current_index];
    //         previous_value = list[previous_index];
    //     }
    // }

    while current_index < list.len() {
        previous_index = current_index;
        while previous_index > 0 && list[previous_index-1] > list[previous_index] {
            &mut list.swap(previous_index, previous_index - 1);
            previous_index -= 1;
        }
        current_index += 1;
    }

    return list;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let list = Vec::from([8,7,3,9,1]);
        let expected_result = Vec::from([1,3,7,8,9]);
        assert_eq!(expected_result, insertion_sort(list));
    }
}
