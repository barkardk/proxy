mod server;
use server::Server;


fn main() {
    let a = 2;
    let result = stack_only(a);
    dbg!(result);
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run();
}

fn stack_only(b: i32) -> i32{
    let c = 3;
    return b+c+ stack_and_heap();
}

fn stack_and_heap() -> i32 {
    let d=5;
    let e = Box::new(7);
    return d+*e;
}