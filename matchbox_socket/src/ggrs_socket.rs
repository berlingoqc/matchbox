use ggrs::{Message, PlayerType};

use std::net::SocketAddr;

use crate::WebRtcSocket;

impl WebRtcSocket {
    #[must_use]
    pub fn players(&self) -> Vec<PlayerType<SocketAddr>> {
        // needs to be consistent order across all peers
        let mut ids = self.connected_peers();
        ids.push(self.id().to_owned());
        ids.sort();
        ids.iter()
            .map(|id| {
                if id == self.id() {
                    PlayerType::Local
                } else {
                    let parse_ip: SocketAddr = id.to_owned().parse().unwrap();
                    PlayerType::Remote(parse_ip)
                }
            })
            .collect()
    }
}

impl ggrs::NonBlockingSocket<SocketAddr> for WebRtcSocket {
    fn send_to(&mut self, msg: &Message, addr: &SocketAddr) {
        let buf = bincode::serialize(&msg).unwrap();
        let packet = buf.into_boxed_slice();
        self.send(packet, addr.to_string());
    }

    fn receive_all_messages(&mut self) -> Vec<(SocketAddr, Message)> {
        // let fake_socket_addrs = self.fake_socket_addrs.clone();
        let mut messages = vec![];
        for (id, packet) in self.receive().into_iter() {
            let msg = bincode::deserialize(&packet).unwrap();
            let id: SocketAddr = id.parse().unwrap();
            messages.push((id, msg));
        }
        messages
    }
}
