# Rustle 🌐🚀

**Rustle** is a memory-safe, efficient HTTP client tool written in Rust, inspired by the popular `curl` utility. Rustle aims to provide a similar experience to `curl` while leveraging Rust's safety and performance features.

## Table of Contents 📚

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
- [Configuration](#configuration)
- [Examples](#examples)
- [Contributing](#contributing)
- [Testing](#testing)
- [License](#license)

## Features ✨

- Supports various HTTP methods: GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS.
- Handles headers, request bodies, and URL parameters.
- Verbose mode for detailed request and response information.
- Timeout handling and redirection following.
- Extensible and modular codebase.
- Error handling and logging support.
- Colorized output for better readability of status codes, headers, and body text.

## Installation 🛠️

To use Rustle, you need to have Rust installed on your system. You can install Rust using [rustup](https://rustup.rs/):

You can use... `curl` to install Rust... 🤦

```bash
curl --proto '=https' --tlsv1.2 -sSf https://rustup.rs | sh
```

Once Rust is installed, clone the Rustle repository and build the project:

```bash
git clone https://github.com/your-username/rustle.git
cd rustle
cargo build --release
```

The executable will be located in the `target/release` directory. You can move it to a directory in your PATH for easy access:

```bash
cp target/release/rustle /usr/local/bin
```

## Usage 🚀

Rustle is designed to work similarly to `curl`. Below are some common usage patterns:

### Basic GET Request 🌐

```bash
rustle http://example.com
```

### POST Request with JSON Data 📦

```bash
rustle -X POST -H "Content-Type: application/json" -d '{"key": "value"}' http://example.com
```

### Set Custom Headers 📋

```bash
rustle -H "Authorization: Bearer token" http://example.com
```

### Verbose Mode 🔍

Enable verbose mode to see detailed information about the request and response:

```bash
rustle -v http://example.com
```

## Configuration ⚙️

Rustle allows users to customize their settings through a configuration file located at `config/log4rs.yaml` in the project root. You can modify this file to adjust logging levels, formats, and other settings. In future versions, more configuration options will be added to customize text coloring and other user preferences.

## Examples 🧩

Here are some additional examples of how you can use Rustle:

### PUT Request with a Body 📝

```bash
rustle -X PUT -d "some data" http://example.com
```

### DELETE Request ❌

```bash
rustle -X DELETE http://example.com
```

### Using the PATCH Method 🔄

```bash
rustle -X PATCH -H "Content-Type: application/json" -d '{"update": "true"}' http://example.com
```

### Handling Timeouts ⏳

If the server does not respond within the set timeout duration, Rustle will terminate the request:

```bash
rustle -X GET --timeout 10 http://example.com
```

## Contributing 🤝

Contributions are welcome! To contribute to Rustle:

1. Fork the repository.
2. Create a new branch for your feature or bug fix.
3. Write your code, including tests if applicable.
4. Submit a pull request with a description of your changes.

Please make sure your code adheres to the coding standards and passes all existing tests.

## Testing 🧪

Rustle includes a robust set of unit tests to ensure reliability and correctness. To run the tests, use:

```bash
cargo test -- --nocapture
```

This will execute all tests and display detailed output, helping you diagnose any issues.

### Adding New Tests 📝

When contributing new features or fixing bugs, please include relevant unit tests. Tests can be added to the `tests` module within each core component file (e.g., `args.rs`, `request.rs`). Ensure your tests cover both typical usage scenarios and edge cases.

## License 📄

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for more details.
