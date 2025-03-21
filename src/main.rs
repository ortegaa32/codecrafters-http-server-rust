// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};
use std::io::{BufReader, prelude::*, Result};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    // println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream).unwrap();
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn read_from_header(header: Vec<&str>, item: &str) -> Option<String> {
    for h in header {
        println!("{}", h);
        if h.to_string().starts_with(item) {
            return Some(h.replace(&format!("{item}: "), ""));
        }
    }
    None
}

fn handle_connection(mut stream: TcpStream) -> Result<()> {
    let buf = BufReader::new(&mut stream);

    let req_line = buf.lines().next().unwrap().unwrap();
    let request = req_line.as_str();
    let lines: Vec<&str> = request.split("\r\n\r\n").collect();
    let status: Vec<&str> = lines[0].split(" ").collect();

    match status[0] {
        "GET" => {
            let response: String = {
                if status[1] == "/" {
                    String::from("HTTP/1.1 200 OK\r\n\r\n")
                } else if status[1].starts_with("/echo/") {
                    let variable = status[1].replace("/echo/", "");
                    format!("HTTP/1.1 200 OK\r\nContent-type: text/plain\r\nContent-length: {}\r\n\r\n{}", variable.len(), variable)
                } else if status[1].starts_with("/user-agent") {
                    let header: Vec<&str> = lines[0].split("\r\n").collect();
                    match read_from_header(header, "User-Agent") {
                        Some(agent) => format!("HTTP/1.1 200 OK\r\nContent-type: text/plain\r\nContent-length: {}\r\n\r\n{}", agent.len(), agent),
                        None => String::from("")
                    };
                } else {
                    String::from("HTTP/1.1 404 Not Found\r\n\r\n")
            }};
            let _ = stream.write_all(response.as_bytes());
        }
        _ => {
            println!("Unknown Method: {}", status[0])
        }
    }
    Ok(())
}