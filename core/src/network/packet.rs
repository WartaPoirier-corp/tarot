use serde::{Deserialize, Serialize};

/// Serverbound packet (client → server)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PacketSb {
    Ping,
    Join,
    // TODO ...
}

/// Clientbound packet (server → client)
#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum PacketCb {
    PingAnswer(String),
    Accept,
    // TODO ...
}
