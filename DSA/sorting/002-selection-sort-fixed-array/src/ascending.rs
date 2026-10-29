pub fn selection_sort_fixed_array_ascending(mut numbers: [i32; 7]) -> [i32; 7]{
    
    let array_length = numbers.len();

    for i in 0..array_length {
        let mut smallest = numbers[i];
        let mut index = i;

        for j in i+1..array_length {
            if numbers[j] < smallest {
                smallest = numbers[j];
                index = j;
            }
        }

        numbers.swap(i, index);

    }

    numbers

}
