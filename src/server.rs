pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Server {
            addr,
        }
    }
    pub fn run(self){
        println!("I am listening to {}", self.addr);
    }
}
