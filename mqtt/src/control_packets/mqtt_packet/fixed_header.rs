pub struct PacketFixedHeader {
    pub packet_type: u8,
    pub remaining_length: u8, // This is the length of the Variable Header plus the length of the Payload. It is encoded as a Variable Byte Integer.
}

impl PacketFixedHeader {
    pub fn new(type_and_flags: u8, remaining_len: u8) -> Self {
        PacketFixedHeader {
            packet_type: type_and_flags,
            remaining_length: remaining_len,
        }
    }
}
