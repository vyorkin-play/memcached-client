use nom::IResult;
use nom::{digit, not_line_ending};
use std::str;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
pub enum Response {
    Version(String)
}

impl Response {
    named!(pub parse<&[u8], Response>,
           do_parse!(
               tag!("VERSION ") >>
               version: map_res!(not_line_ending, str::from_utf8) >>
               tag!("\r\n") >>
               (Response::Version(version.to_string()))
           )
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn should_parse_version() {
        let req = b"VERSION 0.1\r\n";
        let (rest, resp) = Response::parse(req).unwrap();

        let expected = Response::Version("0.1".to_string());
        assert_that(&resp).is_equal_to(expected);
        assert_that(&rest.len()).is_equal_to(0);
    }
}
