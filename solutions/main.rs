pub mod helpers;

mod day18;

fn main() {
    match day18::solve() {
        Ok(_) => println!("Done!"),
        Err(e) => println!("Error: {}", e)
    }
}
