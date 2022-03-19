use std::{io::ErrorKind, net::UdpSocket, time::SystemTime};

struct Connection {
    address: String,
    port: u16,
    //last_activity: SystemTime,
}

impl Connection {
    pub fn new(address: &str, port: u16) -> Self {
        Self {
            address: address.to_string(),
            port,
            //last_activity: SystemTime::UNIX_EPOCH,
        }
    }
}

pub trait Receive {
    fn receive(&mut self) -> Option<(&[u8], String, u16)>;
}

pub struct UdpServer {
    socket: UdpSocket,
    client_connections: Vec<Connection>,
    receive_buffer: Vec<u8>,
}

impl UdpServer {
    pub fn new() -> Self {
        let socket = UdpSocket::bind("[::]:0").unwrap();
        socket.set_nonblocking(true).unwrap();
        Self {
            socket,
            client_connections: Vec::new(),
            receive_buffer: vec![0; 2096],
        }
    }

    pub fn start_listening_on_port(&mut self, port: u16) {
        let socket_address = format!("{}:{}", "[::]", port);
        let socket = UdpSocket::bind(socket_address).unwrap();
        socket.set_nonblocking(true).unwrap();
        self.socket = socket;
    }

    pub fn send(&self, message: &[u8]) {
        for client_connection in &self.client_connections {
            println!(
                ">> message: {}, to: {}:{}",
                std::str::from_utf8(message).unwrap(),
                client_connection.address,
                client_connection.port,
            );
            let dest_addr = format!("{}:{}", client_connection.address, client_connection.port);
            self.socket.send_to(message, dest_addr).unwrap();
        }
    }
}

impl Receive for UdpServer {
    fn receive(&mut self) -> Option<(&[u8], String, u16)> {
        let mut result: Option<(&[u8], String, u16)> = None;
        let recv_result = self.socket.recv_from(&mut self.receive_buffer);
        match recv_result {
            Ok((number_of_bytes_read, source_address)) => {
                println!(
                    "<< message: {}, from: {}:{}",
                    std::str::from_utf8(&self.receive_buffer[..number_of_bytes_read]).unwrap(),
                    source_address.ip().to_string(),
                    source_address.port(),
                );
                if &self.receive_buffer[..number_of_bytes_read] == "hello".as_bytes() {
                    self.client_connections.push(Connection::new(
                        &source_address.ip().to_string(),
                        source_address.port(),
                    ));
                }
                result = Some((
                    &self.receive_buffer[..number_of_bytes_read],
                    source_address.ip().to_string(),
                    source_address.port(),
                ));
            }
            Err(error) if error.kind() != ErrorKind::WouldBlock => {
                println!("receive failed: {:?}", error);
            }
            _ => (),
        }

        result
    }
}

pub struct UdpClient {
    socket: UdpSocket,
    server_connection: Option<Connection>,
    receive_buffer: Vec<u8>,
}

impl UdpClient {
    pub fn new() -> Self {
        let socket = UdpSocket::bind("[::]:0").unwrap();
        socket.set_nonblocking(true).unwrap();
        Self {
            socket: socket,
            server_connection: None,
            receive_buffer: vec![0; 2096],
        }
    }

    pub fn connect_to_server(&mut self, address: &str, port: u16) {
        let server_connection = Connection::new(&address.to_string(), port);
        let server_addr = format!("{}:{}", server_connection.address, server_connection.port,);
        self.socket
            .send_to("hello".as_bytes(), server_addr)
            .unwrap();
        self.server_connection = Some(server_connection);
    }

    pub fn send(&self, message: &[u8]) {
        println!(
            ">> message: {}, to: {}:{}",
            std::str::from_utf8(message).unwrap(),
            self.server_connection.as_ref().unwrap().address,
            self.server_connection.as_ref().unwrap().port,
        );
        let dest_addr = format!(
            "{}:{}",
            self.server_connection.as_ref().unwrap().address,
            self.server_connection.as_ref().unwrap().port,
        );
        self.socket.send_to(message, dest_addr).unwrap();
    }
}

impl Receive for UdpClient {
    fn receive(&mut self) -> Option<(&[u8], String, u16)> {
        let mut result: Option<(&[u8], String, u16)> = None;
        let recv_result = self.socket.recv_from(&mut self.receive_buffer);
        match recv_result {
            Ok((number_of_bytes_read, source_address)) => {
                println!(
                    "<< message: {}, from: {}:{}",
                    std::str::from_utf8(&self.receive_buffer[..number_of_bytes_read]).unwrap(),
                    source_address.ip().to_string(),
                    source_address.port(),
                );
                result = Some((
                    &self.receive_buffer[..number_of_bytes_read],
                    source_address.ip().to_string(),
                    source_address.port(),
                ));
            }
            Err(error) if error.kind() != ErrorKind::WouldBlock => {
                println!("receive failed: {:?}", error);
            }
            _ => (),
        }

        result
    }
}
