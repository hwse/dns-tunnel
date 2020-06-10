use std::io;
use std::io::Read;

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


#[derive(Default, Debug)]
pub struct Header {
    pub frame: [u8; DNS_HEADER_SIZE]
}

#[allow(dead_code)]
impl Header {
    pub fn read<T: Read>(mut reader: T) -> io::Result<Header> {
        let mut frame = [0 as u8; DNS_HEADER_SIZE];
        reader.read_exact(&mut frame)?;
        Ok(Header { frame })
    }

    ///
    ///  A 16 bit identifier assigned by the program that
    ///  generates any kind of query.  This identifier is copied
    ///  the corresponding reply and can be used by the requester
    ///  to match up replies to outstanding queries.
    ///
    pub fn get_id(&self) -> u16 {
        self.get_u16(ID_POS)
    }
    pub fn set_id(&mut self, value: u16) {
        self.set_u16(ID_POS, value)
    }

    ///
    /// A one bit field that specifies whether this message is a
    /// query (0), or a response (1).
    ///
    pub fn get_qr(&self) -> bool {
        self.get_flag_bit(FLAGS_POS, 0)
    }
    pub fn set_qr(&mut self, value: bool) {
        self.set_flag_bit(FLAGS_POS, 0, value)
    }

    ///
    /// A four bit field that specifies kind of query in this
    /// message.  This value is set by the originator of a query
    /// and copied into the response.
    ///
    pub fn get_op_code(&self) -> OpCode {
        let raw_nr = (self.frame[FLAGS_POS] & 0b0111_1000) >> 3;
        OpCode::from_u8(raw_nr)
    }
    pub fn set_op_code(&mut self, value: OpCode) {
        let bit_mask = (value as u8) << 3;
        self.frame[FLAGS_POS] &= 0b1000_0111;
        self.frame[FLAGS_POS] |= bit_mask;
    }

    ///
    /// Authoritative Answer - this bit is valid in responses,
    /// and specifies that the responding name server is an
    /// authority for the domain name in question section.
    ///
    /// Note that the contents of the answer section may have
    /// multiple owner names because of aliases. The AA bit
    /// corresponds to the name which matches the query name, or
    /// the first owner name in the answer section.
    ///
    pub fn get_aa(&self) -> bool {
        self.get_flag_bit(FLAGS_POS, 5)
    }
    pub fn set_add(&mut self, value: bool) {
        self.set_flag_bit(FLAGS_POS, 5, value)
    }

    ///
    /// TrunCation - specifies that this message was truncated
    /// due to length greater than that permitted on the
    /// transmission channel.
    ///
    pub fn get_tc(&self) -> bool {
        self.get_flag_bit(FLAGS_POS, 6)
    }
    pub fn set_tc(&mut self, value: bool) {
        self.set_flag_bit(FLAGS_POS, 6, value)
    }

    ///
    /// Recursion Desired - this bit may be set in a query and
    /// is copied into the response.  If RD is set, it directs
    /// the name server to pursue the query recursively.
    /// Recursive query support is optional.
    ///
    pub fn get_rd(&self) -> bool {
        self.get_flag_bit(FLAGS_POS, 7)
    }
    pub fn set_rd(&mut self, value: bool) {
        self.set_flag_bit(FLAGS_POS, 7, value)
    }

    ///
    /// Recursion Available - this be is set or cleared in a
    /// response, and denotes whether recursive query support is
    /// available in the name server.
    ///
    pub fn get_ra(&self) -> bool {
        self.get_flag_bit(FLAGS_POS + 1, 0)
    }
    pub fn set_ra(&mut self, value: bool) {
        self.set_flag_bit(FLAGS_POS + 1, 0, value)
    }

    pub fn get_rcode(&self) -> Option<RCode> {
        let raw_nr = self.frame[FLAGS_POS + 1] & 0b0000_1111;
        RCode::from_u8(raw_nr)
    }
    pub fn set_rcode(&mut self, value: RCode) {
        let bit_mask = value as u8;
        self.frame[FLAGS_POS + 1] &= 0b1111_0000;
        self.frame[FLAGS_POS + 1] |= bit_mask;
    }

    pub fn get_qdcount(&self) -> u16 {
        self.get_u16(QDCOUNT_POS)
    }
    pub fn set_qdcount(&mut self, value: u16) {
        self.set_u16(QDCOUNT_POS, value)
    }

    pub fn get_ancount(&self) -> u16 {
        self.get_u16(ANCOUNT_POS)
    }
    pub fn set_ancount(&mut self, value: u16) {
        self.set_u16(ANCOUNT_POS, value)
    }

    pub fn get_anscount(&self) -> u16 {
        self.get_u16(NSCOUNT_POS)
    }
    pub fn set_nscount(&mut self, value: u16) {
        self.set_u16(NSCOUNT_POS, value)
    }

    pub fn get_arscount(&self) -> u16 {
        self.get_u16(ARCOUNT_POS)
    }
    pub fn set_nrcount(&mut self, value: u16) {
        self.set_u16(ARCOUNT_POS, value)
    }

    ///
    /// Read a binary flag (a single bit) set in the frame
    ///
    fn get_flag_bit(&self, byte_pos: usize, bit_pos: u8) -> bool {
        assert!(bit_pos < 8);
        // bit mask: bit_pos = 0 -> 0b1000_0000 == 1 << (7 - 0)
        // bit mask: bit_pos = 7 -> 0b0000_0001 == 1 << (7 - 7)
        self.frame[byte_pos] & 1 << (7 - bit_pos) != 0
    }

    ///
    /// Set a binary flag (a single bit) in the frame
    ///
    fn set_flag_bit(&mut self, byte_pos: usize, bit_pos: u8, value: bool) {
        assert!(bit_pos < 8);
        if value {
            self.frame[byte_pos] |= 1 << (7 - bit_pos)
        } else {
            self.frame[byte_pos] &= !(1 << (7 - bit_pos))
        }
    }

    fn get_u16(&self, byte_pos: usize) -> u16 {
        u16::from_be_bytes([self.frame[byte_pos], self.frame[byte_pos + 1]])
    }
    fn set_u16(&mut self, byte_pos: usize, value: u16) {
        let bytes = value.to_be_bytes();
        self.frame[byte_pos] = bytes[0];
        self.frame[byte_pos + 1] = bytes[1];
    }
}

///
/// A four bit field that specifies kind of query in this
/// message.  This value is set by the originator of a query
/// and copied into the response
///
#[derive(Eq, PartialEq, Debug)]
pub enum OpCode {
    /// a standard query (QUERY)
    QUERY = 0,
    /// an inverse query (IQUERY)
    IQUERY = 1,
    /// a server status request (STATUS)
    STATUS = 2,
    /// reserved for future use
    RESERVED = 3,
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

#[derive(Eq, PartialEq, Debug)]
pub enum RCode {
    /// No error condition
    NoError = 0,

    /// Format error - The name server was unable to interpret the query.
    FormatError = 1,

    /// Server failure - The name server was
    /// unable to process this query due to a
    /// problem with the name server.
    ServerFailure = 2,

    /// Name Error - Meaningful only for
    /// responses from an authoritative name
    /// server, this code signifies that the
    /// domain name referenced in the query does
    /// not exist.
    NameError = 3,

    /// Not Implemented - The name server does
    /// not support the requested kind of query.
    NotImplemented = 4,

    /// Refused - The name server refuses to
    /// perform the specified operation for
    /// policy reasons.  For example, a name
    /// server may not wish to provide the
    /// information to the particular requester,
    /// or a name server may not wish to perform
    /// a particular operation (e.g., zone
    Refused = 5,

}

impl RCode {
    pub fn from_u8(value: u8) -> Option<RCode> {
        match value {
            0 => Some(RCode::NoError),
            1 => Some(RCode::FormatError),
            2 => Some(RCode::ServerFailure),
            3 => Some(RCode::NameError),
            4 => Some(RCode::NotImplemented),
            5 => Some(RCode::Refused),
            _ => None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id() {
        let mut header = Header::default();
        header.frame[0] = 10;
        header.frame[1] = 32;
        assert_eq!(header.get_id(), 2592);
        header.set_id(1000);
        assert_eq!(header.get_id(), 1000)
    }

    #[test]
    fn test_qr() {
        let mut qr_true = Header::default();
        qr_true.frame[2] = 0b1000_0000;
        assert_eq!(qr_true.get_qr(), true);
        assert_eq!(Header::default().get_qr(), false);

        let mut header = Header::default();
        header.set_qr(false);
        assert_eq!(header.get_qr(), false);
        header.set_qr(true);
        assert_eq!(header.get_qr(), true);
    }

    #[test]
    fn test_op_code() {
        assert_eq!(Header::default().get_op_code(), OpCode::QUERY);
        let mut header = Header::default();
        header.frame[FLAGS_POS] = 1 << 3;
        assert_eq!(header.get_op_code(), OpCode::IQUERY);
        header.frame[FLAGS_POS] = 2 << 3;
        assert_eq!(header.get_op_code(), OpCode::STATUS);
        header.frame[FLAGS_POS] = 3 << 3;
        assert_eq!(header.get_op_code(), OpCode::RESERVED);
        header.frame[FLAGS_POS] = 4 << 3;
        assert_eq!(header.get_op_code(), OpCode::RESERVED);

        header = Header::default();
        header.set_op_code(OpCode::QUERY);
        assert_eq!(header.get_op_code(), OpCode::QUERY);
        header.set_op_code(OpCode::IQUERY);
        assert_eq!(header.get_op_code(), OpCode::IQUERY);
        header.set_op_code(OpCode::STATUS);
        assert_eq!(header.get_op_code(), OpCode::STATUS);
        header.set_op_code(OpCode::RESERVED);
        assert_eq!(header.get_op_code(), OpCode::RESERVED);
    }

    #[test]
    fn test_get_bit_flag() {
        let mut header = Header::default();
        header.frame[0] = 0b11001010;
        assert_eq!(header.get_flag_bit(0, 0), true);
        assert_eq!(header.get_flag_bit(0, 1), true);
        assert_eq!(header.get_flag_bit(0, 2), false);
        assert_eq!(header.get_flag_bit(0, 3), false);
        assert_eq!(header.get_flag_bit(0, 4), true);
        assert_eq!(header.get_flag_bit(0, 5), false);
        assert_eq!(header.get_flag_bit(0, 6), true);
        assert_eq!(header.get_flag_bit(0, 7), false);
    }

    #[test]
    fn test_set_bit_flag() {
        let mut header = Header::default();
        for i in 0..2 {
            for bit_pos in 0..7 {
                header.set_flag_bit(i, bit_pos, true);
                assert_eq!(header.get_flag_bit(i, bit_pos), true);
                header.set_flag_bit(i, bit_pos, false);
                assert_eq!(header.get_flag_bit(i, bit_pos), false);
            }
        }
    }
}

#[allow(dead_code)]
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