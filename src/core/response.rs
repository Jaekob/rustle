use reqwest::blocking::Response;
use colored::*;

pub fn handle_response(response: Response) {
    let status = response.status();
    let status_color = if status.is_success() {
        "Status".green().bold()
    } else if status.is_client_error() {
        "Status".yellow().bold()
    } else {
        "Status".red().bold()
    };

    println!("{}: {}", status_color, status);

    let headers = response.headers();
    for (key, value) in headers.iter() {
        println!("{}: {}", key.to_string().cyan(), value.to_str().unwrap_or("").magenta());
    }

    // Separate headers and body
    println!("\n{}", "-".repeat(40).blue());

    match response.text() {
        Ok(body) => println!("\n{}:\n{}", "Body".blue().bold(), body.white()),
        Err(e) => eprintln!("Error reading response body: {}", e),
    }
}

