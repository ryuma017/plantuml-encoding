use std::{convert, string};

#[derive(Debug, PartialEq)]
pub struct FromPlantumlError(pub String);

impl convert::From<string::FromUtf8Error> for FromPlantumlError {
    fn from(err: string::FromUtf8Error) -> Self {
        FromPlantumlError(format!(
            "there is a problem during decoding: `{err}`"
        ))
    }
}

impl convert::From<hex::FromHexError> for FromPlantumlError {
    fn from(err: hex::FromHexError) -> Self {
        FromPlantumlError(format!("there is a problem during hex decoding: `{err}`"))
    }
}
