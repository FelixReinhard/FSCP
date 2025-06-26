#[derive(Debug)]
pub enum Error {
    SimpleError(&'static str),
    SimpleErrorStr(String),
}
