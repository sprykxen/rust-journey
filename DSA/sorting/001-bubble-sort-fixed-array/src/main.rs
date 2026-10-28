mod ascending;
mod descending;
mod custom_condition;

fn main() {
    let numbers = [8,3,6,1,5];

    let ascending_numbers = ascending::bubble_sort_ascending(numbers);
    let descending_numbers = descending::bubble_sort_descending(numbers);
    let custom_condition = custom_condition::bubble_sort_custom_condition(numbers);

    println!("Original: {numbers:?}");
    println!("Ascending: {ascending_numbers:?}");
    println!("Descending: {descending_numbers:?}");
    println!("Custom Condition: {custom_condition:?}");
}
