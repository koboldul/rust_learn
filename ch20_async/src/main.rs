use std::sync::Arc;

use async_std::io::prelude::*;
use async_std::net;
use async_std::task;

async fn ch_request(host: Arc<String>, port: u16, path: Arc<String>) -> std::io::Result<String> {
    let mut socket = net::TcpStream::connect(((*host).as_str(), port)).await?;
    let request =
        format!("GET {} HTTP/1.1\r\nHost: {}\r\n\r\n", 
        path, host);
    socket.write_all(request.as_bytes()).await?;
    let _ = socket.shutdown(net::Shutdown::Write);
    let mut response = String::new();
    socket.read_to_string(&mut response).await?;
    println!("Done with {}", path);

    Ok(response)
}

async fn multiple_requests(requests: Vec<(String, u16, String)>) -> 
    Vec<std::io::Result<String>>
{
        let mut futures = vec![];

        for (host, port, path) in requests {
            let a_host = Arc::new(host);
            let a_path = Arc::new(path);
            futures.push(task::spawn_local(ch_request(a_host.clone(), port, a_path.clone())));
        }

        let mut results = vec![];

        for f in futures {
            results.push(f.await);
            println!(" Awaited");
        }

        results
}

fn main() -> std::io::Result<()> {
    let requests = vec![
        ("www.rust-lang.org".to_string(), 80, "/".to_string()),
        ("www.rust-lang.org".to_string(), 80, "/en-US/".to_string()),
        ("www.rust-lang.org".to_string(), 80, "/en-US/install.html".to_string()),
    ];

    let response = task::block_on(multiple_requests(requests));
    for _ in response {
        println!("Dune");
    }

    Ok(())
}