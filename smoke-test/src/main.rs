use tokio::{
    net::{TcpListener, TcpStream},
    io::{AsyncReadExt,AsyncWriteExt}
};

const SERVER_ADDR : &str = "127.0.0.1:1234";

#[tokio::main]
async fn main() {
    println!("starting server {}",SERVER_ADDR);
    let listener = TcpListener::bind(SERVER_ADDR).await.unwrap();
    loop {
        let ( mut stream, _ ) = listener.accept().await.unwrap();
        tokio::spawn(async move{
            handle_conn(stream).await;
        });
    }
}

async fn handle_conn(mut stream:TcpStream){
    //read
    let mut buffer = [0;1024];
    let len = stream.read(&mut buffer).await.unwrap();
    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("Received: {}",message);

    //write
    let _ = stream.write_all(&buffer[..len]).await;
    println!("sent {}",message);
}   