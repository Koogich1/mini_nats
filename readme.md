Mini NATS — свой message broker
Своя версия NATS.io или Kafka-lite: клиенты подключаются по TCP, подписываются на каналы, публикуют сообщения, все подписчики получают их в real-time.
Что прокачаешь:
TCP server на tokio
Свой сетевой протокол (байтовый)
Pub/Sub и Fanout через channels (tokio::sync::broadcast, mpsc)
Worker pool и graceful shutdown
Persistence (append-only log)
Admin CLI и метрики
Итерации:
1) TCP listener + парсинг строк (PING/PONG)
2) Команды PUB topic data, SUB topic через in-memory map
3) Persistence: писать все сообщения в файл (AOF как в Redis)
4) Consumer groups (как в Kafka — один worker получает один мессадж из группы)
5) Metrics endpoint (/metrics) + structured logs через tracing
6) Client crate — чтобы подключаться к брокеру из других проектов
7) TLS + Auth
