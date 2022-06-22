
use std::io;

fn main() {
    loop {
        println!("Please input a number.");
        
        let mut input = String::new();
        
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");
            
        let input: u32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        fib(input);
        break;

    }

}

fn fib(n: u32){
    if n==0 {
        println!("{}", 0);
    }
    else if n ==1 {
        println!("{}", 1);
    }
    else {
        let mut second_last_num :  u128 = 0;
        let mut last_num :  u128 = 1;

        for _number in 2..n+1 {
            let temp: u128 = second_last_num;
            second_last_num = last_num;
            last_num = temp + last_num;
        }

        println!("{}", last_num);
    }
}