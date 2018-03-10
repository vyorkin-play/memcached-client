use bytes::BytesMut;

#[derive(Debug, PartialEq, Clone)]
pub enum Request {
    Version
}

impl Request {
    pub fn write_to(&self, dst: &mut BytesMut) {
        match *self {
            Request::Version => {
                dst.extend_from_slice(b"version\r\n")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bytes::Bytes;
    use spectral::prelude::*;

    #[test]
    fn should_write_version() {
        let req = Request::Version;
        let mut dst = BytesMut::new();

        req.write_to(&mut dst);

        assert_that(&dst.freeze())
            .is_equal_to(Bytes::from("version\r\n"));
    }
}
