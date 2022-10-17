use tiny_http::{Server, Response};
use serde_json::{Value, from_str};

fn main() {

    let server = Server::http("127.0.0.1:3000").unwrap();

    loop {
        let rq = match server.recv() {
            Ok(mut r) => {
                let mut content = String::new();
                r.as_reader().read_to_string(&mut content).unwrap();
                let parsed: Value = from_str(&content).unwrap();
                println!("{}", parsed["amogus"]);
            },
            Err(e) => { println!("error: {}", e); continue;}
        };
    }
}