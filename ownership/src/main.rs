
fn main() {
    let _v = "hello"; //str: non mutable(constant), better performance
                     //can also be declared as:
    let _v2: &str = "Hello";

    let mut foo = String::from("Hello this, is a mut string"); // mutable, dynamic, suited to input/text, vector under the
                                                           // hood. Just like the std::string in c++, it is a wrapper
                                                           // class of a char.
    let str2 = foo; //str being initialized out of other str
    let mut str3 :String = String::from("This is a test"); //

    //all of the String:: variables are located on the heap, while the &str members are on the
    //stack since they can't change at all

    str3.push_str(" And this was appended"); //append str on mut string
    str3.push('e'); //append single char on mut string

    str3 = _v.to_string();
    takes_ownership(str3); //str3 is moved to another scope and its memory is freed therefore it is
                           //not valid anymore

    // println!("{}", str3); //-> wont compile
    
    
    // println!("str3 = {}, str2 = {} , _v = {}", str3,str2, _v); 
    // println!("{} {}", str3, str2);
}



fn takes_ownership(var: String){
    println!("{}", var);
    // var
}
