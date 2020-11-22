use std::net::{TcpListener, TcpStream};
use std::io::Read;

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server {
            addr,
        }
    }
    fn handle_request(&self, s: TcpStream) {
        println!("received {:?}",s);
    }
    pub fn run(self){
        println!("I am listening to {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        loop {
            match listener.accept() {
                Ok((mut socket, addr)) =>  {
                    let mut buffer = [0; 1024];
                    match socket.read(&mut buffer) {
                        Ok(_) => println!("new client: {}", String::from_utf8_lossy(&buffer)),
                        Err(e) => println!("[ERROR] {}", e),
                    }

                }
                Err(e) => println!("[ERROR] couldn't get client: {:?}", e),
            }
        }



    }
}
