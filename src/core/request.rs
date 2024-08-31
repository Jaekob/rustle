use reqwest::blocking::{Client, RequestBuilder, Response};
use crate::core::args::Args;
use std::error::Error;

pub fn build_request(args: &Args) -> Result<RequestBuilder, Box<dyn Error>> {
    let client = Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .redirect(reqwest::redirect::Policy::limited(10))
        .build()?;

    let mut request = match args.method.as_str() {
        "POST" => client.post(&args.url),
        "PUT" => client.put(&args.url),
        "DELETE" => client.delete(&args.url),
        "HEAD" => client.head(&args.url),
        "PATCH" => client.patch(&args.url),
        "OPTIONS" => client.request(reqwest::Method::OPTIONS, &args.url),
        "GET" => client.get(&args.url),
        _ => {
            return Err(format!("Unsupported HTTP method '{}'.", args.method).into());
        }
    };

    // Add headers
    for header in &args.headers {
        if let Some((key, value)) = header.split_once(": ") {
            request = request.header(key, value);
        }
    }

    // Add body if present
    if let Some(body) = &args.body {
        request = request.body(body.clone());
    }

    Ok(request)
}

pub fn send_request(request: RequestBuilder) -> Result<Response, reqwest::Error> {
    request.send().map_err(|e| {
        eprintln!("Error sending request: {}", e);
        e
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::args::Args;
    use reqwest::blocking::Client;
    use reqwest::Method;
    use std::time::Duration;
    use std::thread;
    use std::net::TcpListener;

    #[test]
    fn test_build_get_request() {
        let args = Args {
            method: "GET".to_string(),
            url: "http://example.com".to_string(),
            headers: vec![],
            body: None,
            verbose: false,
        };

        let built_request = build_request(&args).unwrap().build().unwrap();
        assert_eq!(built_request.method(), Method::GET);
        assert_eq!(built_request.url().as_str(), "http://example.com/");
    }

    #[test]
    fn test_build_post_request_with_headers_and_body() {
        let args = Args {
            method: "POST".to_string(),
            url: "http://example.com".to_string(),
            headers: vec!["Content-Type: application/json".to_string()],
            body: Some("{\"key\": \"value\"}".to_string()),
            verbose: false,
        };
    
        let request_builder = build_request(&args).unwrap();
        let built_request = request_builder.build().unwrap();
    
        println!("Headers: {:?}", built_request.headers());
    
        assert_eq!(built_request.method(), Method::POST);
        assert_eq!(built_request.url().as_str(), "http://example.com/");
        assert_eq!(built_request.headers().get("Content-Type").map(|h| h.to_str().unwrap()), Some("application/json"));
    
        let body = built_request.body().and_then(|b| b.as_bytes());
        assert_eq!(body, Some("{\"key\": \"value\"}".as_bytes()), "Expected body to be set correctly");
    }        

    #[test]
    fn test_build_with_invalid_method() {
        let args = Args {
            method: "INVALID".to_string(),
            url: "http://example.com".to_string(),
            headers: vec![],
            body: None,
            verbose: false,
        };
    
        let result = build_request(&args);
        assert!(result.is_err(), "Expected an error for invalid HTTP method.");
    }
    
    #[test]
    fn test_request_timeout_behavior() {
        // Start a test server that doesn't respond
        let listener = TcpListener::bind("127.0.0.1:0").unwrap(); // bind to a free port
        let addr = listener.local_addr().unwrap();

        // Spawn a thread to accept the connection and then sleep
        thread::spawn(move || {
            for stream in listener.incoming() {
                if let Ok(_stream) = stream {
                    thread::sleep(Duration::from_secs(60)); // Simulate long delay
                }
            }
        });

        // Prepare the arguments
        let args = Args {
            method: "GET".to_string(),
            url: format!("http://{}", addr),
            headers: vec![],
            body: None,
            verbose: false,
        };

        // Build the request using the client with a 2-second timeout
        let client = Client::builder()
            .timeout(Duration::from_secs(2)) // Set a short timeout for testing
            .build()
            .expect("Failed to build client");

        let built_request = client.get(&args.url).build().unwrap();
        let result = client.execute(built_request);

        assert!(result.is_err());
        if let Err(err) = result {
            assert!(err.is_timeout());
        }
    }

    pub fn build_request(args: &Args) -> Result<RequestBuilder, Box<dyn Error>> {
        // Validate URL before proceeding
        let parsed_url = args.url.parse::<reqwest::Url>().map_err(|e| {
            let error_message = format!("Invalid URL '{}': {}", args.url, e);
            Box::<dyn Error>::from(error_message)
        })?;
    
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::limited(10))
            .build()?;
    
        let mut request = match args.method.as_str() {
            "POST" => client.post(parsed_url),
            "PUT" => client.put(parsed_url),
            "DELETE" => client.delete(parsed_url),
            "HEAD" => client.head(parsed_url),
            "PATCH" => client.patch(parsed_url),
            "OPTIONS" => client.request(reqwest::Method::OPTIONS, parsed_url),
            "GET" => client.get(parsed_url),
            _ => {
                return Err(format!("Unsupported HTTP method '{}'.", args.method).into());
            }
        };
    
        for header in &args.headers {
            if let Some((key, value)) = header.split_once(": ") {
                request = request.header(key, value);
            }
        }
    
        if let Some(body) = &args.body {
            request = request.body(body.clone());
        }
    
        Ok(request)
    }    
}
