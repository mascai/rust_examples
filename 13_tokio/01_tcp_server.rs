use tokio::io::{AsyncWriteExt, AsyncReadExt};
use tokio::net::{TcpListener, TcpStream, tcp};
use std::sync::{Arc, Mutex, MutexGuard};


#[tokio::main]
async fn main() {
    let balance = Arc::new(Mutex::new(0.0f32));
    let listener = TcpListener::bind("127.0.0.1:8008").await.unwrap();
    loop {
        let balance = balance.clone();
        let (tcp_stream, sock_addr) = listener.accept().await.unwrap();
        tokio::spawn(async move {
            handle_connection(tcp_stream, balance).await;
        });
    }
}


async fn handle_connection(mut stream: TcpStream, balance: Arc<Mutex<f32>>) {
    // Read the first 16 characters from the incoming stream.
    let mut buffer = [0; 16];
    stream.read(&mut buffer).await.unwrap();
    // First 4 characters are used to detect HTTP method
    let method_type = match std::str::from_utf8(&buffer[0..4]) {
        Ok(v) => v,
        Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    };

    let contents = match method_type {
        "GET " => {
            // todo: return real balance
            format!("{{\"balance\": {}}}", balance.lock().unwrap())
        }
        "POST" => {
            // Take characters after 'POST /' until whitespace is detected.
            let input: String = buffer[6..16]
                .iter()
                .take_while(|x| **x != 32u8)
                .map(|x| *x as char)
                .collect();
            let balance_update = input.parse::<f32>().unwrap();
            let mut locked_balance: MutexGuard<f32> = balance.lock().unwrap();
            *locked_balance += balance_update;
            // todo: add balance update handling
            format!("{{\"balance\": {}}}", locked_balance)
        }
        _ => {
            panic!("Invalid HTTP method!")
        }
    };

    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
        contents.len(),
        contents
    );
    stream.write(response.as_bytes()).await.unwrap();
    stream.flush().await.unwrap();
}
