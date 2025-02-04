use std::collections::VecDeque;
use std::time::Duration;
use std::cell::{Cell, RefCell};
use crate::network::Server;
use crate::network::ServerMsg;
use crate::network::ClientMsg;
use crate::network::RxChannel;
use crate::network::TxChannel;
use crate::err::GgResult;
use std::rc::Rc;

type SimMsg<T> = (T, Duration);

struct SimTxChannel<TMsg> {
    time: Rc<Cell<Duration>>,
    latency: Duration,
    pipe: Rc<RefCell<VecDeque<SimMsg<TMsg>>>>
}

impl<TMsg> TxChannel<TMsg> for SimTxChannel<TMsg> {
    fn enqueue(&mut self, msg: TMsg) -> GgResult {
        let arrival_time = self.time.get() + self.latency;
        self.pipe.borrow_mut().push_back((msg, arrival_time));
        Ok(())
    }
}

struct SimRxChannel<TMsg> {
    time: Rc<Cell<Duration>>,
    pipe: Rc<RefCell<VecDeque<SimMsg<TMsg>>>>
}

impl<TMsg> RxChannel<TMsg> for SimRxChannel<TMsg> {
    fn dequeue(&mut self, buffer: &mut Vec::<TMsg>) -> GgResult {
        buffer.clear();

        let current_time = self.time.get();
        
        loop {

            let mut pipe = self.pipe.borrow_mut();
            let front = pipe.get(0);

            match front {
                Some(msg) => {
                    if msg.1 > current_time {
                        return Ok(())
                    }

                    buffer.push(pipe.pop_front().unwrap().0);
                },
                None => return Ok(())
            }
        }
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

pub struct SimServer {
    time: Rc<Cell<Duration>>,
    latency: Duration,
    new_clients: Vec::<SimNetworkEnd<ServerMsg, ClientMsg>>
}

impl SimServer {
    pub fn new(latency: Duration, time: Rc<Cell<Duration>>) -> SimServer {
        SimServer{
            time,
            latency,
            new_clients: vec![]
        }
    }

    pub fn connect(&mut self) -> SimNetworkEnd<ClientMsg, ServerMsg> {

        let pipe_up = Rc::new(RefCell::from(VecDeque::<SimMsg::<ClientMsg>>::new()));
        let pipe_down = Rc::new(RefCell::from(VecDeque::<SimMsg::<ServerMsg>>::new()));

        let client_tx_channel = SimTxChannel{
            time: Rc::clone(&self.time),
            latency: self.latency,
            pipe: Rc::clone(&pipe_up)
        };

        let client_rx_channel = SimRxChannel{
            time: Rc::clone(&self.time),
            pipe: Rc::clone(&pipe_down)
        };

        let client_end = SimNetworkEnd{
            tx: client_tx_channel,
            rx: client_rx_channel
        };

        let server_tx_channel = SimTxChannel{
            time: Rc::clone(&self.time),
            latency: self.latency,
            pipe: Rc::clone(&pipe_down)
        };

        let server_rx_channel = SimRxChannel{
            time: Rc::clone(&self.time),
            pipe: Rc::clone(&pipe_up)
        };

        let server_end = SimNetworkEnd{
            tx: server_tx_channel,
            rx: server_rx_channel
        };

        self.new_clients.push(server_end);

        client_end
    }
}

impl Server<SimNetworkEnd<ServerMsg, ClientMsg>> for SimServer {
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
fn _test_sim_server(latency: u64, client_actions: Vec::<Vec::<u32>>, server_actions: Vec::<Vec::<u32>>) {

    let client_msgs = client_actions.iter().map(|step| step.iter().map(|&msg| ClientMsg::Test(msg)).collect::<Vec::<_>>()).collect::<Vec::<_>>();
    let server_msgs = server_actions.iter().map(|step| step.iter().map(|&msg| ServerMsg::Test(msg)).collect::<Vec::<_>>()).collect::<Vec::<_>>();

    let time = Rc::new(Cell::new(Duration::from_millis(0u64)));
    let mut server = SimServer::new(Duration::from_millis(latency), Rc::clone(&time));

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

        time.set(time.get() + Duration::from_millis(1));
    }
}