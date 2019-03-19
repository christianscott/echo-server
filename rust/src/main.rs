use std::io::{Write, BufRead, BufReader};
use std::net;
use std::thread;
use std::process;

fn main() {
    if let Err(err) = listen("127.0.0.1", 3000) {
        println!("error: {}", err);
        process::exit(1);
    }
}

fn listen(ip: &str, port: u32) -> Result<(), String> {
    let addr = format!("{}:{}", ip, port);
    println!("server listening on {}", addr);
    let listener = net::TcpListener::bind(addr)
        .map_err(|err| format!("could not start server: {}", err))?;

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream);
                });
            },
            Err(err) => {
                println!("could not connect to client: {}", err);
            },
        }
    }

    Ok(())
}

fn handle_client(stream: net::TcpStream) {
    let mut reader = BufReader::new(stream);
    loop {
        let mut line = String::new();
        reader.read_line(&mut line).unwrap();
        if let Err(err) = reader.get_ref().write(line.as_bytes()) {
            println!("could not write line: {}", err);
        }
    }
}

