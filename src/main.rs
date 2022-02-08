use std::net::TcpListener;
use zero2prod::run;
use webbrowser;

// use webbrowser::{open_browser, Browser};
// 

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:0").unwrap();
    let url = listener.local_addr().unwrap();
    // println!("{:?}",listener.local_addr().unwrap());
    webbrowser::open(&format!("http://{}",url)).unwrap();
    // Bubble up the io::Error if we failed to bind the address
    // Otherwise call .await on our Server
    run(listener)?.await
    
}
