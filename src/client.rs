use serde::{Serialize, Deserialize};
use dotenv::dotenv;
use headless_chrome::Browser;
use std::error::Error;
use crate::processor::data::{Data, Course, Category, Assignment};

pub fn fetch_data() -> Result<(), Box<dyn Error>> {
    let (browser, tab) = initialize();
    let (browser, tab) = login(browser, tab)?;
    let data = Data::new();
    
    for num in (1..=13).step_by(3) {
        fetch_grades(browser.clone(), tab.clone(), num, data.clone())?;
    }

    fetch_grades(browser.clone(), tab.clone(), 1, data.clone())?;
    println!("Success!");

    Ok(()) // Return a successful result
}

fn initialize() -> (Browser, std::sync::Arc<headless_chrome::Tab>) {
    dotenv::dotenv().ok(); // Load environment variables

    let browser = Browser::default().unwrap(); // Initialize the browser
    let tab = browser.new_tab().unwrap(); // Unwrap the Result to get the tab

    println!("Browser initialized successfully");
    (browser, tab)
}

fn login(browser: Browser, tab: std::sync::Arc<headless_chrome::Tab>) -> Result<(Browser, std::sync::Arc<headless_chrome::Tab>), Box<dyn Error>> {
    println!("Navigating to login page...");
    tab.navigate_to("https://sis.powayusd.com/PXP2_Login_Student.aspx?regenerateSessionId=true").expect("Navigated to Login Page");

    println!("Logging in...");
    tab.wait_for_element("#ctl00_MainContent_username")?.click()?;
    tab.type_str(&std::env::var("USERNAME").expect("Failed").to_string())?;

    tab.wait_for_element("#ctl00_MainContent_password")?.click()?;
    tab.type_str(&std::env::var("PASSWORD").expect("Failed").to_string())?.press_key("Enter")?;

    println!("Login successful");

    tab.wait_for_element("div.lp-tile:nth-child(2)")?.click()?;
    println!("Move to gradebook");
    Ok((browser, tab))
}

pub fn fetch_grades(browser: Browser, tab: std::sync::Arc<headless_chrome::Tab>, num: i32, data: Data) -> Result<(), Box<dyn Error>> {
    println!("Entering calc");
    let element = tab.wait_for_element(&format!("div.row:nth-child({}) > div:nth-child(1) > button:nth-child(1)", num))?;
    element.click()?;
    let course = element;
    process_course(browser, tab.clone(), data, course.get_inner_text()?)?;

    Ok(())
}

pub fn process_course(browser: Browser, tab: std::sync::Arc<headless_chrome::Tab>, mut data: Data, course: String) -> Result<Data, Box<dyn Error>> {
    let course = Course::new(course);
    let mut categories: Vec<Category> = Vec::new();
    let mut assignments: Vec<Assignment> = Vec::new();

    for i in 1..=10 {
        let cat_type = tab
            .wait_for_element(format!("#CategoryWeightingGrid > div:nth-child(1) > div:nth-child(6) > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child({}) > td:nth-child(1)", i).as_str())?
            .get_inner_text()?.trim().to_string();
        let weight = tab.wait_for_element(format!("#CategoryWeightingGrid > div:nth-child(1) > div:nth-child(6) > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child({}) > td:nth-child(2)", i).as_str())?
            .get_inner_text()?.replace('%', "").trim().to_string().parse().unwrap_or(0.0);
        let points = tab.wait_for_element(format!("#CategoryWeightingGrid > div:nth-child(1) > div:nth-child(6) > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child({}) > td:nth-child(3)", i).as_str())?
            .get_inner_text()?.trim().to_string().parse().unwrap_or(0.0);
        let possible = tab.wait_for_element(format!("#CategoryWeightingGrid > div:nth-child(1) > div:nth-child(6) > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child({}) > td:nth-child(4)", i).as_str())?
            .get_inner_text()?.trim().to_string().parse().unwrap_or(0.0);

        categories.push(Category::new(cat_type.clone(), weight, points, possible));

        if cat_type == "TOTAL" {
            break;
        }
    }

    println!("{:?}", categories);

    for i in 1..=100 {
        let date_str = match tab.wait_for_element(format!(".dx-scrollable-content > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child({}) > td:nth-child(1)", i).as_str()) {
            Ok(element) => element.get_inner_text()?,
            Err(_) => {
                println!("No more rows found at index {}", i);
                break;
            }
        };
        let date = chrono::NaiveDate::parse_from_str(date_str.trim(), "%m/%d/%Y").unwrap_or_else(|_| {
            eprintln!("Failed to parse date: {}", date_str);
            chrono::NaiveDate::from_ymd_opt(1970, 1, 1).unwrap() // Default fallback date
        });
        let assign = tab.wait_for_element(format!(".dx-scrollable-content > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child({}) > td:nth-child(3)", i).as_str())?
            .get_inner_text()?.trim().to_string();
        let category = tab.wait_for_element(format!(".dx-scrollable-content > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child({}) > td:nth-child(4)", i).as_str())?
            .get_inner_text()?.trim().to_string();

        let score_text = tab.wait_for_element(format!(".dx-scrollable-content > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child({}) > td:nth-child(6)", i).as_str())?
            .get_inner_text()?.trim().to_string();
        let parts: Vec<&str> = score_text.split(" out of ").collect();
        let score = parts.get(0).and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
        let score_possible = parts.get(1).and_then(|s| s.parse::<f32>().ok()).unwrap_or(0.0);
        let score_type = tab.wait_for_element(format!(".dx-scrollable-content > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child({}) > td:nth-child(7)", i).as_str())?
            .get_inner_text()?.trim().to_string();
        let points_text = tab.wait_for_element(format!(".dx-scrollable-content > div:nth-child(1) > table:nth-child(1) > tbody:nth-child(2) > tr:nth-child({}) > td:nth-child(8)", i).as_str())?
            .get_inner_text()?.trim().to_string();
        let parts: Vec<&str> = score_text.split('/').collect();
        let points = parts.get(0).and_then(|s| s.trim().parse::<f32>().ok()).unwrap_or(0.0);
        let points_possible = parts.get(1).and_then(|s| s.trim().parse::<f32>().ok()).unwrap_or(0.0);
    
        assignments.push(Assignment::new(date, assign, category, score, score_possible, score_type, points, points_possible))
    }

    println!("{:?}", assignments);

    data.insert_course(course.clone());

    Ok(data)
}