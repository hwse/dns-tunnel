const QTYPE_SIZE: usize = 2;
const QCLASS_SIZE: usize = 2;

#[allow(dead_code)]
pub struct Question {
    frame: Vec<u8>
}

impl Question {
    pub fn new(qname: &str, qtype: u16, qclass: u16) -> Question {
        let mut vec: Vec<u8> = Vec::with_capacity(qname.len() + QTYPE_SIZE + QCLASS_SIZE);
        for label in qname.split('.') {
            vec.push(label.len() as u8);
            for c in label.chars() {
                vec.push(c as u8);
            }
        }
        for byte in &qtype.to_be_bytes() {
            vec.push(*byte);
        }
        for byte in &qclass.to_be_bytes() {
            vec.push(*byte);
        }
        Question { frame: vec }

    }
    pub fn get_qname(&self) -> String {
        let mut i = 0;
        let mut result = String::with_capacity(self.frame.len() - QTYPE_SIZE - QCLASS_SIZE);
        loop {
            let label_size = self.frame[i];
            if label_size == 0 {
                break;
            }
            result.push(self.frame[i] as char);
            i += 1;
        }
        result
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_new() {
        let q = Question::new("google.com", 16, 1);
        let expected_frame: Vec<u8> = vec![
            6, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65,
            3, 0x63, 0x6f, 0x6d,
            0x00, 0x10,
            0x00, 0x01
        ];
        assert_eq!(q.frame, expected_frame);
    }
}