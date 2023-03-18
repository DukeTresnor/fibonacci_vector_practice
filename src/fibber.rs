pub fn calculate_fibonacci_number(input: i32) -> i32 {
    if input < 2 {
        return input;
    }
    calculate_fibonacci_number(input - 1) + calculate_fibonacci_number(input - 2)
}

pub fn make_fibonacci_vector(vector_size: i32) -> Vec<i32> {
    let mut fib_vector: Vec<i32> = Vec::new();

    for number in 0..vector_size+1 {
        fib_vector.push(calculate_fibonacci_number(number));
    }

    fib_vector
}