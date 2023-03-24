use async_std::{sync::Mutex, net::TcpStream};
use ch20_async::{FromServer, utils};

pub struct Outbound(Mutex<TcpStream>);

impl Outbound {
    pub fn new(stream: TcpStream) -> Self {
        Self(Mutex::new(stream))
    }

    pub async fn send(&self, message: FromServer) -> ChatResult<()> {
        let mut stream = self.0.lock().await;
        utils::send_as_json(&mut stream, &message).await?;
        guard.flush().await?;
        
        Ok(())
    }
}
    
}