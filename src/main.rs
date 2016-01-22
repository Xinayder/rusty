use std::io::prelude::*;
use std::io::BufReader;
use std::net::TcpStream;
use std::str::FromStr;

struct IrcMessage {
    raw_message: String,
    prefix: String,
    command: String,
    params: Vec<String>
}

impl IrcMessage {
    fn new(raw_msg: &str) -> IrcMessage {
        let mut raw: &str = raw_msg;
        let mut new_msg = IrcMessage {
            raw_message: String::new(),
            prefix: String::new(),
            command: String::new(),
            params: Vec::with_capacity(15)
        };

        new_msg.raw_message = String::from_str(raw).unwrap();
        if raw.starts_with(":") {
            let first_whitespace: u32 = match raw.find(' ') {
                Some(x) => x as u32,
                None => 0u32
            };
            new_msg.prefix = String::from_str(raw.substr(1, first_whitespace - 1)).unwrap();
            raw = raw.substr(first_whitespace + 1, raw.len() as u32 - (first_whitespace+1));
        }
        
        if raw.contains(' ') {
            let space_index = match raw.find(' ') {
                Some(x) => x as u32,
                None => 0u32,
            };
            new_msg.command = String::from(raw.substr(0, space_index));
            raw = raw.substr(space_index + 1, raw.len() as u32 - (space_index + 1));

            // Parse parameters
            let mut parameters: Vec<String> = Vec::new();
            while raw != "" {
                if raw.starts_with(":") { 
                    parameters.push(String::from(raw.substr(1, raw.len() as u32 - 1)));
                    break;
                }

                if !raw.contains(' ') {
                    parameters.push(String::from(raw));
                    raw = "";
                    break;
                }
                let space_index = match raw.find(' ') {
                    Some(x) => x as u32,
                    None => 0u32
                };
                parameters.push(String::from(raw.substr(0, space_index)));
                raw = raw.substr(space_index + 1, raw.len() as u32 - (space_index + 1));
            }
            new_msg.params = parameters;
        }

        return new_msg;
    }
}

trait Substring {
    fn substr(&self, start_index: u32, length: u32) -> &str;
}

impl Substring for str {
    fn substr(&self, start_index: u32, length: u32) -> &str {
        return &self[start_index as usize .. start_index as usize + length as usize];
    }
}

fn main() {
    let mut stream = TcpStream::connect("irc.esper.net:6667").unwrap();
    loop {
        let reader = BufReader::new(&mut stream);
        for l in reader.lines() {
            match l {
                Ok(x) => {
                    println!("{}", x);
                    parse_response(&x);
                },
                Err(_) => panic!("Failed to read line"),
            };
        }
    }
}

fn parse_response(buffer: &str) {
    // sample message:
    // :availo.esper.net 401 test_nick :No such nick/channel
    let mut msg = buffer;
    if msg != "" {
        msg = msg.trim();
        let message = IrcMessage::new(msg);
        println!(">> {}", message.raw_message);

        /*if message.command == "PING" { 
            let mut ping_msg = String::from("PONG :");
            ping_msg.push_str(&message.params[0]);
            send_raw_message(stream, &ping_msg); 
        }*/
    }
}
