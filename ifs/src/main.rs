
fn check_val(var : i32) -> bool{ // checks val & returns boolean value
    if var % 2 == 0{
        return true
    }

    false
}
fn main() {
    
    let mut num : i32 = 0;

    loop {
        
    if check_val(num){
        println!("Jackpot! {}", num);
        if num > 1000{
            break;
        }
    }else{
        
        println!("not a Jackpot!");
        
    }
    num+=1;

    }

}

