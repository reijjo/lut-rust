pub mod receipt;
use receipt::content::*;

fn main() {
   if let Err(_) = store_loop() {
        println!("An error occurred during the store loop!");
    }
}
