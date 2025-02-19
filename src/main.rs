use std::{io::{self, stdin, stdout, Write}, net::UdpSocket};

fn main() {
    println!("Log to network; each message you enter will be broadcast on the network");
    println!("The use case of this program is network-analysis where the network capture");
    println!("is easier to understand if it contains messages explaining what is happening.");
    println!("");

    loop {
        let mut buffer = String::new();

        print!("> ");
        stdout().flush().unwrap();
        // `read_line` returns `Result` of bytes read
        stdin().read_line(&mut buffer).unwrap();
        match buffer.trim_end() {
            "q" => {
                return;
            }
            status => {
                if let Err(e) = broadcast_message(status) {
                    println!("Oops: {e}");
                }
            }
        };
    }
}

fn broadcast_message(s: &str) -> io::Result<()> {
    let socket:UdpSocket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    socket.connect("255.255.255.255:9999")?;
    socket.send(s.as_bytes())?;
    Ok(())
}