mod core;

use colored::*;

fn main() {
    core::logger::init_logging();

    let args = core::args::parse_args();

    if args.verbose {
        println!("{}", "Verbose mode enabled".yellow().bold().to_string());
        println!("{}: {}", "Request Method".green().bold().to_string(), args.method);
        println!("{}: {}", "Request URL".green().bold().to_string(), args.url);
        for header in &args.headers {
            println!("{}: {}", "Header".cyan(), header);
        }
        if let Some(body) = &args.body {
            println!("{}: {}", "Request Body".blue().bold().to_string(), body);
        }
    }

    match core::request::build_request(&args) {
        Ok(request) => match core::request::send_request(request) {
            Ok(response) => {
                if args.verbose {
                    println!("{}: {}", "Response Status".green().bold().to_string(), response.status());
                    println!("{}:", "Response Headers".green().bold().to_string());
                    for (key, value) in response.headers().iter() {
                        println!("{}: {}", key.to_string().cyan(), value.to_str().unwrap_or("").magenta());
                    }
                }
                core::response::handle_response(response);
            }
            Err(e) => eprintln!("Failed to send request: {}", e),
        },
        Err(e) => {
            eprintln!("Failed to build request: {}", e);
            std::process::exit(1);
        }
    }
}
