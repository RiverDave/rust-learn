
fn main() {
    let mut foo = String::from("This is a str");
    println!("Org str = {}", foo);
    foo = changes_it(foo);
    println!("changed str = {}", foo);


    let mut x :i32 = 23;
    println!("Org x = {}", x);
    x = do_operation(x);
    println!("Changed x = {}", x);




}

fn changes_it(mut var: String) -> String{
    println!("var has gotten in: {}", var);
    let foo = "Hello world";
    var = foo.to_string();
    var
}

fn do_operation(mut x :i32) -> i32{
    // let swap :&str = x.to_string();
    x+=10;
    x
}
