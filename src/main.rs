use std::io;
use std::cmp::Ordering;

fn recheck(random_num: &i32) -> Ordering {
    println!("write ur num");
    let mut num = String::new();
    io::stdin()
        .read_line(&mut num)
        .expect("err");
    let num: i32 = match num.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("try number nex time");
            0
        }, 
    };
    println!("ur num: {num}");
    num.cmp(&random_num) 
}

fn loop_to_ten(random_num: &i32) {
    let mut i = 0;
    let mut ten = 10;
    loop {
        i = i + 1;
        match (i.cmp(&ten)) {
            Ordering::Less => println!("-------\nthis is loop #{} out of {ten}", i),
            Ordering::Greater => println!("_"),
            Ordering::Equal => {
                println!("u r looser");       
                break;
            }
        }
        match recheck(&random_num) {
            Ordering::Less => println!("ur num is less"),
            Ordering::Greater => println!("ur num is grater"),
            Ordering::Equal => {
                println!("u mf right");
                break;
            },
        }
    }
}

fn main() {
    println!("start, write ur num");
    // let random_num = rand::thread_rng().gen_range(1..=100);
    let random_num: i32 = 69; // fake rundom for now, cause i dont have internet on airplane :) 
    loop_to_ten(&random_num);
}

// thats it thanks for watching :)
