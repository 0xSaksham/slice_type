fn main() {
    let s = String::from("Hello World");
    let _word = first_word(&s);

    let word_one = first_word(&s[0..6]);
    let word_two = first_word(&s[..]);

    // s.clear(); // error!

    // Slices with Arrays
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3];
    assert_eq!(slice,&[2,3]);

    println!("The slice is: {:?}\n", slice);
    
    println!("The first word is: {}\n", _word);
    println!("The first word is: {}\n", word_one);
    println!("The first word is: {}\n", word_two);


}
// Slice Type with &str
// fn first_word(s: &str) -> &str
fn first_word(s: &str) -> &str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}



// fn first_word(s: &String) -> usize {
//     let bytes = s.as_bytes();

//     for (i, &item) in bytes.iter().enumerate()
//     {
//         if item == b' '
//         {
//             return i;
//         }
//     }

//     s.len()
// }

