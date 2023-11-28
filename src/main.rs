use std::io::{self, Write};
use tokio;
use utils::ai::get_response;
mod utils {
    pub mod ai;
}
#[tokio::main]
async fn main() {
    loop {
        let mut prompt = String::new();
        print!("YOU  >>> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut prompt).unwrap();
        let response = get_response(prompt.as_str()).await;
        println!("ANYA >>> {}", response);
    }
}
