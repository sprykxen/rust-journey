pub fn bubble_sort_custom_condition(mut numbers: [i32; 5]) -> [i32; 5] {

    let required_pass = numbers.len() - 1;

    for i in 0..required_pass {

        let mut swapped = false;

        for j in 0..required_pass - i {
            if numbers[j] % 2 != 0 && numbers[j + 1] % 2 == 0 {
                numbers.swap(j, j + 1);
                swapped = true;
            }
        }

        if swapped == false {
            break;
        }
    }

    numbers
}
