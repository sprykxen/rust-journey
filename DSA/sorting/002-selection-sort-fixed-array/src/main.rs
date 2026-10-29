mod ascending;
mod descending;

fn main() {
    let numbers = [2,7,5,6,8,3,4];
    let ascending = ascending::selection_sort_fixed_array_ascending(numbers);
    let descending = descending::selection_sort_fixed_array_descending(numbers);
    println!("Original: {numbers:?}");
    println!("Ascending: {ascending:?}");
    println!("Descending: {descending:?}");
}
