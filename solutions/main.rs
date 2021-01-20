pub mod helpers;

mod day25;

fn main() {
    match day25::solve() {
        Ok(_) => println!("Done!"),
        Err(e) => println!("Error: {}", e)
    }
}
