// Fábio Gandini - 2022
// Rust sorting algorithm example.


//Rust Bubble Sort implementation:
//Just like how c++ can use templates to create polymorphic functions
//Rust can do the same, this Bubble Sort function works not only for numbers
// but, for example, strings too
fn bubble_sort<T: Ord>(vector: &mut [T]) {
    for i in 0..vector.len() {
        for j in 0..(vector.len() - 1 - i) {
            if vector[j] > vector[j + 1] {
                vector.swap(j, j + 1);
            }
        }
    }
}


fn main() {
    println!("Sorting integers:");
    let mut number_vector = [89, -11, 4, 7, 90, -1, 0, 100, -100, 50];
    println!("Original: {:?}", number_vector);
    bubble_sort(&mut number_vector);
    println!("Sorted:  {:?}\n", number_vector);

    println!("Sorting strings (alphabetically):");
    let mut string_vector = ["fire", "truck", "block", "tiger", "air", "water"];
    println!("Original: {:?}", string_vector);
    bubble_sort(&mut string_vector);
    println!("Sorted:  {:?}\n", string_vector);
}
