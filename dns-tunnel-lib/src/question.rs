use std::io::Read;
use std::io;

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
        vec.push(0);

        for byte in &qtype.to_be_bytes() {
            vec.push(*byte);
        }
        for byte in &qclass.to_be_bytes() {
            vec.push(*byte);
        }
        Question { frame: vec }
    }

    pub fn read<T: Read>(mut reader: T) -> io::Result<Question> {
        let mut frame = vec![];
        loop {
            let mut label_size = [0 as u8];
            reader.read_exact(&mut label_size)?;
            let label_size = label_size[0];
            frame.push(label_size);

            if label_size == 0 {
                break;
            }

            let mut buffer = vec![0 as u8; label_size as usize];
            reader.read_exact(&mut buffer)?;
            frame.append(&mut buffer);
        }

        let mut buffer = vec![0 as u8; QTYPE_SIZE + QCLASS_SIZE];
        reader.read_exact(&mut buffer)?;
        frame.append(&mut buffer);

        Ok(Question { frame })
    }

    pub fn get_qname(&self) -> String {
        let mut i = 0;
        let mut result = String::with_capacity(self.frame.len() - QTYPE_SIZE - QCLASS_SIZE);
        loop {
            let label_size = self.frame[i];
            i += 1;

            if label_size == 0 {
                break;
            } else {
                for _x in 0..label_size {
                    result.push(self.frame[i] as char);
                    i += 1;
                }
                // do not attach dot for last label
                if self.frame[i] != 0 {
                    result.push('.');
                }
            }
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
            0,
            0x00, 0x10,
            0x00, 0x01
        ];
        assert_eq!(q.frame, expected_frame);
    }

    #[test]
    fn test_read() {
        let expected_frame: Vec<u8> = vec![
            6, 0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65,
            3, 0x63, 0x6f, 0x6d,
            0,
            0x00, 0x10,
            0x00, 0x01
        ];
        let mut reader = expected_frame.clone();
        reader.append(&mut vec![0 as u8; 10]);
        let question = Question::read(reader.as_slice()).unwrap();
        assert_eq!(question.frame, expected_frame);
        println!("reader = {:?}", reader);
        println!("expected_frame = {:?}", expected_frame);
    }

    #[test]
    fn test_qname() {
        let q = Question::new("my.url.de", 0, 0);
        assert_eq!(q.get_qname(), "my.url.de".to_string());
    }
}