fn main() {
    let mut s = String::from("Hello World");
    let _word = first_word(&s);
    
    s.clear();
    println!("The first word is: {}", _word);
}

// Slice Type

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate()
    {
        if item == b' '
        {
            return i;
        }
    }

    s.len()
}
