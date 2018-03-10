use tokio_proto::pipeline::ClientProto;
use tokio_io::codec::Framed;
use tokio_io::{AsyncRead, AsyncWrite};
use std::io;

use super::codec::Codec;
use protocol::{Request, Response};

pub struct Proto;

impl<T> ClientProto<T> for Proto
    where
    T: AsyncWrite + AsyncRead + 'static
{
    type Request = Request;
    type Response = Response;
    type Transport = Framed<T, Codec>;
    type BindTransport = Result<Self::Transport, io::Error>;

    fn bind_transport(&self, io: T) -> Self::BindTransport {
        Ok(io.framed(Codec))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use spectral::prelude::*;
    use tokio_proto::TcpClient;
    use tokio_core::reactor::Core;
    use futures::Future;
    use tokio_service::Service;

    #[test]
    fn should_call_service() {
        let mut core = Core::new().unwrap();
        let addr = ([127, 0, 0, 1], 11211).into();
        let fut =
            TcpClient::new(Proto)
            .connect(&addr, &core.handle())
            .and_then(|svc| svc.call(Request::Version));

        let result = core.run(fut).unwrap();
        let expected = Response::Version("1.5.6".to_string());

        assert_that(&result).is_equal_to(&expected)
    }
}
