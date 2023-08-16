use tokio::{
    net::{TcpListener, TcpStream},
};

const SERVER_ADDR : &str = "0.0.0.0:1234";

#[derive(Debug, PartialEq, Deserialize)]
struct PrimeRequest{
    method: String,
    number: f64 | i64,
}

#[tokio::main]
async fn main() {
    println!("starting server {}",SERVER_ADDR);
    let listener = TcpListener::bind(SERVER_ADDR).await.unwrap();
    match (socket,x) = listener.accept().await?
}

async fn handle_conn(&mut stream:TcpStream){
    let mut buffer = [0u8;1024];
    loop{
        match stream.read(&mut buffer).await{
            Ok(0) => break,
            Ok(n) =>{
                //log::debug!("read {} bytes",n);
                if let Err(e) = stream.write_all(&buffer[..n]).await{
                    //log::error!("failed to write to socket:{}",e);
                    break;
                }
                //log::debug!("wrote {} bytes",n);
            },
            Err(e) =>{
                    //log::error!("failed to read from socket: {}",e);
                    break;
            }
        };
    }
}