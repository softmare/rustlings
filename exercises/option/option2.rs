// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints


fn main() {
    // let optional_word = Some(String::from("rustlings"));
    let optional_word :Option<String> = None;
    // TODO: Make this an if let statement whose value is "Some" type
    if let Some(word) = optional_word {
        println!("The word is: {}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }
    optional_integers_vec.push(None);
    optional_integers_vec.push(Some(11));
    optional_integers_vec.push(Some(12));
    optional_integers_vec.push(None);
    optional_integers_vec.push(Some(13));

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let Some(integer) = optional_integers_vec.pop() {
        if let Some(i) = integer{
            println!("current value: {}", i);
        }
    }
}
