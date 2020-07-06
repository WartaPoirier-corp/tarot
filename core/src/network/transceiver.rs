use std::sync::mpsc;
use std::thread;
use serde::{Deserialize, Serialize};
use std::io::{Write, Read};
use serde::de::DeserializeOwned;
use serde::export::PhantomData;
use std::sync::Arc;

pub struct Transceiver<Packet: 'static + Send, MPSC> {
    mpsc: MPSC,

    // Require the Packet to be the same for the whole `Transceiver` to prevent misuse
    _p: PhantomData<Packet>,
}

impl<Packet: 'static + Send, MPSC> Drop for Transceiver<Packet, MPSC> {
    fn drop(&mut self) {
        // TODO kill threads
    }
}

impl<Packet: 'static + Send + Serialize> Transceiver<Packet, mpsc::Sender<Packet>> {
    pub fn create_transmitter<Stream: 'static + Send + Write>(mut stream: Stream) -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || loop {
            match rx.try_recv() {
                Ok(packet) => bincode::serialize_into(&mut stream, &packet).unwrap(),
                Err(mpsc::TryRecvError::Disconnected) => break,
                Err(mpsc::TryRecvError::Empty) => (),
            }
        });

        Self {
            mpsc: tx,
            _p: Default::default(),
        }
    }

    pub fn send(&mut self, packet: Packet) {
        self.mpsc.send(packet).unwrap();
    }
}

impl<Packet: 'static + Send + DeserializeOwned> Transceiver<Packet, mpsc::Receiver<Packet>> {
    pub fn create_receiver<Stream: 'static + Send + Read>(mut stream: Stream) -> Self {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || loop {
            match bincode::deserialize_from(&mut stream) {
                Ok(packet) => { tx.send(packet).unwrap() },
                Err(_err) => (), // TODO do something with err
            }
        });

        Self {
            mpsc: rx,
            _p: Default::default(),
        }
    }
}

impl<Packet: 'static + Send> Iterator for Transceiver<Packet, mpsc::Receiver<Packet>> {
    type Item = Packet;

    fn next(&mut self) -> Option<Self::Item> {
        self.mpsc.try_recv().ok()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io;
    use std::sync::Mutex;
    use std::io::Cursor;

    fn poll_some<T, I: Iterator<Item=T>>(iterator: &mut I) -> T {
        loop {
            if let Some(t) = iterator.next() {
                return t;
            }
        }
    }

    #[derive(Debug, Deserialize, PartialEq, Serialize)]
    enum Packet {
        P1,
        P2(u16),
    }

    fn create_p1_p2_42_buffer() -> Vec<u8> {
        let mut buffer = vec![];
        bincode::serialize_into(&mut buffer, &Packet::P1).unwrap();
        bincode::serialize_into(&mut buffer, &Packet::P2(42)).unwrap();
        buffer
    }

    #[test]
    fn encode() {
        #[derive(Clone, Default)]
        struct ThreadSafeBuffer(Arc<Mutex<Vec<u8>>>);

        impl ThreadSafeBuffer {
            fn await_len(&self, len: usize) -> Vec<u8> {
                loop {
                    let vec = self.0.lock().unwrap();

                    if vec.len() >= len {
                        return vec.clone()
                    }
                }
            }
        }

        impl Write for ThreadSafeBuffer {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                self.0.lock().unwrap().write(buf)
            }

            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }

        let vec = create_p1_p2_42_buffer();

        let buffer_w: ThreadSafeBuffer = Default::default();
        let buffer_r = buffer_w.clone();

        {
            let mut transmitter = Transceiver::create_transmitter(buffer_w);

            transmitter.send(Packet::P1);
            transmitter.send(Packet::P2(42));
        }

        let result = buffer_r.await_len(vec.len());

        assert_eq!(&result[..], &vec[..]);
    }

    #[test]
    fn decode() {
        let reader = Cursor::new(create_p1_p2_42_buffer());

        let mut receiver = Transceiver::create_receiver(reader);
        let p1: Packet = poll_some(&mut receiver);
        let p2: Packet = poll_some(&mut receiver);

        assert_eq!(p1, Packet::P1);
        assert_eq!(p2, Packet::P2(42));
        assert!(receiver.next().is_none());
    }

    /// # Actual TCP server test
    ///
    /// Might fail on some systems; the test binds to port 0, which – at least on unix systems –
    /// gives us a random available port. On other systems, it might fail. You can override the port
    /// with the env variable `TAROT_NETWORK_TEST_PORT`.
    #[test]
    fn test_network() {
        use std::net::*;

        let port: u16 = option_env!("TAROT_NETWORK_TEST_PORT")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        // Server
        let server = TcpListener::bind(("127.0.0.1", port)).unwrap();

        // Client
        {
            let client = TcpStream::connect(server.local_addr().unwrap()).unwrap();

            let mut transmitter = Transceiver::create_transmitter(client);
            transmitter.send(Packet::P1);
            transmitter.send(Packet::P2(42));
        }

        // Server again
        {
            let (client_socket, _) = server.accept().unwrap();

            let mut receiver = Transceiver::create_receiver(client_socket);
            let p1: Packet = poll_some(&mut receiver);
            let p2: Packet = poll_some(&mut receiver);

            assert_eq!(p1, Packet::P1);
            assert_eq!(p2, Packet::P2(42));
            assert!(receiver.next().is_none());
        }
    }
}
