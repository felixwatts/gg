use crate::network::ClientMsg;
use crate::network::ServerMsg;
use std::thread::JoinHandle;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use std::net::{TcpStream, TcpListener};
use crate::network::RxChannel;
use crate::network::TxChannel;
use std::marker::PhantomData;
use crate::err::GgResult;
use std::convert::TryInto;
use std::sync::mpsc::channel;
use std::sync::Arc;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::atomic::AtomicBool;

pub struct RealNetwork<TTx, TRx>{
    tcp_stream: TcpStream,
    is_closed: Arc<AtomicBool>,
    tx_q_out: Sender<TTx>,
    rx_q_in: Receiver<TRx>,
    tx_thread: JoinHandle<GgResult>,
    rx_thread: JoinHandle<GgResult>,
    
    phantom1: PhantomData<TTx>,
    phantom2: PhantomData<TRx>,
}

fn u32_from_slice(slice: &[u8]) -> Option<u32> {
    let arr: [u8; 4] = slice.try_into().ok()?;
    Some(u32::from_ne_bytes(arr))
}

fn rx_loop<TRx>(
    tcp_stream: TcpStream, 
    rx_q_in: Sender<TRx>
) -> GgResult where TRx: DeserializeOwned {
    loop {
        let msg: TRx = serde_cbor::from_reader(tcp_stream)?;
        rx_q_in.send(msg)?;
    }
}

fn tx_loop<TTx>(
    tcp_stream: TcpStream, 
    tx_q_out: Receiver<TTx>
) -> GgResult where TTx: Serialize {
    loop {
        let msg = tx_q_out.recv()?;
        serde_cbor::to_writer(tcp_stream, &msg)?;
    }
}

impl<TTx, TRx> RealNetwork<TTx, TRx> 
    where 
        TTx: 'static + Send + Serialize, 
        TRx: 'static + Send + DeserializeOwned{

    pub fn new(tcp_stream: TcpStream) -> RealNetwork<TTx, TRx> {

        let (tx_q_out, tx_q_in) = channel::<TTx>();
        let (rx_q_out, rx_q_in) = channel::<TRx>();

        let is_closed = Arc::new(AtomicBool::new(false));
        let is_closed_copy = is_closed.clone();

        let tx_thread = std::thread::spawn(move || {
            match tx_loop(tcp_stream, tx_q_in) {
                Ok(_) => {
                    *is_closed_copy.get_mut() = true;
                    Ok(())
                },
                Err(e) => {
                    *is_closed_copy.get_mut() = true;
                    Err(e)
                }
            }
        });

        let rx_thread = std::thread::spawn(move || {
            match rx_loop(tcp_stream, rx_q_out) {
                Ok(_) => {
                    *is_closed_copy.get_mut() = true;
                    Ok(())
                },
                Err(e) => {
                    *is_closed_copy.get_mut() = true;
                    Err(e)
                }
            }
        });

        RealNetwork{
            tcp_stream,
            is_closed,
            tx_q_out,
            rx_q_in,
            tx_thread,
            rx_thread,
            phantom1: PhantomData{},
            phantom2: PhantomData{}
        }
    }
}

impl<TTx, TRx> TxChannel<TTx> for RealNetwork<TTx, TRx> {
    fn enqueue(&mut self, msg: TTx) -> GgResult{
        match self.is_closed.load(std::sync::atomic::Ordering::Relaxed) {
            true => Err("channel closed".into()),
            false => {
                self.tx_q_out.send(msg)?;
                Ok(())
            }
        }
    }
}

impl<TTx, TRx> RxChannel<TRx> for RealNetwork<TTx, TRx> {
    fn dequeue(&mut self, buffer: &mut Vec::<TRx>) -> GgResult{
        match self.is_closed.load(std::sync::atomic::Ordering::Relaxed) {
            true => Err("channel closed".into()),
            false => {
                buffer.clear();
                buffer.extend(self.rx_q_in.try_iter());
                Ok(())
            }
        }
    }
}

#[test]
fn test_real_network() {
    let listener = TcpListener::bind("127.0.0.1:9001").unwrap();
    let server_thread = std::thread::spawn(move || {
        let clients = vec![];
        for stream in listener.incoming() {
            let client = RealNetwork::<ServerMsg, ClientMsg>::new(stream.unwrap());
            client.enqueue(ServerMsg::Kill(42u64)).unwrap();
            clients.push(client);
        }
    });

    let mut client_stream = TcpStream::connect("127.0.0.1:9001").unwrap();
    let client = RealNetwork::<ClientMsg, ServerMsg>::new(client_stream);

    client.enqueue(ClientMsg::ButtonStateChange([true, true])).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(50));

    let mut buffer = vec![];
    client.dequeue(&mut buffer).unwrap();
}