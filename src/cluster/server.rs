use cluster::client::ClusterClient;
use std::net::{SocketAddr, TcpListener};
use std::sync::{Arc, RwLock};
use std::thread;

use super::manager::Manager;
pub fn listen(addr: SocketAddr, connection_manager: Arc<RwLock<Manager>>) {
    info!("Initializing Cluster server...");
    let listener = TcpListener::bind(addr).unwrap();
    for stream in listener.incoming() {
        info!("New Node connected!");
        let stream = stream.unwrap();
        thread::spawn(move || {
            let mut client = ClusterClient::new(stream);
            client.run();
        });
    }
}
