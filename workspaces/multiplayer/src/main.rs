use std::collections::HashMap;

/// Fake ClientId (just u64 for now)
type ClientId = u64;

// Socket Consists of: 
// IP address, IPV4 is 4 bytes 32-bits, IPV6 is 16 bytes
// 16 bit port number
// 
// pub enum SocketAddr {
//     V4(#[stable(feature = "rust1", since = "1.0.0")] SocketAddrV4),
// }

/// A minimal fake Endpoint — just enough to test creation & client handling
pub struct Endpoint {
    clients: HashMap<ClientId, String>, // In real implementation this would be a PeerConnection
    next_client_id: ClientId,
}

impl Endpoint {
    pub fn new() -> Self {
        Self {
            clients: HashMap::new(),
            next_client_id: 0,
        }
    }

    /// Return current connected clients
    pub fn clients(&self) -> Vec<ClientId> {
        self.clients.keys().cloned().collect()
    }

    /// Fake adding a new client connection
    pub fn accept_client(&mut self, name: String) -> ClientId {
        self.next_client_id += 1;
        let id = self.next_client_id;
        self.clients.insert(id, name);
        id
    }

    /// Disconnect a client
    pub fn disconnect(&mut self, id: ClientId) -> bool {
        self.clients.remove(&id).is_some()
    }
}

//
// ─── DRIVER PROGRAM ─────────────────────────────────────────────────────────────
//
fn main() {
    let mut endpoint = Endpoint::new();
    let c1 = endpoint.accept_client("Alice".into());
    let c2 = endpoint.accept_client("Bob".into());

    println!("Server has clients: {:?}", endpoint.clients());
    endpoint.disconnect(c1);
    println!("After disconnect: {:?}", endpoint.clients());
}

//
// ─── BASIC TESTS ────────────────────────────────────────────────────────────────
//
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn endpoint_can_be_created() {
        let endpoint = Endpoint::new();
        assert_eq!(endpoint.clients().len(), 0);
    }

    #[test]
    fn client_can_connect() {
        let mut endpoint = Endpoint::new();
        let id = endpoint.accept_client("TestClient".into());
        let clients = endpoint.clients();
        assert_eq!(clients.len(), 1);
        assert_eq!(clients[0], id);
    }

    #[test]
    fn client_can_disconnect() {
        let mut endpoint = Endpoint::new();
        let id = endpoint.accept_client("TestClient".into());
        assert!(endpoint.disconnect(id));
        assert!(endpoint.clients().is_empty());
    }
}
