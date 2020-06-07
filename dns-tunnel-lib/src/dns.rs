
const ID_SIZE: usize = 2;
const ID_POS: usize = 0;

const FLAGS_SIZE: usize = 2;
const FLAGS_POS: usize = 2;

const QDCOUNT_SIZE: usize = 2;
const QDCOUNT_POS: usize = 4;

const ANCOUNT_SIZE: usize = 2;
const ANCOUNT_POS: usize = 6;

const NSCOUNT_SIZE: usize = 2;
const NSCOUNT_POS: usize = 8;

const ARCOUNT_SIZE: usize = 2;
const ARCOUNT_POS: usize = 10;

const DNS_HEADER_SIZE: usize =
    ID_SIZE + FLAGS_SIZE +
    QDCOUNT_SIZE + ANCOUNT_SIZE +
    NSCOUNT_SIZE + ARCOUNT_SIZE;


#[derive(Default)]
pub struct DnsHeader {
    pub frame: [u8; DNS_HEADER_SIZE]
}

impl DnsHeader {
    ///
    ///  A 16 bit identifier assigned by the program that
    ///  generates any kind of query.  This identifier is copied
    ///  the corresponding reply and can be used by the requester
    ///  to match up replies to outstanding queries.
    ///
    pub fn get_id(&self) -> u16 {
        u16::from_be_bytes([self.frame[ID_POS], self.frame[ID_POS+1]])
    }
    pub fn set_id(&mut self, value: u16) {
        let bytes = value.to_be_bytes();
        self.frame[ID_POS] = bytes[0];
        self.frame[ID_POS+1] = bytes[1];
    }
    ///
    /// A one bit field that specifies whether this message is a
    /// query (0), or a response (1).
    ///
    pub fn get_qr(&self) -> bool {
        self.frame[FLAGS_POS] & 0b1000_0000 != 0
    }
    pub fn set_qr(&mut self, value: bool) {
        if value {
            self.frame[FLAGS_POS] |= 0b1000_0000;
        } else {
            self.frame[FLAGS_POS] &= 0b0111_1111;
        }
    }
    pub fn get_op_code(&self) -> OpCode {
        let raw_nr = (self.frame[FLAGS_POS] & 0b0111_1000) >> 3;
        OpCode::from_u8(raw_nr)
    }
    pub fn set_op_code(&mut self, value: OpCode) {
        let bit_mask = (value as u8) << 3;
        self.frame[FLAGS_POS] &= 0b1000_0111;
        self.frame[FLAGS_POS] |= bit_mask;
    }
}

/// A four bit field that specifies kind of query in this
/// message.  This value is set by the originator of a query
/// and copied into the response
#[derive(Eq, PartialEq, Debug)]
pub enum OpCode {
    /// a standard query (QUERY)
    QUERY = 0,
    /// an inverse query (IQUERY)
    IQUERY = 1,
    /// a server status request (STATUS)
    STATUS = 2,
    /// reserved for future use
    RESERVED = 3
}

impl OpCode {
    pub fn from_u8(value: u8) -> OpCode {
        match value {
            0 => OpCode::QUERY,
            1 => OpCode::IQUERY,
            2 => OpCode::STATUS,
            _ => OpCode::RESERVED
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let mut header = DnsHeader::default();
        header.frame[0] = 10;
        header.frame[1] = 32;
        assert_eq!(header.get_id(), 2592);
        header.set_id(1000);
        assert_eq!(header.get_id(), 1000)
    }

    #[test]
    fn test_qr() {
        let mut qr_true = DnsHeader::default();
        qr_true.frame[2] = 0b1000_0000;
        assert_eq!(qr_true.get_qr(), true);
        assert_eq!(DnsHeader::default().get_qr(), false);

        let mut header = DnsHeader::default();
        header.set_qr(false);
        assert_eq!(header.get_qr(), false);
        header.set_qr(true);
        assert_eq!(header.get_qr(), true);
    }

    #[test]
    fn test_op_code() {
        assert_eq!(DnsHeader::default().get_op_code(), OpCode::QUERY);
        let mut header = DnsHeader::default();
        header.frame[FLAGS_POS] = 1 << 3;
        assert_eq!(header.get_op_code(), OpCode::IQUERY);
        header.frame[FLAGS_POS] = 2 << 3;
        assert_eq!(header.get_op_code(), OpCode::STATUS);
        header.frame[FLAGS_POS] = 3 << 3;
        assert_eq!(header.get_op_code(), OpCode::RESERVED);
        header.frame[FLAGS_POS] = 4 << 3;
        assert_eq!(header.get_op_code(), OpCode::RESERVED);

        header = DnsHeader::default();
        header.set_op_code(OpCode::QUERY);
        assert_eq!(header.get_op_code(), OpCode::QUERY);
        header.set_op_code(OpCode::IQUERY);
        assert_eq!(header.get_op_code(), OpCode::IQUERY);
        header.set_op_code(OpCode::STATUS);
        assert_eq!(header.get_op_code(), OpCode::STATUS);
        header.set_op_code(OpCode::RESERVED);
        assert_eq!(header.get_op_code(), OpCode::RESERVED);
    }
}


enum ResourceRecordType {
    /// a host address
    A = 1,

    /// an authoritative name server
    NS = 2,

    /// a mail destination (Obsolete - use MX)
    MD = 3,

    /// a mail forwarder (Obsolete - use MX)
    MF = 4,

    /// the canonical name for an alias
    CNAME = 5,

    /// marks the start of a zone of authority
    SOA = 6,

    /// a mailbox domain name (EXPERIMENTAL)
    MB = 7,

    /// a mail group member (EXPERIMENTAL)
    MG = 8,

    /// a mail rename domain name (EXPERIMENTAL)
    MR = 9,

    /// a null RR (EXPERIMENTAL)
    NULL = 10,

    /// a well known service description
    WKS = 11,

    /// a domain name pointer
    PTR = 12,

    /// host information
    HINFO = 13,

    /// mailbox or mail list information
    MINFO = 14,

    /// mail exchange
    MX = 15,

    /// text strings
    TXT = 16,
}