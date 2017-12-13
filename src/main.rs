extern crate getopts;

use std::io::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, TcpListener, TcpStream};
use std::io;

fn main() {
	println!("Running rcat");
	let args = std::env::args().collect::<Vec<String>>();
	let program = args[0].clone();

	let mut opts = getopts::Options::new();
	opts.optflag("h", "help", "print usage info");
	opts.optopt("l", "listen", "listen on given port",  "PORT");
	opts.optopt("w", "write", "write from stdin to port",  "PORT");

	let matches = match opts.parse(&args[1..]) {
	   Ok(m) => { m }
	   Err(f) => { panic!(f.to_string()) }
   };

	if matches.opt_present("h") {
		print_usage(&program, opts);
		return;
	}

	if matches.opt_present("l") {
		if let Some(listen_port) = matches.opt_str("l"){
			listen(listen_port.parse().unwrap());
		} else {
			listen(8080);
		}
	} else if matches.opt_present("w") {
		if let Some(write_port) = matches.opt_str("w") {
			write(write_port.parse().unwrap());
			println!("Write port is {:?}", write_port);
		} else {
			write(8080);
		}
	}
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} FILE [options]", program);
    print!("{}", opts.usage(&brief));
}

fn write(port: u16){
	let socket = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port);
	if let Ok(mut stream) = TcpStream::connect(&socket) {
		println!("Connected to server");
		loop {
			let mut input = String::new();
			io::stdin().read_line(&mut input);
			println!("Sent: {}", input.trim());
			stream.write(input.as_bytes());
		}
	} else {
		println!("Couldn't connect");
	}
}

fn listen(port: u16){
	println!("Listening on port {:?}", port);
	let listener = TcpListener::bind(("127.0.0.1", port)).unwrap();

	for stream in listener.incoming() {
		let stream = stream.unwrap();
		handle_connection(stream);
	}
}

fn handle_connection(mut stream: TcpStream){
	loop {
		let mut buffer = [0; 512];
		stream.read(&mut buffer).unwrap();
		println!("Received: {}", String::from_utf8_lossy(&buffer[..]));
	}
}
