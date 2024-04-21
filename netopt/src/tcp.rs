use std::net::{TcpListener, TcpStream, SocketAddr, ToSocketAddrs, Shutdown, SocketAddrV4, Ipv4Addr};
use std::io::{self, Read, Write, BufReader, BufWriter};
use std::time::Duration;

use ssl::{SslContext, SslStream};
use mock::MockStream;

use NetworkStream::{
    Tcp,
    Ssl,
    Mock
};

pub struct NetworkOptions {
    ssl: Option<SslContext>,
    mock: Option<MockStream>
}

impl NetworkOptions {
    pub fn new() -> NetworkOptions {
        NetworkOptions {
            ssl: None::<SslContext>,
            mock: None::<MockStream>
        }
    }

    pub fn attach(&mut self, mock: MockStream) -> &mut NetworkOptions {
        self.mock = Some(mock); self
    }

    pub fn tls(&mut self, ssl: SslContext) -> &mut NetworkOptions {
        self.ssl = Some(ssl); self
    }

    pub fn bind<A: ToSocketAddrs>(&self, addr: A) -> io::Result<NetworkListener> {
        Ok(NetworkListener {
            tcp: TcpListener::bind(addr)?,
            ssl: match self.ssl {
                Some(ref ssl) => Some(ssl.clone()),
                None => None
            }
        })
    }

    pub fn connect<A: ToSocketAddrs>(&self, addr: A) -> io::Result<NetworkStream> {
        if let Some(ref mock) = self.mock {
            return Ok(NetworkStream::Mock(mock.clone()));
        };

        let stream = TcpStream::connect(addr)?;
        match self.ssl {
            Some(ref ssl) => Ok(NetworkStream::Ssl(ssl.connect(stream)?)),
            None => Ok(NetworkStream::Tcp(stream))
        }
    }
}

pub struct NetworkListener {
    tcp: TcpListener,
    ssl: Option<SslContext>,
}

impl NetworkListener {
    pub fn accept(&mut self) -> io::Result<(NetworkStream, SocketAddr)> {
        let (stream, addr) = self.tcp.accept()?;
        match self.ssl {
            Some(ref ssl) => {
                match ssl.accept(stream) {
                    Ok(ssl_stream) => Ok((NetworkStream::Ssl(ssl_stream), addr)),
                    Err(e) => Err(e)
                }
            },
            None => Ok((NetworkStream::Tcp(stream), addr))
        }
    }
}

pub enum NetworkStream {
    Tcp(TcpStream),
    Ssl(SslStream),
    Mock(MockStream)
}

impl NetworkStream {
    pub fn peer_addr(&self) -> io::Result<SocketAddr> {
        match *self {
            Tcp(ref s) => s.peer_addr(),
            Ssl(ref s) => s.get_ref().peer_addr(),
            Mock(_) => Ok(SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(127,0,0,1), 80)))
        }
    }

    pub fn shutdown(&self, how: Shutdown) -> io::Result<()> {
        match *self {
            Tcp(ref s) => s.shutdown(how),
            Ssl(ref s) => s.get_ref().shutdown(how),
            Mock(_) => Ok(())
        }
    }

    pub fn set_read_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        match *self {
            Tcp(ref s) => s.set_read_timeout(dur),
            Ssl(ref s) => s.get_ref().set_read_timeout(dur),
            Mock(_) => Ok(())
        }
    }

    pub fn set_write_timeout(&self, dur: Option<Duration>) -> io::Result<()> {
        match *self {
            Tcp(ref s) => s.set_write_timeout(dur),
            Ssl(ref s) => s.get_ref().set_write_timeout(dur),
            Mock(_) => Ok(())
        }
    }
}

impl Read for NetworkStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match *self {
            Tcp(ref mut s) => s.read(buf),
            Ssl(ref mut s) => s.read(buf),
            Mock(ref mut s) => s.read(buf)
        }
    }
}

impl Write for NetworkStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match *self {
            Tcp(ref mut s) => s.write(buf),
            Ssl(ref mut s) => s.write(buf),
            Mock(ref mut s) => s.write(buf)
        }
    }

    fn flush(&mut self) -> io::Result<()> {
        match *self {
            Tcp(ref mut s) => s.flush(),
            Ssl(ref mut s) => s.flush(),
            Mock(ref mut s) => s.flush()
        }
    }
}

pub type NetworkReader = BufReader<NetworkStream>;
pub type NetworkWriter = BufWriter<NetworkStream>;

#[cfg(test)]
mod test {
    use std::net::Shutdown;
    use std::io::{Read, Write};
    use std::thread;
    use super::NetworkOptions;
    use mock::MockStream;

    #[test]
    fn tcp_server_client_test() {
        let mut listener = NetworkOptions::new().bind("127.0.0.1:8432").unwrap();

        thread::spawn(|| {
            let mut client = NetworkOptions::new().connect("127.0.0.1:8432").unwrap();
            client.write(&[0, 1, 2, 3, 4, 5]).unwrap();
            client.flush().unwrap();
            client.shutdown(Shutdown::Both).unwrap();
        });

        let (mut stream, _) = listener.accept().unwrap();
        let mut req = Vec::new();
        stream.read_to_end(&mut req).unwrap();
        assert_eq!(req, vec![0, 1, 2, 3, 4, 5]);
    }

    #[test]
    fn tcp_attach_test() {
        let mock = MockStream::with_vec(vec![0xFE, 0xFD]);
        let mut options = NetworkOptions::new();
        options.attach(mock);
        let mut client = options.connect("127.0.0.1:80").unwrap();
        let mut buf = Vec::new();
        client.read_to_end(&mut buf).unwrap();
        assert_eq!(buf, vec![0xFE, 0xFD]);
    }
}
