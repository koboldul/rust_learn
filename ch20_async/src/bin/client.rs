use std::sync::Arc;

use async_std::prelude::*;
use async_std::{net, io};
use ch20_async::{FromServer, FromClient};
use ch20_async::utils::{self, ChatResult};
use async_std::task;

fn main() -> ChatResult<()> {
    let address = std::env::args().nth(1).expect("Usage: client ADDRESS:PORT");
    task::block_on(async {
        let mut socket = net::TcpStream::connect(address).await?;
        socket.set_nodelay(true)?;

        let to_server = send_commands(socket.clone());
        let from_server = handle_replies(socket);

        from_server.race(to_server).await?;

        Ok(())
    })
}

async fn send_commands(mut to_server: net::TcpStream) -> ChatResult<()> {
    println!("Commands:\n\
              join GROUP\n\
              post GROUP MESSAGE...\n\
              Type Control-D (on Unix) or Control-Z (on Windows) \
              to close the connection.");

    let mut commdan_lines = io::BufReader::new(io::stdin()).lines();

    while let Some(line) = commdan_lines.next().await {
        let cmd = line?;
        let request = match parse_command(&cmd) {
             Some(request) => request,
             None => continue,   
        };

        utils::send_as_json(&mut to_server, &request).await?;
        to_server.flush().await?;
    }

    Ok(())
}

/// Parse a line (presumably read from the standard input) as a `Request`.
fn parse_command(line: &str) -> Option<FromClient> {
    let (command, rest) = get_next_token(line)?;
    if command == "post" {
        let (group, rest) = get_next_token(rest)?;
        let message = rest.trim_start().to_string();
        return Some(FromClient::Post {
            group_name: Arc::new(group.to_string()),
            message: Arc::new(message),
        });
    } else if command == "join" {
        let (group, rest) = get_next_token(rest)?;
        if !rest.trim_start().is_empty() {
            return None;
        }
        return Some(FromClient::Join {
            group_name: Arc::new(group.to_string()),
        });
    } else {
        eprintln!("Unrecognized command: {:?}", line);
        return None;
    }
}

/// Given a string `input`, return `Some((token, rest))`, where `token` is the
/// first run of non-whitespace characters in `input`, and `rest` is the rest of
/// the string. If the string contains no non-whitespace characters, return
/// `None`.
fn get_next_token(mut input: &str) -> Option<(&str, &str)> {
    input = input.trim_start();

    if input.is_empty() {
        return None;
    }

    match input.find(char::is_whitespace) {
        Some(space) => Some((&input[0..space], &input[space..])),
        None => Some((input, "")),
    }
}

async fn handle_replies(from_server: net::TcpStream) -> ChatResult<()> {
    let buffered = io::BufReader::new(from_server);
    let mut reply_stream = utils::receive_as_json(buffered);
    
    while let Some(reply) = reply_stream.next().await {
        match reply? {
            FromServer::Message { group_name, mesaage } => {
                println!("Message posted {}: {}", group_name, mesaage);
            },
            FromServer::Error(error) => {
                println!("Error: {}", error);
            },
        }
    }
    
    Ok(())
}
