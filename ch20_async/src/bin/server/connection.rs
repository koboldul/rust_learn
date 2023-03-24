use std::sync::Arc;

use async_std::io::BufReader;
use async_std::prelude::*;
use async_std::{sync::Mutex, net::TcpStream};
use ch20_async::utils::ChatResult;
use ch20_async::{FromServer, utils, FromClient};

use crate::group_table::GroupTable;

pub struct Outbound(Mutex<TcpStream>);

impl Outbound {
    pub fn new(stream: TcpStream) -> Self {
        Self(Mutex::new(stream))
    }

    pub async fn send(&self, message: FromServer) -> ChatResult<()> {
        let mut guard = self.0.lock().await;
        utils::send_as_json(&mut *guard, &message).await?;
        guard.flush().await?;

        Ok(())
    }
}

pub async fn serve(socket: TcpStream, groups: Arc<GroupTable>) -> ChatResult<()> {
    let outbound = Arc::new(Outbound::new(socket.clone()));
    let buffered = BufReader::new(socket);
    let mut from_client = utils::receive_as_json(buffered);

    while let Some(packet) = from_client.next().await {
        let packet = packet?;
        let result = match packet {
            FromClient::Join { group_name } => {
                let group = groups.get_or_create(group_name);
                group.join(outbound.clone());

                Ok(())
            }
            FromClient::Post { group_name, message } => {
                match groups.get(&group_name) {
                    Some(group) => {
                        group.post(message);

                        Ok(())
                    },
                    None => Err("No such group".into()),
                }
            }
        };

        if let Err(message) = result {
            let report = FromServer::Error(message);
            outbound.send(report).await?
        }
    }

    Ok(())
}
