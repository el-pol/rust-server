# Rust Server

A simple HTTP server written in Rust using only the standard library. This lightweight web server demonstrates basic HTTP request handling and static file serving without external dependencies.

## Features

- **Pure Rust Implementation**: Built using only Rust's standard library
- **HTTP/1.1 Support**: Handles basic HTTP GET requests
- **Static File Serving**: Serves HTML files from the project directory
- **Error Handling**: Returns custom 404 pages for non-GET requests
- **Lightweight**: No external dependencies required

## Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2021 edition or later)

### Installation

1. Clone the repository:
```bash
git clone https://github.com/el-pol/rust-server.git
cd rust-server
```

2. Build the project:
```bash
cargo build
```

### Running the Server

Start the server with:
```bash
cargo run
```

The server will start listening on `http://127.0.0.1:8000`.

## Usage

Once the server is running, you can:

- **Visit the homepage**: Navigate to `http://127.0.0.1:8000` in your browser to see the main page
- **Test error handling**: Try any other HTTP method or invalid endpoint to see the custom 404 page

### Example Requests

```bash
# Get the main page
curl http://127.0.0.1:8000

# Trigger 404 error (any non-GET request)
curl -X POST http://127.0.0.1:8000
```

## Project Structure

```
rust-server/
├── src/
│   └── main.rs          # Main server implementation
├── index.html           # Homepage content
├── 404.html            # Error page content
├── Cargo.toml          # Project configuration
├── .gitignore          # Git ignore rules
└── README.md           # This file
```

## How It Works

The server operates by:

1. **Binding to Port**: Listens for incoming connections on `127.0.0.1:8000`
2. **Request Processing**: Reads incoming HTTP requests into a buffer
3. **Route Handling**: 
   - GET requests → serves `index.html`
   - All other requests → serves `404.html`
4. **Response Generation**: Builds proper HTTP/1.1 responses with correct headers
5. **Connection Management**: Handles each connection synchronously

## Technical Details

- **Protocol**: HTTP/1.1
- **Port**: 8000
- **Host**: 127.0.0.1 (localhost)
- **Request Buffer**: 1024 bytes
- **Response Format**: Standard HTTP with Content-Length header

## Development

### Building
```bash
cargo build
```

### Checking Code
```bash
cargo check
```

### Running Tests
```bash
cargo test
```

## License

This project is open source. Please check the repository for license details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.