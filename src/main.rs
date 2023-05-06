use std::process::Command;
use std::string::String;
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("127.0.0.1:8000")?;
    loop {
        let mut buf = [0; 1024];
        match socket.recv_from(&mut buf){
			Ok((sz, _srz)) =>{
		        let bu: &Vec<u8> = &buf[..sz].to_vec();
                let s = String::from_utf8((&bu).to_vec()).unwrap();
                let v = s.trim();
                let args: Vec<&str> = v.split_whitespace().collect();
                let _cmd = Command::new(args[0]).args(&args[1..]).output();
			}
			Err(_) =>{}
        }
    }
}
