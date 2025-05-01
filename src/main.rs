use std::error::Error;
use dotenv::dotenv;

use headless_chrome::Browser;
use headless_chrome::protocol::cdp::Page;

fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok(); // Load environment variables

    let browser = Browser::default()?; // Initialize the browser
    let tab = browser.new_tab()?; // Unwrap the Result to get the tab
    println!("Browser initialized successfully");

    println!("Navigating to login page...");
    tab.navigate_to("https://sis.powayusd.com/PXP2_Login_Student.aspx?regenerateSessionId=true").expect("Navigated to Login Page");

    println!("Logging in...");
    tab.wait_for_element("#ctl00_MainContent_username")?.click()?;
    tab.type_str(&std::env::var("USERNAME").expect("Failed").to_string());

    tab.wait_for_element("#ctl00_MainContent_password")?.click()?;
    tab.type_str(&std::env::var("PASSWORD").expect("Failed").to_string())?.press_key("Enter")?;

    println!("Test grabbing greeting...");
    let elem = tab.wait_for_element("#Greeting")?;
    println!("{}", elem.get_inner_text()?);

    tab.wait_for_element("div.lp-tile:nth-child(2)")?.click()?;

    let elem2 = tab.wait_for_element("#GradebookHeader")?;
    println!("{}", elem2.get_inner_text()?);

    println!("Entering calc");
    tab.wait_for_element("div.row:nth-child(1) > div:nth-child(1) > button:nth-child(1)")?.click()?;
    println!("Trying to grab class");
    let elem3 = tab.wait_for_element("#CategoryWeightingGrid > div:nth-child(1) > div:nth-child(6) > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child(1) > td:nth-child(3)")?;
    println!("{}", elem3.get_inner_text()?);


    Ok(()) // Return a successful result
}