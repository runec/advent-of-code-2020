pub mod helpers;

mod day20;

fn main() {
    match day20::solve() {
        Ok(_) => println!("Done!"),
        Err(e) => println!("Error: {}", e)
    }
}
