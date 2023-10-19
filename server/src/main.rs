//use std::io::{self, Read, Write, BufRead};
use std::net::UdpSocket;
//use std::env;
//use std::str;

fn main() -> std::io::Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:2000")?; // for UDP4
    let mut buf = [0; 2048];

    loop {
        // Receives a single datagram message on the socket.
	// If `buf` is too small to hold
        // the message, it will be cut off.
        let (amt, src) = socket.recv_from(&mut buf)?;

        // Redeclare `buf` as slice of the received data
	// and send data back to origin.
        let buf = &mut buf[..amt];
        socket.send_to(buf, &src)?;
    }
}