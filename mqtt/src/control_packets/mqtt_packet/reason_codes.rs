/// Connect Reason Code
/// Byte 2 in the Variable Header is the Connect Reason Code.
/// 0 - 0x00 - Success
/// The Connection is accepted.
/// 16 - 0x10 - No matching subscribers
/// No matching subscribers. The Client or Server will not forward the PUBLISH packet.
/// 128 - 0x80 - Unspecified error
/// The Server does not wish to reveal the reason for the failure, or none of the other Reason Codes apply.
/// 129 - 0x81 - Malformed Packet
/// Data within the CONNECT packet could not be correctly parsed.
/// 130 - 0x82 - Protocol Error
/// Data in the CONNECT packet does not conform to this specification.
/// 131 - 0x83 - Implementation specific error
/// The CONNECT is valid but is not accepted by this Server.
/// 132 - 0x84 - Unsupported Protocol Version
/// The Server does not support the version of the MQTT protocol requested by the Client.
/// 133 - 0x85 - Client Identifier not valid
/// The Client Identifier is a valid string but is not allowed by the Server.
/// 134 - 0x86 - Bad User Name or Password
/// The Server does not accept the User Name or Password specified by the Client
/// 135 - 0x87 - Not authorized
/// The Client is not authorized to connect.
/// 136 - 0x88 - Server unavailable
/// The MQTT Server is not available.
/// 137 - 0x89 - Server busy
/// The Server is busy. Try again later.
/// 138 - 0x8A - Banned
/// This Client has been banned by administrative action. Contact the server administrator.
/// 140 - 0x8C - Bad authentication method
/// The authentication method is not supported or does not match the authentication method currently in use.
/// 144 - 0x90 - Topic Name invalid
/// The Will Topic Name is not malformed, but is not accepted by this Server.
/// 145 - 0x91 - Packet Identifier in use
/// The Packet Identifier is already in use. This will only ever be returned for a CONNACK or PUBACK packet.
/// 149 - 0x95 - Packet too large
/// The CONNECT packet exceeded the maximum permissible size.
/// 151 - 0x97 - Quota exceeded
/// An implementation or administrative imposed limit has been exceeded.
/// 153 - 0x99 - Payload format invalid
/// The Will Payload does not match the specified Payload Format Indicator.
/// 154 - 0x9A - Retain not supported
/// The Server does not support retained messages, and Will Retain was set to 1.
/// 155 - 0x9B - QoS not supported
/// The Server does not support the QoS set in Will QoS.
/// 156 - 0x9C - Use another server
/// The Client should temporarily use another server.
/// 157 - 0x9D - Server moved
/// The Client should permanently use another server.
/// 159 - 0x9F - Connection rate exceeded
/// The connection rate limit has been exceeded.

pub enum ReasonMode {
    Success,
    _NoMatchingSubscribers,
    _UnspecifiedError,
    MalformedPacket,
    _ProtocolError,
    _ImplementationSpecificError,
    UnsupportedProtocolVersion,
    ClientIdentifierNotValid,
    _BadUserNameOrPassword,
    _NotAuthorized,
    _ServerUnavailable,
    _ServerBusy,
    _Banned,
    _BadAuthenticationMethod,
    _TopicNameInvalid,
    _PacketIdentifierInUse,
    _PacketTooLarge,
    _QuotaExceeded,
    _PayloadFormatInvalid,
    _RetainNotSupported,
    QoSNotSupported,
    _UseAnotherServer,
    _ServerMoved,
    _ConnectionRateExceeded,
}

impl ReasonMode {
    pub fn get_id(&self) -> u8 {
        match *self {
            ReasonMode::Success => 0,
            ReasonMode::_NoMatchingSubscribers => 16, // PUBACK
            ReasonMode::_UnspecifiedError => 128,     // CONNACK - PUBACK
            ReasonMode::MalformedPacket => 129,
            ReasonMode::_ProtocolError => 130,
            ReasonMode::_ImplementationSpecificError => 131, // CONNACK - PUBACK
            ReasonMode::UnsupportedProtocolVersion => 132,
            ReasonMode::ClientIdentifierNotValid => 133,
            ReasonMode::_BadUserNameOrPassword => 134,
            ReasonMode::_NotAuthorized => 135, // CONNACK - PUBACK
            ReasonMode::_ServerUnavailable => 136,
            ReasonMode::_ServerBusy => 137,
            ReasonMode::_Banned => 138,
            ReasonMode::_BadAuthenticationMethod => 140,
            ReasonMode::_TopicNameInvalid => 144, // CONNACK - PUBACK
            ReasonMode::_PacketIdentifierInUse => 145, // PUBACK
            ReasonMode::_PacketTooLarge => 149,
            ReasonMode::_QuotaExceeded => 151, // CONNACK - PUBACK
            ReasonMode::_PayloadFormatInvalid => 153, // CONNACK - PUBACK
            ReasonMode::_RetainNotSupported => 154,
            ReasonMode::QoSNotSupported => 155,
            ReasonMode::_UseAnotherServer => 156,
            ReasonMode::_ServerMoved => 157,
            ReasonMode::_ConnectionRateExceeded => 159,
        }
    }
}