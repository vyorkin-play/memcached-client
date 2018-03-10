use tokio_io::codec::{Decoder, Encoder};
use std::io;
use protocol::{Request, Response};
use nom::IResult;
use bytes::BytesMut;

pub struct Codec;

impl Encoder for Codec {
    type Item = Request;
    type Error = io::Error;

    fn encode(&mut self, item: Self::Item, dst: &mut BytesMut) -> Result<(), Self::Error> {
        item.write_to(dst);
        Ok(())
    }
}

impl Decoder for Codec {
    type Item = Response;
    type Error = io::Error;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        if src.is_empty() {
            return Ok(None)
        }

        let bytes = src.clone().freeze();
        let result = Response::parse(&bytes);

        match result {
            IResult::Done(rest, resp) => {
                let offset = bytes.len() - rest.len();
                src.advance(offset);
                Ok(Some(resp))
            },
            IResult::Incomplete(_) => Ok(None),
            IResult::Error(ref err) => {
                let io_err = io::Error::new(io::ErrorKind::InvalidData, err.description());
                Err(io_err)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;

    #[test]
    fn should_decode_complete_message() {
        let mut codec = Codec;
        let mut src = BytesMut::from("VERSION 0.1\r\n");

        let expected = Response::Version("0.1".to_string());

        let result = codec.decode(&mut src);
        assert_that(&result).is_ok().is_some().is_equal_to(expected);

        let result = codec.decode(&mut src);
        assert_that(&result).is_ok().is_none();
    }

    #[test]
    fn should_decode_incomplete_message() {
        let mut codec = Codec;
        let mut src = BytesMut::from("VERSION");

        let expected = Response::Version("0.1".to_string());

        let result = codec.decode(&mut src);
        assert_that(&result).is_ok().is_none();

        src.extend_from_slice(b" 0.1\r\n");

        let result = codec.decode(&mut src);
        assert_that(&result).is_ok().is_some().is_equal_to(expected);

        let result = codec.decode(&mut src);
        assert_that(&result).is_ok().is_none();
    }

    #[test]
    fn should_decode_completed_and_incomplete_message() {
        let mut codec = Codec;
        let mut src = BytesMut::from("VERSION 0.1\r\nVERSION");

        let expected = Response::Version("0.1".to_string());

        let result = codec.decode(&mut src);
        assert_that(&result).is_ok().is_some().is_equal_to(&expected);

        let result = codec.decode(&mut src);
        assert_that(&result).is_ok().is_none();

        src.extend_from_slice(b" 0.1\r\n");

        let result = codec.decode(&mut src);
        assert_that(&result).is_ok().is_some().is_equal_to(&expected);

        let result = codec.decode(&mut src);
        assert_that(&result).is_ok().is_none();
    }
}
