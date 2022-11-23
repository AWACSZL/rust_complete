//不得在这里导入模块！！


pub mod guessnum{

    //注意在模块内导入其他模块等不得在外面
    use rand::Rng;
    use std::cmp::Ordering;
    use std::{io, char, str::from_boxed_utf8_unchecked};

    
    pub fn guessnum(){

        println!("Guess the number!");
    
        let secret_number = rand::thread_rng().gen_range(1..101);
    
        loop {
            println!("Please input your guess.");
    
            let mut guess = String::new();
    
            io::stdin().read_line(&mut guess).expect("Failed to read line");
    
            let guess: u32 = match guess.trim().parse() {
                Ok(num) => num,
                Err(_) => continue,
            };
    
            println!("You guessed: {}", guess);
    
            match guess.cmp(&secret_number) {
                Ordering::Less => println!("Too small!"),
                Ordering::Greater => println!("Too big!"),
                Ordering::Equal => {
                    println!("You win!");
                    break;
                }
            }
        }
    
    }


}