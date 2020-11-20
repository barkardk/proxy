pub enum Method {
    GET,
    POST,
    HEAD,
    PUT,
    DELETE,
    CONNECT,
    OPTIONS,
    TRACE,
    PATCH,
}

pub fn get_method(method: Method) {
    match method {
        Method::GET => println!("GET"),
        _ => {}
    }
}