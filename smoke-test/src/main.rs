
use tokio::{
    net::{TcpListener, TcpStream},
    io::{AsyncReadExt,AsyncWriteExt}
};

const SERVER_ADDR : &str = "0.0.0.0:1234";

#[tokio::main]
async fn main() {
    env_logger::init();
    println!("starting server {}",SERVER_ADDR);
    let listener = TcpListener::bind(SERVER_ADDR).await.unwrap();
    log::info!("Listening on {}", listener.local_addr().unwrap());
    loop {

        match listener.accept().await {
            Err(e) => {
                log::error!("Error accepting connection: {}", &e);
            },
            Ok((stream,addr))=>{
                log::debug!(
                    "Accepted connection from {}", addr
                );
                tokio::spawn(async move{
                    handle_conn(stream).await
                });
            }
        }
        
    }

}

async fn handle_conn(mut stream:TcpStream){
    //read
    let mut buffer = [0u8;1024];
    loop{
        match stream.read(&mut buffer).await{
            Ok(0) => break,
            Ok(n) =>{
                log::debug!("read {} bytes",n);
                if let Err(e) = stream.write_all(&buffer[..n]).await{
                    log::error!("failed to write to socket:{}",e);
                    break;
                }
                log::debug!("wrote {} bytes",n);
            },
            Err(e) =>{
                    log::error!("failed to read from socket: {}",e);
                    break;
            }
        };
    }
}   