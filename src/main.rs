use dotenv::dotenv;
use std::error::Error;

use headless_chrome::Browser;
use headless_chrome::protocol::cdp::Page;

mod client;
mod processor;

fn main() -> Result<(), Box<dyn Error>> {
    client::fetch_data();

    Ok(())
}
