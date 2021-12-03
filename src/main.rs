use std::io;
use std::io::Write;

#[macro_use]
mod common;
mod solutions;

fn main() {
    let sol = [
        solutions::day_01_a::main(),
        solutions::day_01_b::main(),
        solutions::day_02_a::main(),
        solutions::day_02_b::main(),
        solutions::day_03_a::main(),
        solutions::day_03_b::main(),
    ];

    for (i, item) in sol.iter().enumerate() {
        println!("[{}] {}", i, item.name);
    }

    print!("\n❯ ");

    let mut buff = String::new();
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buff).unwrap();
    while buff.ends_with('\n') || buff.ends_with('\r') {
        buff.pop();
    }

    let num = match buff.parse::<usize>() {
        Ok(i) => i,
        Err(_) => {
            println!("Das not a number...");
            return;
        }
    };

    if num >= sol.len() {
        println!("[*] Invaild Id");
        return;
    }

    let this_sol = &sol[num];

    println!("[*] Running: {}", this_sol.name);
    let out = (this_sol.run)();
    println!("[+] OUT: {}", out);
}
