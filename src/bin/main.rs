extern crate web_server;

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;

fn main() {
    // println!("Hello, world!");
    let host_address = "127.0.0.1:8080";
    let thread_pool_size = 5;
    let listener =  TcpListener::bind(host_address);
    let mut pool = ThreadPool::new(thread_pool_size);
    let mut count = 0;

    for stream in listener.unwrap().incoming() {
        if count > 5 {
            println!("Shut Down initiated from main");
            break;
        }
        count += 1;
        let stream = stream.unwrap();
        println!("Connection established");
        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let response = "\
                            HTTP/1.1 200 OK\r\n\
                            Content-Type: text/html\r\n\
                            \r\n\
                            <!DOCTYPE html>\r\n\
                            <html lang=\"en\">\r\n\
                            <head>\r\n\
                                <meta charset=\"UTF-8\">\r\n\
                                <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\r\n\
                                <title>Rustacean Page</title>\r\n\
                            </head>\r\n\
                            <body>\r\n\
                                <h1>Hello</h1>\r\n\
                                <p>I am a Rustacean, a Good One</p>\r\n\
                            </body>\r\n\
                            </html>";
    let fail_response = "\
                            HTTP/1.1 400 NOT_FOUND\r\n\
                            Content-Type: text/html\r\n\
                            \r\n\
                            <!DOCTYPE html>\r\n\
                            <html lang=\"en\">\r\n\
                            <head>\r\n\
                                <meta charset=\"UTF-8\">\r\n\
                                <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\r\n\
                                <title>Rustacean Page</title>\r\n\
                            </head>\r\n\
                            <body>\r\n\
                                <h1>Hello</h1>\r\n\
                                <p>I do not know what you are asking for</p>\r\n\
                            </body>\r\n\
                            </html>";
    let mut buffer = [0; 512]; // [b1, b2, b3..]
    _ = stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1";
    let sleep = b"GET /sleep HTTP/1.1";
    if buffer.starts_with(get) {
        //println!("Request received {}", String::from_utf8_lossy(&buffer[..]));
        println!("==========");

        _ = stream.write(response.as_bytes());
        stream.flush().unwrap();
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        _ = stream.write(response.as_bytes());
        stream.flush().unwrap();
    }
    else {
        _ = stream.write(fail_response.as_bytes());
        stream.flush().unwrap();
    }

    println!("************************************************************************");

}
