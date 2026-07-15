use std::net::TcpListener;
use std::io::{Read, Write};

fn main() {
    // 1. Привязываемся к порту (Bind)
    // Создаем "телефон", который будет лежать на порту 8080
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    println!("🚀 TCP Сервер запущен и слушает порт 8080...");

    // 2. Ждем входящего соединения (Accept)
    // ⚠️ ВАЖНО: Эта строка БЛОКИРУЕТ выполнение программы.
    // Сервер будет стоять и ждать, пока кто-то не постучится.
    let (mut stream, addr) = listener.accept().unwrap();
    println!("✅ Новое подключение от клиента: {}", addr);

    // 3. Читаем данные от клиента
    let mut buffer = [0; 1024];
    // read() тоже блокирующий. Он ждет, пока придут байты.
    let bytes_read = stream.read(&mut buffer).unwrap();

    let received_text = String::from_utf8_lossy(&buffer[..bytes_read]);
    println!("📩 Получено от клиента: {}", received_text);

    // 4. Отправляем ответ
    // Используем write_all, чтобы гарантировать, что все байты уйдут в сеть
    stream.write_all("Привет, клиент!TCP-сервер тебя услышал.".as_bytes()).unwrap();
    println!("📤 Ответ отправлен.");

    // 5. Завершение
    // Когда переменная `stream` выйдет из области видимости (в конце main),
    // Rust автоматически вызовет Drop, который корректно закроет TCP-соединение (FIN).
    println!("🔒 Соединение закрыто.");
}
