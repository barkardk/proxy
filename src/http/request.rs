use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Display, Debug};
use core::fmt;
use std::str;
use crate::http::method::MethodError;
use std::str::Utf8Error;


pub struct Request {
    path: String,
    query_string: Option<String>,
    method: Method,
}

impl Request{
    fn from_byte_array(buf: &[u8]) -> Result<Self, String> {   }
}

impl TryFrom<&[u8]> for Request {
    type Error = ParseError;
    // GET /search?name=stuff&sort=1 HTTP/1.1
    fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
        // The ? in the end replaces the usual match to unwrap the result
        let request = str::from_utf8(buf)?;
        let (method, request)  =  get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request)  =  get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        let (protocol, _)  =  get_next_word(request).ok_or(ParseError::InvalidRequest)?;
        if protocol != "HTTP/1.1." {
            return Err(ParseError::InvalidProtocol);
        }
        let method: Method = method.parse()?;

    }
}

fn get_next_word(request: &str)-> Option<(&str, &str)> {
    for (i, c)  in request.chars().enumerate(){
        if c == ' ' || c == '\r' {
            return Some((&request[..i],&request[i+1..]));
        }
    }
    None
}
pub enum ParseError {
    InvalidRequest,
    InvalidEncoding,
    InvalidProtocol,
    InvalidMethod,
}

impl ParseError {
    fn message(&self)-> &str {
        match self {
            self::InvalidRequest=> "[ERROR] Invalid request",
            self::InvalidEncoding=> "[ERROR] Invalid encoding",
            self::InvalidProtocol=> "[ERROR] Invalid protocol",
            self::InvalidMethod=> "[ERROR] Invalid method",
        }
    }
}
impl Error for ParseError {}
impl From<MethodError> for ParseError {
    fn from(_: MethodError) -> Self {
        Self::InvalidMethod
    }
}
impl From<Utf8Error> for ParseError{
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}
impl Display for ParseError{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}
impl Debug for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message())
    }
}