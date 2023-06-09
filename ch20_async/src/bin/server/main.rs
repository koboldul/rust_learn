use std::sync::Arc;

use async_std::stream::StreamExt;
use ch20_async::utils::ChatResult;

use crate::connection::serve;

mod connection;
mod group;
mod group_table;

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: server ADDRESS:PORT");
    let chat_group_table = Arc::new(group_table::GroupTable::new());

    async_std::task::block_on(async{
        use async_std::{net, task};

        let listener = net::TcpListener::bind(address).await?;
        let mut new_connections = listener.incoming();

        while let Some(socket_result) = new_connections.next().await  {
            let socket = socket_result?;
            let groups = chat_group_table.clone();
            task::spawn(async {
                log_error(serve(socket, groups).await);
            });
        }

        Ok(())
    })
}

fn log_error(result: ChatResult<()>) {
    if let Err(error) = result {
        eprint!("Error: {}", error);
    }
}