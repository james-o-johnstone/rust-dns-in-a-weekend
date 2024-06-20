use rand::Rng;
use struct_bytes::ToBytes;
use std::net::UdpSocket;

// https://implement-dns.wizardzines.com/book/part_1

#[derive(ToBytes)]
struct DNSHeader {
    id: u16,
    flags: u16,
    num_questions: u16,
    num_answers: u16,
    num_authorities: u16,
    num_additionals: u16,
}

#[derive(ToBytes)]
struct DNSQuestion {
    name: String,
    r#type: u16, // escaped with r# to use as identifier
    class:  u16, // not reserved word in rust
}

impl DNSQuestion {
    pub fn new(domain_name: String, r#type: u16, class: u16) -> Self {
        Self {
            name: Self::encode_dns_name(domain_name),
            r#type,
            class,
        }
    }

    fn encode_dns_name(domain_name: String) -> String {
        let mut encoded = Vec::<u8>::new();
        domain_name.split(".").for_each(|part| {
            encoded.push(part.len().try_into().unwrap()); // add the number of bytes of the part
            encoded.extend_from_slice(part.as_bytes());
        });
        encoded.push(b'\0'); // add zero byte
        String::from_utf8(encoded).unwrap()
    }
}

const TYPE_A: u16 = 1;
const CLASS_IN: u16 = 1;
fn build_query(domain_name: String, record_type: u16) -> Vec<u8> {
    let RECURSION_DESIRED = 1 << 8;
    let header = DNSHeader {
        id: rand::thread_rng().gen(),
        flags: RECURSION_DESIRED,
        num_questions: 1,
        num_answers: 0,
        num_authorities: 0,
        num_additionals: 0
    };
    let question = DNSQuestion::new(
        domain_name,
        record_type,
        CLASS_IN
    );
    let mut query = Vec::<u8>::new();
    query.append(header.to_bytes().as_mut());
    query.append(question.to_bytes().as_mut());
    query
}

fn main() -> std::io::Result<()> {
    {
        let query = build_query(String::from("www.example.com"), TYPE_A);
        let socket = UdpSocket::bind("0.0.0.0:34254").unwrap();
        socket.send_to(&query, "8.8.8.8:53").unwrap();
        let mut buf = [0; 1024];
        socket.recv_from(&mut buf)?;
    }
    Ok(())
}
