use std::fs;
use std::io::BufReader;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use webstuff::ThreadPool;

//TODO Add error handling to every unwrap
//TODO Organize data into a tree of routes and subroutes eg. /test/ -> page1, page2
//TODO                                                       /main/ -> page3, page4
fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming().take(100) {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });

    }

    println!("Shutting down.")
}


fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let request_line = buf_reader.lines().next().unwrap().unwrap();
    println!("{}",request_line);
    let (status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}

//
//Add more documentation to ThreadPool and its public methods.
//Add tests of the library’s functionality.
//Change calls to unwrap to more robust error handling.
//Use ThreadPool to perform some task other than serving web requests.
//Find a thread pool crate on crates.io and implement a similar web server using the crate instead. Then compare its API and robustness to the thread pool we implemented.