pub fn selection_sort_fixed_array_descending(mut numbers: [i32; 7]) -> [i32; 7] {

   let array_length = numbers.len();

   for i in 0..array_length - 1 {
       let mut largest = numbers[i];
       let mut index = i;

       for j in i+1..array_length {
          if numbers[j] > largest {
              largest = numbers[j];
              index = j;
          } 
       }

       numbers.swap(i, index);
   }

    numbers
}
