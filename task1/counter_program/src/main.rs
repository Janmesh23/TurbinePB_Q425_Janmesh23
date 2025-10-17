use std::io::{self, Write};

struct counter {
    count: i32,
}
impl counter {
    fn new() -> Self {
        counter { count: 0 }
    }

    fn increment(&mut self) {
        self.count += 1;
    }

    fn decrement(&mut self) {
        self.count -= 1;
    }

    fn get_count(&self) -> i32 {
        self.count
    }

    fn reset(&mut self) {
        self.count = 0;
    }
}

fn main(){
    let mut counter = counter::new();
    println!("Hey there! Welcome to the Counter App.");
    loop {
        let result = counter.get_count();
        println!("Counter: {}", result);
        println!("Enter command (inc,dec,get,reset,exit): ");

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let command = input.trim();

        match command {
            "inc" => counter.increment(),
            "dec" => counter.decrement(),
            "get" => println!("Counter: {}", counter.get_count()),
            "reset" => counter.reset(),
            "exit" => break,
            _ => println!("Unknown command"),
        }
    }
}