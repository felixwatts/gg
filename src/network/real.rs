use crate::network::Server;
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
use std::sync::mpsc::channel;
use std::sync::Arc;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::io::Read;
use byteorder::{ReadBytesExt, WriteBytesExt};

pub struct RealNetwork<TTx, TRx>{
    #[allow(dead_code)]
    tcp_stream: TcpStream,
    is_closed: Arc<AtomicBool>,
    tx_q_out: Sender<TTx>,
    rx_q_in: Receiver<TRx>,
    #[allow(dead_code)]
    tx_thread: JoinHandle<GgResult>,
    #[allow(dead_code)]
    rx_thread: JoinHandle<GgResult>,
    
    phantom1: PhantomData<TTx>,
    phantom2: PhantomData<TRx>,
}

fn rx_loop<TRx>(
    mut tcp_stream: TcpStream, 
    rx_q_in: Sender<TRx>
) -> GgResult where TRx: DeserializeOwned + std::fmt::Debug {
    let mut buffer = vec![0u8; 2048];
    loop {
        let msg_length = tcp_stream.read_u32::<byteorder::BigEndian>()?;
        let msg_buffer = &mut buffer[0..msg_length as usize];
        tcp_stream.read_exact(msg_buffer)?;
        let msg: TRx = serde_cbor::from_slice(msg_buffer)?;

        #[cfg(debug)]
        println!("<-- {:?} {}", &msg, msg_length);

        rx_q_in.send(msg)?;
    }
}

fn tx_loop<TTx>(
    mut tcp_stream: TcpStream, 
    tx_q_out: Receiver<TTx>
) -> GgResult where TTx: Serialize + std::fmt::Debug {
    loop {
        let msg = tx_q_out.recv()?;
        let msg_buffer = serde_cbor::to_vec(&msg)?;
        let msg_length = msg_buffer.len();
        tcp_stream.write_u32::<byteorder::BigEndian>(msg_length as u32)?;
        serde_cbor::to_writer(&mut tcp_stream, &msg)?;

        #[cfg(debug)]
        println!("--> {:?} {}", &msg, msg_length);
    }
}

impl<TTx, TRx> RealNetwork<TTx, TRx> 
    where 
        TTx: 'static + Send + Serialize + std::fmt::Debug, 
        TRx: 'static + Send + DeserializeOwned + std::fmt::Debug{

    pub fn new(tcp_stream: TcpStream) -> GgResult<RealNetwork<TTx, TRx>> {

        tcp_stream.set_nodelay(true)?;

        let (tx_q_out, tx_q_in) = channel::<TTx>();
        let (rx_q_out, rx_q_in) = channel::<TRx>();

        let is_closed = Arc::new(AtomicBool::new(false));
        let tx_is_closed = is_closed.clone();
        let rx_is_closed = is_closed.clone();

        let tx_stream = tcp_stream.try_clone()?;
        let rx_stream = tcp_stream.try_clone()?;

        let tx_thread = std::thread::spawn(move || {
            let result = tx_loop(tx_stream, tx_q_in);
            
            // #[cfg(debug)]
            println!("tx thread loop exited: {:?}", result);
            
            tx_is_closed.store(true, Ordering::Relaxed);
            result
        });

        let rx_thread = std::thread::spawn(move || {
            let result = rx_loop(rx_stream, rx_q_out);

            // #[cfg(debug)]
            println!("rx thread loop exited: {:?}", result);

            rx_is_closed.store(true, Ordering::Relaxed);
            result
        });

        Ok(RealNetwork{
            tcp_stream,
            is_closed,
            tx_q_out,
            rx_q_in,
            tx_thread,
            rx_thread,
            phantom1: PhantomData{},
            phantom2: PhantomData{}
        })
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

    let mut server = RealServer::new().unwrap();

    let client_1_stream = TcpStream::connect("127.0.0.1:9001").unwrap();
    let mut client_1 = RealNetwork::<ClientMsg, ServerMsg>::new(client_1_stream).unwrap();

    let client_2_stream = TcpStream::connect("127.0.0.1:9001").unwrap();
    let mut client_2 = RealNetwork::<ClientMsg, ServerMsg>::new(client_2_stream).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(50));

    let mut new_clients = vec![];
    server.get_new_clients(&mut new_clients);

    assert_eq!(2, new_clients.len());

    client_1.enqueue(ClientMsg::Test(1)).unwrap();
    client_2.enqueue(ClientMsg::Test(2)).unwrap();
    client_1.enqueue(ClientMsg::Test(3)).unwrap();
    client_2.enqueue(ClientMsg::Test(4)).unwrap();
    client_1.enqueue(ClientMsg::Test(5)).unwrap();
    client_2.enqueue(ClientMsg::Test(6)).unwrap();

    std::thread::sleep(std::time::Duration::from_millis(50));

    let mut server_msg_buffer = vec![];
    
    (&mut new_clients[0]).dequeue(&mut server_msg_buffer).unwrap();
    assert_eq!(3, server_msg_buffer.len());
    assert_eq!(ClientMsg::Test(1), server_msg_buffer[0]);
    assert_eq!(ClientMsg::Test(3), server_msg_buffer[1]);
    assert_eq!(ClientMsg::Test(5), server_msg_buffer[2]);

    &new_clients[1].dequeue(&mut server_msg_buffer).unwrap();
    assert_eq!(3, server_msg_buffer.len());
    assert_eq!(ClientMsg::Test(2), server_msg_buffer[0]);
    assert_eq!(ClientMsg::Test(4), server_msg_buffer[1]);
    assert_eq!(ClientMsg::Test(6), server_msg_buffer[2]);
}

pub struct RealServer {
    #[allow(dead_code)]
    listen_thread: JoinHandle<GgResult>,
    new_client_recv: Receiver<RealNetwork<ServerMsg, ClientMsg>>
}

impl RealServer {
    pub fn new() -> GgResult<RealServer> {
        let (new_client_send, new_client_recv) = channel();
        let listener = TcpListener::bind("0.0.0.0:9001")?;
        let listen_thread = std::thread::spawn(move || {
            for stream in listener.incoming() {
                let client = RealNetwork::<ServerMsg, ClientMsg>::new(stream.unwrap()).unwrap();
                new_client_send.send(client)?;
            }

            Ok(())
        });

        println!("ggs is listening on port 9001");

        Ok(RealServer {
            listen_thread,
            new_client_recv
        })
    }
}

impl Server<RealNetwork<ServerMsg, ClientMsg>> for RealServer {
    fn get_new_clients(&mut self, buffer: &mut Vec<RealNetwork<ServerMsg, ClientMsg>>) {
        buffer.clear();
        buffer.extend(self.new_client_recv.try_iter());
    }
}

#[test]
fn test_real_server() {
    unimplemented!();
}