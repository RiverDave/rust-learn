fn main() {
    println!("{}", first_word(&String::from("manri gol")));
}

fn first_word(wrd: &String)-> usize {
    let bytes = wrd.as_bytes();

    for(i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }
    



    wrd.len()
}
