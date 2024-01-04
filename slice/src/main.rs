fn main() {
    // println!("{}", first_word(&String::from("ALA gol")));
    
    // let str = String::from("ALA gol");
    // let size : (&str, usize) = first_word(&str);
    // println!("{}, {}", size.0, size.1);

    let word :String = String::from("Hola Mundo");

    let word2 :&str = &word[..5];
    println!("{}", word2);

    let sensize = sentinel_char(&word, 'a');
    println!("{}",sensize);
    
}

fn first_word(wrd: &String)-> (&str, usize) {
    let bytes = wrd.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return (&wrd[..i], i);
        }
    }
    (&wrd[..], wrd.len())
}

//returns length of str until sentinel character is found
fn sentinel_char(wrd: &String, sentinel: char) -> usize {

    //used to be able to iterate through wrd
    let bytes = wrd.as_bytes();

    for (i, &index) in bytes.iter().enumerate(){
        if index == sentinel as u8 { //casting sentinel
            return i;
        }
    }
    0
}
