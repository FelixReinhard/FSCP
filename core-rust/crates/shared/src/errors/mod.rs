use core::fmt;

#[derive(Debug)]
pub enum Error {
    SimpleError(&'static str),
    SimpleErrorStr(String),
}

impl Error {
    // turns an error into our errors
    pub fn from<E, M>(err: Result<M, E>) -> Result<M, Error>
    where
        E: fmt::Debug,
    {
        match err {
            Ok(v) => Ok(v),
            Err(err) => Err(Error::SimpleErrorStr(format!("{:?}", err))),
        }
    }
}
