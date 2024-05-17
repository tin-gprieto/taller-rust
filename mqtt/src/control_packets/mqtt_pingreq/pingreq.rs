use std::io::{Error, Read, Write};

use crate::control_packets::mqtt_packet::{
    fixed_header::{PacketFixedHeader, PINGREQ_PACKET},
    packet::generic_packet::{PacketReceived, Serialization},
};

#[allow(dead_code)]
pub struct PingReq {}

impl Serialization for PingReq {
    fn read_from(_stream: &mut dyn Read, remaining_length: u16) -> Result<Self, Error> {
        if remaining_length != 0 {
            return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                "PingReq packet must have remaining length 0",
            ));
        }

        Ok(PingReq::new())
    }

    fn write_to(&self, stream: &mut dyn Write) -> Result<(), Error> {
        let fixed_header = PacketFixedHeader::new(PINGREQ_PACKET, 0);
        let fixed_header_bytes = fixed_header.as_bytes();
        stream.write_all(&fixed_header_bytes)?;

        Ok(())
    }

    fn packed_package(package: PingReq) -> PacketReceived {
        PacketReceived::PingReq(Box::new(package))
    }
}

impl PingReq {
    #[allow(dead_code)]
    pub fn new() -> Self {
        PingReq {}
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn pingreq() {
        let pingreq = PingReq::new();

        let mut buffer = Vec::new();
        pingreq.write_to(&mut buffer).unwrap();

        let mut buffer = buffer.as_slice();
        let pingreq_fixed_header = PacketFixedHeader::read_from(&mut buffer).unwrap();

        assert_eq!(pingreq_fixed_header.get_packet_type(), PINGREQ_PACKET);
    }
}
