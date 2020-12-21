pub mod helpers;

mod day12;

fn main() {
    match day12::solve() {
        Ok(_) => println!("Done!"),
        Err(e) => println!("Error: {}", e)
    }
}
