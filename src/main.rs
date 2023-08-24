use std::time::Duration;
use std::time::Instant;
use rand::Rng;
use std::io;

fn main() {
    println!("Hello, challanger!");
    println!("When you read the word Rust, press quickly enter");
    println!("Your reflex time will be shown afterwards.");
   
    let rand_sec = rand::thread_rng()
        .gen_range(5..10);
    let rand_nano = rand::thread_rng()
        .gen_range(1..999);
    let rand_dur = Duration::new(rand_sec, rand_nano);
    let wait = Instant::now();
    while wait.elapsed() < rand_dur {
    }
    println!("Rust");
    let rflx_time = Instant::now();
    rflx_action();
    let elapsed_time = rflx_time.elapsed();
    println!("Your reflex time is: {:?}" , elapsed_time);
}

fn rflx_action(){
    let mut enter = String::new(); 
    io::stdin()
        .read_line(&mut enter)
        .expect("Failed to read line");
}
