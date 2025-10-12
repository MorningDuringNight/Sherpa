use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug, Clone)]
pub struct HandshakeData {
    pub tick: u32,
    pub player_number: u8,
    pub packet_number: u8,
    pub character_selection: String,
    pub player_name: String,
}

#[derive(Debug, Clone)]
pub struct HandshakeResponse {
    pub label: String,        // "ACK"
    pub player_number: u8,
    pub packet_number: u8,
}

impl HandshakeResponse {
    /// Create a new ACK (server only)
    pub fn new(player_number: u8, packet_number: u8) -> Self {

        Self {
            label: "ACK".to_string(),
            player_number,
            packet_number,
        }
    }

    // Encode into a human-readable string for sending over UDP
    // Example: "ACK|1|3|1733890041234"
    pub fn encode(&self) -> Vec<u8> {
        let msg = format!(
            "{}|{}|{}",
            self.label, self.player_number, self.packet_number,
        );
        msg.into_bytes()
    }

    /// Decode from a UTF-8 packet string
    pub fn decode(data: &[u8]) -> Option<Self> {
        let msg = String::from_utf8_lossy(data);
        let parts: Vec<&str> = msg.split('|').collect();
        if parts.len() != 3 || parts[0] != "ACK" {
            return None;
        }

        Some(Self {
            label: parts[0].to_string(),
            player_number: parts[1].parse().ok()?,
            packet_number: parts[2].parse().ok()?,
        })
    }
}

impl HandshakeData {
    pub fn new(
        packet_number: u8,
        tick: u32,
        player_number: u8,
        character_selection: &str,
        player_name: &str,
    ) -> Self {
        Self {
            tick,
            player_number,
            packet_number,
            character_selection: character_selection.to_string(),
            player_name: player_name.to_string(),
        }
    }

    /// Encode into compact byte vector
    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::new();

        // tick (4 bytes)
        buf.extend_from_slice(&self.tick.to_be_bytes());

        // player_number (1 byte)
        buf.push(self.player_number);

        // packet_number (1 byte)
        buf.push(self.packet_number);

        // character_selection (1 byte len + bytes)
        let char_bytes = self.character_selection.as_bytes();
        buf.push(char_bytes.len() as u8);
        buf.extend_from_slice(char_bytes);

        // player_name (1 byte len + bytes)
        let name_bytes = self.player_name.as_bytes();
        buf.push(name_bytes.len() as u8);
        buf.extend_from_slice(name_bytes);

        buf
    }

    /// Decode from bytes
    pub fn decode(data: &[u8]) -> Option<Self> {
        if data.len() < 6 {
            return None;
        }

        let tick = u32::from_be_bytes(data[0..4].try_into().ok()?);
        let player_number = data[4];
        let packet_number = data[5];

        let mut cursor = 6;

        // character_selection
        if cursor >= data.len() {
            return None;
        }
        let char_len = data[cursor] as usize;
        cursor += 1;
        if cursor + char_len > data.len() {
            return None;
        }
        let character_selection =
            String::from_utf8_lossy(&data[cursor..cursor + char_len]).to_string();
        cursor += char_len;

        // player_name
        if cursor >= data.len() {
            return None;
        }
        let name_len = data[cursor] as usize;
        cursor += 1;
        if cursor + name_len > data.len() {
            return None;
        }
        let player_name =
            String::from_utf8_lossy(&data[cursor..cursor + name_len]).to_string();

        Some(Self {
            tick,
            player_number,
            packet_number,
            character_selection,
            player_name,
        })
    }
}
