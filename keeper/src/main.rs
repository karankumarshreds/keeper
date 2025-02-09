use std::io::prelude::*;
use std::net;
use std::net::TcpStream;
use std::result::Result;

mod config;

struct UpstreamState<'a> {
    index: usize,
    config: &'a config::RootConfig,
}

/* keeper */
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root_config = config::load().expect("Should load config");
    let mut upstream_state = UpstreamState::new(&root_config);
    println!(
        "Root config: port {:?}, servers: {:#?}",
        root_config.port, root_config.servers
    );

    let listener = net::TcpListener::bind("127.0.0.1:80")?;
    println!("Listening on 127.0.0.1:80");
    for stream in listener.incoming() {
        let upstream = upstream_state.decide_upstream()?;
        let nbytes = handle_stream(stream?, upstream)?;
        println!("Send bytes: {nbytes}");
    }
    Ok(())
}

impl UpstreamState<'_> {
    fn new<'a>(config: &'a config::RootConfig) -> UpstreamState<'a> {
        UpstreamState { index: 0, config }
    }

    fn decide_upstream(&mut self) -> Result<TcpStream, Box<dyn std::error::Error>> {
        let addr = format!(
            "{}:{}",
            self.config.servers[self.index].address, self.config.servers[self.index].port
        );
        let upstream = TcpStream::connect(addr)?;
        Ok(upstream)
    }
}

// Sends to the upstream server and returns the number of bytes sent
fn handle_stream(
    mut stream: TcpStream,
    mut upstream: TcpStream,
) -> Result<usize, Box<dyn std::error::Error>> {
    let mut buf = [0; 128];
    let nbytes = stream.read(&mut buf)?; // bytes read
    println!("Recieved bytes: {nbytes}");
    let nbytes = upstream
        .write(&buf[..nbytes])
        .expect("Should write to upstream");
    Ok(nbytes)
}
