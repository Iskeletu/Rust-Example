// Fábio Gandini - 2022
// Simple rust Fibonacci sequence implementation.

//Prints fibonacci sequence given a sequence size as a limiter.
fn fibonacci(sequence_size: i32) {
    if sequence_size > 0 {
        let mut x = 0;
        let mut y = 1;
        
        for _value in 0..sequence_size {
            println!("{}", x);
            x = x + y;
            y = x - y;
        }
    }
}


fn main() {
    fibonacci(10);
}
