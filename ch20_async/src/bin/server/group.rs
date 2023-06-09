use std::sync::Arc;
use async_std::task;
use ch20_async::FromServer;
use tokio::sync::broadcast::{self, error::RecvError};

use crate::connection::Outbound;

pub struct Group {
    name: Arc<String>,
    sender: broadcast::Sender<Arc<String>>,
}

impl Group {
    pub fn new(name: Arc<String>) -> Self {
        let (sender, _) = broadcast::channel(1000);
        Self { name, sender }
    }

    pub fn join(&self, outbound: Arc<Outbound>) {
        let receiver = self.sender.subscribe();
        task::spawn(handle_subscriber(self.name.clone(), receiver, outbound));
    }

    pub fn post(&self, message: Arc<String>) {
        let _ = self.sender.send(message);
    }
}

async fn handle_subscriber(
    group_name: Arc<String>,
    mut receiver: broadcast::Receiver<Arc<String>>,
    outbound: Arc<Outbound>,
) {
    loop{
        let message = match receiver.recv().await {
            Ok(message) => FromServer::Message {
                group_name: group_name.clone(),
                message: message.clone(),
            },
            Err(RecvError::Lagged(n)) => FromServer::Error(format!("{} lagged {} messages", group_name, n)),
            Err(RecvError::Closed) => break,
        };

        
        if outbound.send(message).await.is_err() {
            break;
        }
    }
}