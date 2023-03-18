use clap::Parser;

mod fibber;

// This code block lets you add arguments on the command line
// in the command line, do cargo run -- --desired_vector_size the_size_number
#[derive(Parser)]
#[clap(author, version, about)]
struct Arguments {
    #[clap(short = 'S', long, help = "The number of elements that you want in your vector")]
    desired_vector_size: usize,
}


fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

pub fn print_vector(vector: &Vec<i32>) {
    for element in vector {
        println!("Element: {}", element);
    }
}

fn main() {

    let arguments = Arguments::parse();
    dbg!(arguments.desired_vector_size);

    let converted_desired_vector_size: i32 = arguments.desired_vector_size as i32;
    let my_second_fibon_vector = fibber::make_fibonacci_vector(converted_desired_vector_size);
    print_type_of(&my_second_fibon_vector);
    print_vector(&my_second_fibon_vector);

}

