pub mod client;
pub mod manager;
pub mod server;

type NodeAlias = String;
type NodeIP = String;
type NodePort = String;
type NodeInfo = (NodeAlias, NodeIP, NodePort);
