
struct Person {
    name: String,
    age: i32,
}

impl Person {

    fn change(&mut self, name: String, age: i32) {
        self.name = name;
        self.age = age;
    }
}

fn print_person(person: &Person) {
    println!("{} is {} years old.", person.name, person.age);
}

//is this statement & function correct?
//return reference to oldest person, the lifetime specifier indicates that both of the parameters
//need to be in the same scope 1as the return value, so that if one of 1the parameters is dropped
//1the return value will not be invalid and we're not dere1ferencing a dangling pointer.
fn biggest_person<'a>(p1: &'a Person, p2 :&'a Person) -> &'a Person { 
    if p1.age > p2.age {
        &p1
    } else {
        &p2
    }
}

//same principle could be applied to a struct with a reference to another variable

struct Dawg<'a> {
    name: String,
    age: &'a i32,
}



fn main() {

    let mut p1 = Person { name: "John".to_string(), age: 20 };
    let mut p2 = Person { name: "Jane".to_string(), age: 30 };

    print_person(&p1);
    print_person(&p2);

    let mut oldest = biggest_person(&p1, &p2);
    println!("{} is the oldest person.", oldest.name);

    p1.change("Paul Mcartney".to_string(), 81);

    // it is necessary to reassign the oldest variable, because the reference is no longer valid
    oldest = biggest_person(&p1, &p2);
    println!("{} is the oldest person.", oldest.name);
}
