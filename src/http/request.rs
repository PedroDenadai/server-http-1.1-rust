use std::str;
use std::str::Utf8Error;
use super::method::Method;
use std::convert::TryFrom;
use std::error::Error;
use std::fmt::{Result as FmtResult, Display, Debug, Formatter};



pub struct Request{
    path: String,
    query_string: Option<String>,
    method: Method, // use super because of the scope of the struct 
}


impl TryFrom<&[u8]> for Request{
    type Error = ParseError;
fn try_from(buf: &[u8]) -> Result<Self, Self::Error>{
        //match str::from_utf8(buf){
            //Ok(request) => {}
            //Err(_) => return Err(ParseError::InvalidEncoding),}

        //match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
          //  Ok(request) => {}
           // Err(e) => return Err(e),}

        let request = str::from_utf8(buf)?;  

        //match get_word(request) {
        //    Some((method, request)) => {},
        //    None => return Err(ParseError::InvalidRequest),
        //}
        let (method, request) = get_word(request).ok_or(ParseError::InvalidRequest)?;
        let (path, request) = get_word(request).ok_or(ParseError::InvalidRequest)?; 
        let (protocol, _) = get_word(request).ok_or(ParseError::InvalidRequest)?;

        if protocol != "HTTP/1.1" {
            return Err(ParseError::InvalidProtocol);
        }

        unimplemented!()   
    }
}


//recebe uma str e retorna um tuple que tem 2 valores, 1 que é a str de interesse (ate um espaco) e o resto da str
fn get_word(request: &str) -> Option<(&str, &str)>{
    //let mut iter = request.chars();
    //loop {
    //    let item = iter.next();
      //  match item {
  //          Some(c) => {},
//            None => break,
      //  }
    //}

    for (i, c) in request.chars().enumerate() {
        if c == ' ' || c == '\r' {
            return Some((&request[..i], &request[i + 1..]));
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

impl ParseError{
    fn message(&self) -> &str{
        match self{
            Self::InvalidEncoding => "Invalid Encoding",
            Self::InvalidMethod => "Invalid Method",
            Self::InvalidProtocol => "Invalid Protocol",
            Self::InvalidRequest => "Invalid Request",
        }
    }
}

impl From<Utf8Error> for ParseError {
    fn from(_: Utf8Error) -> Self {
        Self::InvalidEncoding
    }
}

impl Display for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

impl Debug for ParseError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", self.message())
    }
}

//impl Error for ParseError{

//}