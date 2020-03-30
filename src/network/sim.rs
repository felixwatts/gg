use std::cell::Cell;
use std::sync::mpsc::Receiver;
use std::sync::mpsc::Sender;
use crate::network::Server;
use crate::network::ServerMsg;
use crate::network::ClientMsg;
use crate::network::RxChannel;
use crate::network::TxChannel;
use crate::err::GgResult;
use std::rc::Rc;

enum SimMsg<T> {
    Msg(T),
    Delay(u32)
}

// #[test]
// fn test_sim_channel_zero_latency() {
//     let mut subject = SimChannel::<u8>::new(0);
//     let mut buffer = vec![0u8];
//     subject.enqueue(1).unwrap();
//     subject.enqueue(2).unwrap();
//     subject.enqueue(3).unwrap();
//     subject.dequeue(&mut buffer).unwrap();
//     assert_eq!(3, buffer.len());
//     assert_eq!(1, buffer[0]);
//     assert_eq!(2, buffer[1]);
//     assert_eq!(3, buffer[2]);

//     subject.step();

//     subject.dequeue(&mut buffer).unwrap();
//     assert_eq!(0, buffer.len());
// }

// #[test]
// fn sim_channel_max_latency() {
//     let mut subject = SimChannel::<u8>::new(1);
//     let mut buffer = vec![0u8];

//     subject.step();
//     subject.dequeue(&mut buffer).unwrap();

//     subject.step();
//     subject.enqueue(1).unwrap();
//     subject.dequeue(&mut buffer).unwrap();

//     subject.step();
//     subject.dequeue(&mut buffer).unwrap();

//     assert_eq!(1, buffer.len());
//     assert_eq!(1, buffer[0]);
// }

// #[test]
// fn sim_channel_following_latency() {
//     let mut subject = SimChannel::<u8>::new(3);
//     let mut buffer = vec![0u8];

//     subject.enqueue(1).unwrap();
//     subject.dequeue(&mut buffer).unwrap();
//     assert_eq!(0, buffer.len());

//     subject.step();

//     subject.enqueue(2).unwrap();
//     subject.enqueue(3).unwrap();
//     subject.dequeue(&mut buffer).unwrap();
//     assert_eq!(0, buffer.len());

//     subject.step();

//     subject.dequeue(&mut buffer).unwrap();
//     assert_eq!(0, buffer.len());

//     subject.step();
    
//     subject.dequeue(&mut buffer).unwrap();
//     assert_eq!(1, buffer.len());
//     assert_eq!(1, buffer[0]);

//     subject.step();
    
//     subject.dequeue(&mut buffer).unwrap();
//     assert_eq!(2, buffer.len());
//     assert_eq!(2, buffer[0]);
//     assert_eq!(3, buffer[1]);
// }

struct SimTxChannel<TMsg> {
    current_step: Rc<Cell<u32>>,
    last_tx_step: u32,
    latency: u32,
    sender: Sender<SimMsg<TMsg>>
}

impl<TMsg> TxChannel<TMsg> for SimTxChannel<TMsg> {
    fn enqueue(&mut self, msg: TMsg) -> GgResult {
        let delay_write = std::cmp::min(self.current_step.get() - self.last_tx_step, self.latency);
        self.last_tx_step = self.current_step.get();
        self.sender.send(SimMsg::<TMsg>::Delay(delay_write))?;
        self.sender.send(SimMsg::<TMsg>::Msg(msg))?;
        Ok(())
    }
}

struct SimRxChannel<TMsg> {
    current_step: Rc<Cell<u32>>,
    next_read_step: u32,
    receiver: Receiver<SimMsg<TMsg>>
}

impl<TMsg> RxChannel<TMsg> for SimRxChannel<TMsg> {
    fn dequeue(&mut self, buffer: &mut Vec::<TMsg>) -> GgResult {
        buffer.clear();
        loop {
            if self.next_read_step > self.current_step.get() { 
                break; 
            }
            match self.receiver.try_recv() {
                Ok(front) => {
                    match front {
                        SimMsg::<TMsg>::Delay(delay) => {
                                self.next_read_step = self.current_step.get() + delay;
                            },
                        SimMsg::<TMsg>::Msg(msg) => {
                                buffer.push(msg);
                                // loop
                            }
                        }
                    },
                Err(_) => break
            }
        }
        Ok(())
    }
}

pub struct SimNetworkEnd<TTx, TRx> {
    tx: SimTxChannel<TTx>,
    rx: SimRxChannel<TRx>
}

impl<TTx, TRx> TxChannel<TTx> for SimNetworkEnd<TTx, TRx>  {
    fn enqueue(&mut self, msg: TTx) -> GgResult {
        self.tx.enqueue(msg)
    }
}

impl<TTx, TRx> RxChannel<TRx> for SimNetworkEnd<TTx, TRx> {
    fn dequeue(&mut self, buffer: &mut Vec::<TRx>) -> GgResult {
        self.rx.dequeue(buffer)
    }
}

pub struct SimServerContainer{
    current_step: Rc<Cell<u32>>,
}

impl<'a> SimServerContainer{
    pub fn new() -> SimServerContainer{
        SimServerContainer{
            current_step: Rc::new(Cell::new(0u32))
        }
    }

    pub fn get_server(&self, latency: u32) -> SimServer{
        SimServer::new(latency, Rc::clone(&self.current_step))
    }

    pub fn step(&mut self) {
        self.current_step.set(self.current_step.get() + 1);
    }
}

pub struct SimServer {
    current_step: Rc<Cell<u32>>,
    latency: u32,
    new_clients: Vec::<SimNetworkEnd<ServerMsg, ClientMsg>>
}

impl SimServer {
    pub fn new(latency: u32, current_step: Rc<Cell<u32>>) -> SimServer {
        SimServer{
            current_step,
            latency,
            new_clients: vec![]
        }
    }

    pub fn connect(&mut self) -> SimNetworkEnd<ClientMsg, ServerMsg> {

        let (client_sender, server_receiver) = std::sync::mpsc::channel();
        let (server_sender, client_receiver) = std::sync::mpsc::channel();

        let client_tx_channel = SimTxChannel{
            current_step: Rc::clone(&self.current_step),
            last_tx_step: 0,
            latency: self.latency,
            sender: client_sender
        };

        let client_rx_channel = SimRxChannel{
            current_step: Rc::clone(&self.current_step),
            next_read_step: 0,
            receiver: client_receiver
        };

        let client_end = SimNetworkEnd{
            tx: client_tx_channel,
            rx: client_rx_channel
        };

        let server_tx_channel = SimTxChannel{
            current_step: Rc::clone(&self.current_step),
            last_tx_step: 0,
            latency: self.latency,
            sender: server_sender
        };

        let server_rx_channel = SimRxChannel{
            current_step: Rc::clone(&self.current_step),
            next_read_step: 0,
            receiver: server_receiver
        };

        let server_end = SimNetworkEnd{
            tx: server_tx_channel,
            rx: server_rx_channel
        };

        self.new_clients.push(server_end);

        client_end
    }
}

impl<'a> Server<SimNetworkEnd<ServerMsg, ClientMsg>> for SimServer {
    fn get_new_clients(&mut self, buffer: &mut Vec<SimNetworkEnd<ServerMsg, ClientMsg>>) {
        buffer.clear();
        buffer.extend(self.new_clients.drain(..));
        self.new_clients.clear();
    }
}