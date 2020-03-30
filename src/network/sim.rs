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

struct SimTxChannel<TMsg> {
    current_step: Rc<Cell<u32>>,
    last_tx_step: Option<u32>,
    latency: u32,
    sender: Sender<SimMsg<TMsg>>
}

impl<TMsg> TxChannel<TMsg> for SimTxChannel<TMsg> {
    fn enqueue(&mut self, msg: TMsg) -> GgResult {
        let delay_write = match self.last_tx_step {
            Some(step) => std::cmp::min(self.current_step.get() - step, self.latency),
            None => self.latency
        };
        self.last_tx_step = Some(self.current_step.get());
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
            last_tx_step: None,
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
            last_tx_step: None,
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

#[test]
fn test_sim_server() {
    _test_sim_server(0, vec![vec![], vec![], vec![]], vec![vec![], vec![], vec![]]);

    _test_sim_server(0, vec![vec![1], vec![], vec![]], vec![vec![], vec![], vec![]]);
    _test_sim_server(0, vec![vec![1, 2, 3], vec![], vec![]], vec![vec![], vec![], vec![]]);
    _test_sim_server(0, vec![vec![1], vec![2], vec![3]], vec![vec![], vec![], vec![]]);

    _test_sim_server(0, vec![vec![], vec![], vec![]], vec![vec![1], vec![], vec![]]);
    _test_sim_server(0, vec![vec![], vec![], vec![]], vec![vec![1, 2, 3], vec![], vec![]]);
    _test_sim_server(0, vec![vec![], vec![], vec![]], vec![vec![1], vec![2], vec![3]]);

    _test_sim_server(0, vec![vec![1, 2, 3], vec![], vec![]], vec![vec![1, 2, 3], vec![], vec![]]);
    _test_sim_server(0, vec![vec![1], vec![2], vec![3]], vec![vec![1], vec![2], vec![3]]);

    _test_sim_server(1, vec![vec![], vec![], vec![]], vec![vec![], vec![], vec![]]);

    _test_sim_server(1, vec![vec![1], vec![], vec![]], vec![vec![], vec![], vec![]]);
    _test_sim_server(1, vec![vec![1, 2, 3], vec![], vec![]], vec![vec![], vec![], vec![]]);
    _test_sim_server(1, vec![vec![1], vec![2], vec![3]], vec![vec![], vec![], vec![]]);

    _test_sim_server(1, vec![vec![], vec![], vec![]], vec![vec![1], vec![], vec![]]);
    _test_sim_server(1, vec![vec![], vec![], vec![]], vec![vec![1, 2, 3], vec![], vec![]]);
    _test_sim_server(1, vec![vec![], vec![], vec![]], vec![vec![1], vec![2], vec![3]]);

    _test_sim_server(1, vec![vec![1, 2, 3], vec![], vec![]], vec![vec![1, 2, 3], vec![], vec![]]);
    _test_sim_server(1, vec![vec![1], vec![2], vec![3]], vec![vec![1], vec![2], vec![3]]);
}

#[cfg(test)]
fn _test_sim_server(latency: u32, client_actions: Vec::<Vec::<u32>>, server_actions: Vec::<Vec::<u32>>) {

    let client_msgs = client_actions.iter().map(|step| step.iter().map(|&msg| ClientMsg::Test(msg)).collect::<Vec::<_>>()).collect::<Vec::<_>>();
    let server_msgs = server_actions.iter().map(|step| step.iter().map(|&msg| ServerMsg::Test(msg)).collect::<Vec::<_>>()).collect::<Vec::<_>>();

    let mut subject = SimServerContainer::new();
    let mut server = subject.get_server(latency);
    let mut client_end = server.connect();
    let mut new_clients = vec![];
    server.get_new_clients(&mut new_clients);
    let server_end = &mut new_clients[0];

    let mut client_msg_buffer = vec![];
    let mut server_msg_buffer = vec![];

    for step in 0..(client_msgs.len() + latency as usize) {
        if step < client_msgs.len() {
            for client_action in client_msgs[step].iter().map(|m| m.clone()) {
                client_end.enqueue(client_action).unwrap();
            }

            for server_action in server_msgs[step].iter().map(|m| m.clone()) {
                server_end.enqueue(server_action).unwrap();
            }
        }

        server_msg_buffer.clear();
        server_end.dequeue(&mut server_msg_buffer).unwrap();

        if step < latency as usize {
            assert_eq!(0, server_msg_buffer.len())
        } else {
            assert_eq!(client_msgs[step - latency as usize].len(), server_msg_buffer.len());
            for i in 0..server_msg_buffer.len() {
                assert_eq!(client_msgs[step - latency as usize][i], server_msg_buffer[i]);
            }
        }

        client_msg_buffer.clear();
        client_end.dequeue(&mut client_msg_buffer).unwrap();

        if step < latency as usize {
            assert_eq!(0, client_msg_buffer.len())
        } else {
            assert_eq!(server_msgs[step - latency as usize].len(), client_msg_buffer.len());
            for i in 0..client_msg_buffer.len() {
                assert_eq!(server_msgs[step - latency as usize][i], client_msg_buffer[i]);
            }
        }

        subject.step();
    }
}