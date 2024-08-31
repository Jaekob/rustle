use clap::{Arg, ArgAction, Command};

pub struct Args {
    pub method: String,
    pub url: String,
    pub headers: Vec<String>,
    pub body: Option<String>,
    pub verbose: bool,
}

pub fn parse_args() -> Args {
    let matches = Command::new("Rustle")
        .version("0.1")
        .author("Jaekob Childress")
        .about("Rustle: A memory-safe, efficient HTTP client tool written in Rust, inspired by curl.")
        .arg(
            Arg::new("method")
                .short('X')
                .long("request")
                .help("Specifies the request method to use (e.g., GET, POST, PUT, DELETE, PATCH, OPTIONS). Default is GET.")
                .num_args(1)
                .default_value("GET"),
        )
        .arg(
            Arg::new("url")
                .help("The URL to send the request to. This argument is required.")
                .required(true)
                .index(1),
        )
        .arg(
            Arg::new("headers")
                .short('H')
                .long("header")
                .help("Custom headers to include in the request. Use multiple -H options to specify more than one header.")
                .num_args(1)
                .action(ArgAction::Append),
        )
        .arg(
            Arg::new("body")
                .short('d')
                .long("data")
                .help("The data to send in the request body.")
                .num_args(1),
        )
        .arg(
            Arg::new("verbose")
                .short('v')
                .long("verbose")
                .help("Enable verbose output to see detailed information about the request and response.")
                .action(ArgAction::SetTrue),
        )
        .get_matches();

    let method = matches.get_one::<String>("method").unwrap_or_else(|| {
        eprintln!("Error: No method specified. Use -X to specify an HTTP method.");
        std::process::exit(1);
    }).to_uppercase();

    let url = matches.get_one::<String>("url").unwrap_or_else(|| {
        eprintln!("Error: No URL provided. Please specify a valid URL.");
        std::process::exit(1);
    }).to_string();

    Args {
        method,
        url,
        headers: matches
            .get_many::<String>("headers")
            .map_or_else(Vec::new, |v| v.map(|s| s.to_string()).collect()),
        body: matches.get_one::<String>("body").map(|s| s.to_string()),
        verbose: matches.get_flag("verbose"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use clap::{Arg, ArgAction, Command};

    fn build_test_command(args: Vec<&str>) -> Args {
        let matches = Command::new("Rustle")
            .version("0.1")
            .author("Jaekob Childress")
            .about("A memory-safe, efficient curl clone written in Rust")
            .arg(
                Arg::new("method")
                    .short('X')
                    .long("request")
                    .help("Specifies the request method to use")
                    .num_args(1)
                    .default_value("GET"),
            )
            .arg(
                Arg::new("url")
                    .help("The URL to send the request to")
                    .required(true)
                    .index(1),
            )
            .arg(
                Arg::new("headers")
                    .short('H')
                    .long("header")
                    .help("Custom headers to include in the request")
                    .num_args(1)
                    .action(ArgAction::Append),
            )
            .arg(
                Arg::new("body")
                    .short('d')
                    .long("data")
                    .help("The data to send in the request body")
                    .num_args(1),
            )
            .arg(
                Arg::new("verbose")
                    .short('v')
                    .long("verbose")
                    .help("Enable verbose output")
                    .action(ArgAction::SetTrue),
            )
            .get_matches_from(args);

        Args {
            method: matches.get_one::<String>("method").unwrap().to_uppercase(),
            url: matches.get_one::<String>("url").unwrap().to_string(),
            headers: matches
                .get_many::<String>("headers")
                .map_or_else(Vec::new, |v| v.map(|s| s.to_string()).collect()),
            body: matches.get_one::<String>("body").map(|s| s.to_string()),
            verbose: matches.get_flag("verbose"),
        }
    }

    #[test]
    fn test_parse_args_get_method() {
        let args = build_test_command(vec!["Rustle", "http://example.com"]);
        assert_eq!(args.method, "GET");
        assert_eq!(args.url, "http://example.com");
        assert!(args.headers.is_empty());
        assert!(args.body.is_none());
        assert!(!args.verbose); // Verbose should be false by default
    }

    #[test]
    fn test_parse_args_post_method_with_body() {
        let args = build_test_command(vec![
            "Rustle",
            "-X",
            "POST",
            "-d",
            "{\"key\": \"value\"}",
            "http://example.com",
        ]);
        assert_eq!(args.method, "POST");
        assert_eq!(args.url, "http://example.com");
        assert!(args.headers.is_empty());
        assert_eq!(args.body, Some("{\"key\": \"value\"}".to_string()));
        assert!(!args.verbose); // Verbose should be false by default
    }

    #[test]
    fn test_parse_args_with_headers() {
        let args = build_test_command(vec![
            "Rustle",
            "-H",
            "Content-Type: application/json",
            "-H",
            "Accept: */*",
            "http://example.com",
        ]);
        assert_eq!(args.method, "GET");
        assert_eq!(args.url, "http://example.com");
        assert_eq!(args.headers, vec!["Content-Type: application/json", "Accept: */*"]);
        assert!(!args.verbose); // Verbose should be false by default
    }

    #[test]
    fn test_parse_args_verbose_flag() {
        let args = build_test_command(vec!["Rustle", "-v", "http://example.com"]);
        assert_eq!(args.method, "GET");
        assert_eq!(args.url, "http://example.com");
        assert!(args.headers.is_empty());
        assert!(args.body.is_none());
        assert!(args.verbose); // Verbose should be true when flag is set
    }
}

