
#[derive(Debug, Clone)]
pub struct Request {
    pub msg:String
}

impl Request {
    pub fn new() -> Request{
        Request{
            msg: String::new()
        }
    }
}