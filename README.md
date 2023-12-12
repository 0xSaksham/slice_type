# Rust String Slicer

This Rust program demonstrates the use of string slices and arrays. It includes a function `first_word` that takes a string slice (`&str`) and returns the first word in the string.

## Usage

```rust
fn main() {
    let s = String::from("Hello World");
    let _word = first_word(&s);

    let word_one = first_word(&s[0..6]);
    let word_two = first_word(&s[..]);

    // Uncommenting the line below will result in a compilation error, showcasing Rust's ownership system.
    // s.clear();

    // Slices with Arrays
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice, &[2, 3]);

    println!("The slice is: {:?}\n", slice);
    
    println!("The first word is: {}\n", _word);
    println!("The first word is: {}\n", word_one);
    println!("The first word is: {}\n", word_two);
}
```

## `first_word` Function

The `first_word` function takes a string slice (`&str`) as input and returns a reference to the first word in the slice. It works by iterating through the byte representation of the string, looking for a space (`' '`), and returning the slice up to that point.

```rust
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}
```

## Ownership System

The program includes a commented-out line (`s.clear()`) that, if uncommented, will result in a compilation error. This showcases Rust's ownership system, preventing modifications to a borrowed value.

## Array Slicing

The program also demonstrates slicing arrays in Rust. It creates an array `a` and slices it to obtain a subset of elements.

Feel free to use, modify, and learn from this Rust example!
