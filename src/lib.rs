use std::{
    io::{Bytes, ErrorKind},
    net::UdpSocket,
};

const PACKET_BUFFER_SIZE: usize = 2096;

struct ConnectPayload {}

struct MessagePayload {
    string: String,
}

pub enum Payload {
    ConnectPayload,
    MessagePayload,
}

struct Packet {
    payload: Payload,
    address: String,
    port: u16,
}

impl Payload {
    pub const fn serialize_as_bytes(&self) -> [u8; 2048] {
        let mut bytes: [u8; 2048] = [0; 2048];
        match self {
            Payload::ConnectPayload => {
                bytes[0] = 0;
            }
            Payload::MessagePayload => {
                bytes[0] = 1;
            }
        }
        bytes
    }

    pub fn deserialize_from_bytes(&self, bytes: &[u8]) {
        match bytes[0] {
            0 => {
                println!("received ConnectPayload");
            }
            1 => {
                println!("received MessagePayload");
            }
            _ => {
                println!("received unknown Payload");
            }
        }
    }
}

pub(crate) fn send_payload(socket: &UdpSocket, payload: &Payload) {
    socket.send(&payload.serialize_as_bytes());
}

pub(crate) fn receive_payload(socket: &UdpSocket) -> Payload {
    let mut received_payload = Payload::ConnectPayload;
    let mut receive_buffer = [0; PACKET_BUFFER_SIZE];

    let recv_result = socket.recv_from(&mut receive_buffer);
    match recv_result {
        Ok((number_of_bytes_read, source_address)) => {
            received_payload.deserialize_from_bytes(&receive_buffer[..number_of_bytes_read])
        }
        Err(error) if error.kind() != ErrorKind::WouldBlock => {
            println!("receive failed: {:?}", error);
        }
        _ => (),
    }

    Payload::ConnectPayload
}

struct ServerSocket {
    socket: Option<UdpSocket>,
    buffer: Vec<u8>,
}

impl ServerSocket {
    fn new() -> Self {
        ServerSocket {
            socket: None,
            buffer: vec![0; PACKET_BUFFER_SIZE],
        }
    }

    fn start_listening_on_port(&mut self, port: u16) -> Result<(), &str> {
        self.socket = Some(UdpSocket::bind(format!("[::]:{}", port)).unwrap());
        Ok(())
    }
}

struct ClientSocket {
    socket: Option<UdpSocket>,
    buffer: Vec<u8>,
}

impl ClientSocket {
    fn new() -> Self {
        ClientSocket {
            socket: None,
            buffer: vec![0; PACKET_BUFFER_SIZE],
        }
    }

    fn send(&mut self, payload: &Payload) {
        send_payload(&self.socket.as_mut().unwrap(), &payload);
    }

    fn connect_to_server(&mut self, address: &str, port: u16) -> Result<(), &str> {
        let mut socket = UdpSocket::bind("[::]:0").unwrap();
        match socket.connect(format!("{}:{}", address, port)) {
            Ok(()) => {}
            Err(err) => {
                return Err("Could not connect!");
            }
        }
        let payload = Payload::ConnectPayload;
        send_payload(&socket, &payload);
        self.socket = Some(socket);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;

        let mut server_socket = crate::ServerSocket::new();
        let mut client_socket = crate::ClientSocket::new();

        server_socket.start_listening_on_port(1337).unwrap();

        client_socket.connect_to_server("127.0.0.1", 1337).unwrap();

        let bla = "hello".as_bytes();

        crate::receive_payload(&server_socket.socket.as_mut().unwrap());

        /*let packet_received_by_server_socket = server_socket.receive();

        let packet_responded_to_client = hermes::Packet::new("hello".as_bytes());

        client_socket.receive();

        server_socket.send();*/

        assert_eq!(result, 4);
    }
}
