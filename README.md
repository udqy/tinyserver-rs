
# tinyserver-rs

A simple Web Server written in Rust.
> This is a fun project I made to learn Rust and the working of a web server.
> It does not belong in a production environment.

####  Quick start:
```bash
git clone https://github.com/udqy/tinyserver-rs.git
cd tinyserver-rs
cargo run
```

#### Test:
```bash
ab -n 500 -c 4 http://localhost:8420/
```

#### Features:
- Multi-threaded connection handling   
- Implements a ThreadPool to manage worker threads  
- Can only handle GET requests as of now for static files
- Gracefully handles errors (depending on what graceful means to you)  
