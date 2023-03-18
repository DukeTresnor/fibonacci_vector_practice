pub fn make_number_vector(number_list: &[i32]) -> Vec<i32> {
    let mut changing_numbers: Vec<i32> = Vec::new();
    for list_element in number_list {
        changing_numbers.push(*list_element);
    }
    for number in &mut changing_numbers {
        let old_number = *number;
        *number += 6;
        println!("Old number: {} ==> New number: {}", old_number, number);
    }
    changing_numbers
}

pub fn print_element_of_integer_vector(vector: &Vec<i32>, desired_index: usize) {
    // indeces need to be of type usize
    // desired_element still needs to be an &i32
    // do i need references in this let line below \/ \/ \/ ??
    let desired_element: &i32 = &vector[desired_index];
    println!("Element at index: {} is: {}", desired_index, desired_element);
}