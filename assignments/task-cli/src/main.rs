/*  You may start from this template when implementing Task 1,
    or use entirely own code.
 */

use std::{
    io::{Read, Write},
    net::TcpStream,
    time::Instant,
    error::Error,
};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Task-CLI starting");


    /* TODO:
        - Open TCP connection to adnet-agent server
        - Write command message to socket: "TASK-001 keyword"
        - Read all data incoming to the socket until the other end closes TCP connection
        - Pay attention to total bytes read, and the last 8 characters read
     */
     
     // setting target IP address and the secret message
     let target_ip = "10.0.0.3:12345";
     let message = "TASK-CLI gorilla";

     // Opening TCP connection
     let mut socket = TcpStream::connect(&target_ip)?;

    // Start clock to measure the time it takes do finish transmission
    let start = Instant::now();

    // Writing the message to the socket
    let n = socket.write(message.as_bytes())?;

    // Checking that the whole message was delivered
     if n < message.len() {
        println!("Some data is missing");
     }
    
    // Variable to store all bytes
    let mut total: usize = 0;
     // Variable to store last 8 characters
    let mut last8: Vec<u8> = Vec::new();

     // Buffer where data is read
     let mut buf: [u8; 10000] = [0; 10000];

     // Looping over the received data, breaking when no more incoming data
     loop {
        // Reading data to buffer
        let m = socket.read(&mut buf)?;
        if m == 0 {
            println!("Connection closed, exiting.");
            break;
        }
        // adding data to total
        total += m;

        // Looping over data in buffer and keeping track of the last 8 characters
        for &b in &buf[..m] {
            if last8.len() == 8 {
                last8.remove(0);
            }
            last8.push(b);
        }
     }

    let duration = start.elapsed();
    
 println!("Total size: {} -- Last bytes: {} -- Duration: {:?}", total, std::str::from_utf8(&last8).unwrap(), duration);

    Ok(())
}