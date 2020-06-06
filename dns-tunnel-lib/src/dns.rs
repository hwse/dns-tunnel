
use std::io;
use std::io::{Read};

const ID_SIZE: usize = 2;
const ID_POS: usize = 0;

const DNS_HEADER_SIZE: usize = ID_SIZE;

struct DnsHeader {
    frame: [u8; DNS_HEADER_SIZE]
}

impl DnsHeader {
    fn id(self) -> u16 {
        u16::from_be_bytes()
    }
}

impl Read for DnsHeader {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        buf.
    }
}

enum ResourceRecordType {
    // a host address
    A = 1,

    // an authoritative name server
    NS = 2,

    // a mail destination (Obsolete - use MX)
    MD = 3,

    // a mail forwarder (Obsolete - use MX)
    MF = 4,

    // the canonical name for an alias
    CNAME = 5,

    // marks the start of a zone of authority
    SOA = 6,

    // a mailbox domain name (EXPERIMENTAL)
    MB = 7,

    // a mail group member (EXPERIMENTAL)
    MG = 8,

    // a mail rename domain name (EXPERIMENTAL)
    MR = 9,

    // a null RR (EXPERIMENTAL)
    NULL = 10,

    // a well known service description
    WKS = 11,

    // a domain name pointer
    PTR = 12,

    // host information
    HINFO = 13,

    // mailbox or mail list information
    MINFO = 14,

    // mail exchange
    MX = 15,

    // text strings
    TXT = 16,
}