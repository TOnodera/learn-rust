use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let server_address = "127.0.0.1:8888";
    let (tx, rx) = mpsc::channel::<String>();
    let mut clients: Vec<TcpStream> = Vec::new();

    let server = TcpListener::bind(server_address).expect("サーバーの起動に失敗しました。");
    server.set_nonblocking(true).expect("利用できません。");
    println!("{}でサーバーを起動しました。", server_address);

    loop {
        if let Ok((client, addr)) = server.accept() {
            println!("クライアントが接続:{}", addr);
            clients.push(client.try_clone().unwrap());
            start_thread(client, tx.clone());
        }

        if let Ok(msg) = rx.try_recv() {
            println!("全員に送信: {}", msg.trim());
            clients = send_all(clients, &msg);
        }
        thread::sleep(Duration::from_millis(100));
    }
}

fn start_thread(client: TcpStream, tx: mpsc::Sender<String>) {
    let mut reader = BufReader::new(client);
    thread::spawn(move || loop {
        let mut msg = String::new();
        if let Ok(n) = reader.read_line(&mut msg) {
            if n > 0 {
                tx.send(msg).unwrap();
            }
        }
    });
}

fn send_all(clients: Vec<TcpStream>, s: &str) -> Vec<TcpStream> {
    let mut collector = vec![];
    for mut socket in clients.into_iter() {
        let bytes = String::from(s).into_bytes();
        if let Err(e) = socket.write_all(&bytes) {
            println!("送信エラー: {}", e);
            continue;
        }
        collector.push(socket);
    }
    collector
}
