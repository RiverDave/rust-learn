
fn main() {
    let var : i32 = 45;
    print_var(var);


    let x : i32 = 23; 
    let y : i32 = 23; 

    println!("Var sum of {} and {} :", x , y);
    println!("{}", sum_vars(x, y));


    // to modify the value below, we gotta make it mut
    let mut num = 0;
    println!("{}", num);
    add_one(&mut num); // passing mutable var by reference
    println!("{}", num);
    



}

fn add_one(var : &mut i32){ // pass by reference
    *var += 1; // expression since we don't return anything, we just add a value to a variable.
}


fn sum_vars(x: i32,  y : i32) -> i32{ // returns i32 as format -> specifies

    let res : i32 = x + y;
    res // this is how we return a value.
}

fn print_var(var: i32){
    println!("{}", var);
}
