use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    println!("📞 Звоним на сервер...");

    let mut stream = TcpStream::connect("127.0.0.1:8080").await?;
    println!("✅ Соединение установлено!");

    stream.write_all("Привет, сервер! Это TCP клиент.".as_bytes()).await?;
    println!("📤 Сообщение отправлено.");

    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer).await?;

    let response = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("📩 Ответ сервера: {}", response);

    Ok(())
}
