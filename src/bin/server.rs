use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::signal;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("🚀 Mini-NATS сервер запущен на порту 8080...");
    println!("💡 Для теста открой другой терминал и введи: nc 127.0.0.1 8080");

    loop {
        tokio::select! {
            res = listener.accept() => {
                let (mut socket, addr) = res?;
                println!("✅ Новое подключение: {}", addr);

                tokio::spawn(async move {
                    let (reader, mut writer) = socket.split();
                    let mut buf_reader = tokio::io::BufReader::new(reader);
                    let mut line = String::new();
                    loop {
                        line.clear();

                        let bytes_read = match buf_reader.read_line(&mut line).await {
                            Ok(n) => n,
                            Err(e) => {
                                eprintln!("⚠️ Ошибка чтения от {}: {}", addr, e);
                                break;
                            }
                        };

                        if bytes_read == 0 {
                            println!("🔒 Клиент {} отключился", addr);
                            break;
                        }

                        let command = line.trim();
                        println!("📩 От {}: '{}'", addr, command);

                        if command == "PING" {
                            if let Err(e) = writer.write_all(b"PONG\r\n").await {
                                eprintln!("⚠️ Ошибка записи {}: {}", addr, e);
                                break;
                            }
                        } else {
                            println!("⚠️ Неизвестная команда от {}", addr);
                        }
                    }
                });
            }
            _ = signal::ctrl_c() => {
                println!("\n❌ Завершение по Ctrl+C...");
                break;
            }
        }
    }
    println!("🔒 Сервер остановлен.");
    Ok(())
}
