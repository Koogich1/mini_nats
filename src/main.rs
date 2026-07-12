use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    // Мы просто создаем сокет. Никакого handshake!
    let socket = UdpSocket::bind("127.0.0.1:34254")?;

    // Мы отправляем пакет. Просто шлепаем IP-заголовок и летит.
    // Мы даже не знаем, дошел ли он! (Если только не ждем ответ вручную).
    socket.send_to("Привет, это UDP!".as_bytes(), "127.0.0.1:8080")?;

    // Ждем ответ (а вдруг он придет, а может и нет).
    let mut buf = [0; 1024];
    let (amt, src) = socket.recv_from(&mut buf)?;

    println!("Получено {} байт от {}", amt, src);
    Ok(())
}
