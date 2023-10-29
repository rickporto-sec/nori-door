// netcat ! ```nc -lup 5000``` !
// svudp
// rickporto_
use tokio::net::UdpSocket;
use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    let sock = UdpSocket::bind("0.0.0.0:5001").await?;
    let mut buf = [0; 1024];
    let addr = "0.0.0.0:5000";
    loop {
        let out = "out!\n";
        buf[..out.len()].copy_from_slice(out.as_bytes());
        let len = sock.send_to(&buf[..out.len()], addr).await?;
        println!("{:?} bytes", len);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

