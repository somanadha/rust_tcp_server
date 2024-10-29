use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

const HTTP_REQUEST: &str = "GET / HTTP/1.1";
const HTTP_RESPONSE_OK_200: &str = "HTTP/1.1 200 OK";
const HTTP_SLEEP_REQUEST: &str = "GET /sleep HTTP/1.1";
const HTTP_RESPONSE_NOT_FOUND_404: &str = "HTTP/1.1 404 NOT FOUND";

const HTTP_CONTENT_LENGTH_STRING: &str = "Content-Length: "; // One empty blank space after the "Content-Length:" text is intentional:
const CR_LF: &str = "\r\n";

const FILE_INDEX_HTML: &str = "index.html";
const FILE_404_HTML: &str = "404.html";

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in tcp_listener.incoming() {
        let stream = stream.unwrap();

        //println!("Connection established!");
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|one_line_result| one_line_result.unwrap())
        .take_while(|one_line| !one_line.is_empty())
        .collect();
    //println!("Request: {http_request:#?}");

    let request_header_line = &http_request.get(0).unwrap()[..];
    //println!("request_header_line: {request_header_line:#?}");

    let (response_header_line, file_name) = match request_header_line {
        HTTP_REQUEST => (HTTP_RESPONSE_OK_200, FILE_INDEX_HTML),
        HTTP_SLEEP_REQUEST => {
            thread::sleep(Duration::from_secs(5));
            (HTTP_RESPONSE_OK_200, FILE_INDEX_HTML)
        }
        _ => (HTTP_RESPONSE_NOT_FOUND_404, FILE_404_HTML),
    };

    let contents = fs::read_to_string(file_name).unwrap();
    let length = contents.len();

    let response = format!(
        "{response_header_line}{CR_LF}{HTTP_CONTENT_LENGTH_STRING}{length}{CR_LF}{CR_LF}{contents}"
    );

    stream.write_all(response.as_bytes()).unwrap();
}
