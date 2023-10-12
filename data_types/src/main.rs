use std::io;

fn main() {


    let _guess: u32 = "42".parse().expect("Not a number!"); //parsing char to type : u32
    let tup: (u32, i64, u32) = (_guess, 345, 432);

    let (x, y, z) = tup; // binding x,y,z to tupple structure

    println!("Playing around with tuples: {}, {}, {}", tup.0, tup.1, tup.2 ); // Since tuple are a class it is accessed as if
                                                  // it was an object with the . operator
    println!("Plying around with bound tuples : {}, {}, {}", x, y, z ); 

    let foo = [1,2,3,4]; // c- styled array

    let _stat_arr: [i32;5] = [1,2,3,4,5]; //defining [type:size] 

    let _data: i32 = foo[0]; // accessing array just like in c

    println!("Enter inx");

    let mut selection = String::new();

    io::stdin()
        .read_line(&mut selection)
        .expect("Failed to read user input");


    let indx : usize = selection
        .trim()
        .parse()
        .expect("Couldn't read index from input");

    println!("Your return index is {indx} of value : {}", _stat_arr[indx]);


        


}
