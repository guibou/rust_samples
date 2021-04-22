use std::io::Write;
use std::io::Read;

pub fn handle_client(mut stream: std::net::TcpStream) -> ! {
    let mut i = 0;
    let mut last_buffer: Vec<u8> = Vec::new();

    loop {
        let mut buffer = [0; 10];
        let r = stream.read(&mut buffer).unwrap();
        last_buffer.extend_from_slice(&buffer[0..r]);

        let mut last_idx = 0;
        (0..(last_buffer.len())).for_each(|current_idx| {
            if last_buffer[current_idx] == 10 || last_buffer[current_idx] == 32 {
                let b = &mut last_buffer[last_idx..current_idx];
                b.reverse();
                stream.write(b).unwrap();

                i += 1;
                stream.write(b" ").unwrap();
                last_idx = current_idx + 1;
            }
        });

        last_buffer = last_buffer.split_off(last_idx);
    }
}

fn main() -> std::io::Result<()> {
    println!("Hello, world!");
    let listener = std::net::TcpListener::bind("127.0.0.1:8080")?;

    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}
