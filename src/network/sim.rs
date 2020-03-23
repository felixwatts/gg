use crate::network::ServerMsg;
use crate::network::ClientMsg;
use crate::network::RxChannel;
use crate::network::TxChannel;
use ggez::GameResult;
use std::collections::VecDeque;

enum SimMsg<T> {
    Msg(T),
    Delay(u32)
}

pub struct SimChannel<TMsg> {
    latency: u32,
    delay_write: u32,
    delay_read: u32,
    queue: VecDeque::<SimMsg<TMsg>>,
    needs_dequeue_before_step: bool
}

impl<TMsg> SimChannel<TMsg> {
    pub fn new(latency: u32) -> SimChannel<TMsg> {
        SimChannel::<TMsg>{
            latency: latency,
            delay_write: latency,
            delay_read: 0,
            queue: VecDeque::<SimMsg<TMsg>>::new(),
            needs_dequeue_before_step: false
        }
    }

    pub fn step(&mut self) {
        if self.needs_dequeue_before_step {
            panic!("after calling step you must call dequeue before calling step again")
        }

        if self.delay_write < self.latency {
            self.delay_write += 1;
        }

        if self.delay_read > 0 {
            self.delay_read -= 1;
        }

        self.needs_dequeue_before_step = true;
    }
}

impl<TMsg> TxChannel<TMsg> for SimChannel<TMsg> {
    fn enqueue(&mut self, msg: TMsg) -> GameResult {
        self.queue.push_back(SimMsg::<TMsg>::Delay(self.delay_write));
        self.queue.push_back(SimMsg::<TMsg>::Msg(msg));
        self.delay_write = 0;
        Ok(())
    }
}

impl<TMsg> RxChannel<TMsg> for SimChannel<TMsg> {
    fn dequeue(&mut self, buffer: &mut Vec::<TMsg>) -> GameResult {
        buffer.clear();

        // loop until there are no more messages to read
        loop {
            // the next message is in flight but has not arrived yet
            // no more messages to read at this time
            if self.delay_read > 0 { break; }

            // No messages are in flight, have any messages arrived?

            match self.queue.pop_front() {
                // There is a message in the channel
                Some(front) => {
                    match front {
                        // It's delay message, set read delay
                        SimMsg::<TMsg>::Delay(delay) => {
                                self.delay_read = delay;
                            },
                        // It's a real message
                        SimMsg::<TMsg>::Msg(msg) => {
                                // read it
                                buffer.push(msg);
                                // there may be more messages to read
                            }
                        }
                    },
                // the channel is empty
                // no more messages to read at this time
                None => break
            }
        }

        self.needs_dequeue_before_step = false;

        Ok(())
    }
}

pub struct SimNetwork{
    up_channel: SimChannel<ClientMsg>,
    down_channel: SimChannel<ServerMsg>
}

impl SimNetwork{
    pub fn new(latency: u32) -> SimNetwork{
        SimNetwork{
            up_channel: SimChannel::<ClientMsg>::new(latency),
            down_channel: SimChannel::<ServerMsg>::new(latency)
        }
    }

    pub fn step(&mut self){
        self.up_channel.step();
        self.down_channel.step();
    }
}

impl TxChannel<ServerMsg> for SimNetwork {
    fn enqueue(&mut self, msg: ServerMsg) -> GameResult {
        self.down_channel.enqueue(msg)
    }
}

impl TxChannel<ClientMsg> for SimNetwork {
    fn enqueue(&mut self, msg: ClientMsg) -> GameResult {
        self.up_channel.enqueue(msg)
    }
}

impl RxChannel<ServerMsg> for SimNetwork {
    fn dequeue(&mut self, buffer: &mut Vec::<ServerMsg>) -> GameResult {
        self.down_channel.dequeue(buffer)
    }
}

impl RxChannel<ClientMsg> for SimNetwork {
    fn dequeue(&mut self, buffer: &mut Vec::<ClientMsg>) -> GameResult {
        self.up_channel.dequeue(buffer)
    }
}

#[test]
fn test_sim_channel_zero_latency() {
    let mut subject = SimChannel::<u8>::new(0);
    let mut buffer = vec![0u8];
    subject.enqueue(1).unwrap();
    subject.enqueue(2).unwrap();
    subject.enqueue(3).unwrap();
    subject.dequeue(&mut buffer).unwrap();
    assert_eq!(3, buffer.len());
    assert_eq!(1, buffer[0]);
    assert_eq!(2, buffer[1]);
    assert_eq!(3, buffer[2]);

    subject.step();

    subject.dequeue(&mut buffer).unwrap();
    assert_eq!(0, buffer.len());
}

#[test]
fn sim_channel_max_latency() {
    let mut subject = SimChannel::<u8>::new(1);
    let mut buffer = vec![0u8];

    subject.step();
    subject.dequeue(&mut buffer).unwrap();

    subject.step();
    subject.enqueue(1).unwrap();
    subject.dequeue(&mut buffer).unwrap();

    subject.step();
    subject.dequeue(&mut buffer).unwrap();

    assert_eq!(1, buffer.len());
    assert_eq!(1, buffer[0]);
}

#[test]
fn sim_channel_following_latency() {
    let mut subject = SimChannel::<u8>::new(3);
    let mut buffer = vec![0u8];

    subject.enqueue(1).unwrap();
    subject.dequeue(&mut buffer).unwrap();
    assert_eq!(0, buffer.len());

    subject.step();

    subject.enqueue(2).unwrap();
    subject.enqueue(3).unwrap();
    subject.dequeue(&mut buffer).unwrap();
    assert_eq!(0, buffer.len());

    subject.step();

    subject.dequeue(&mut buffer).unwrap();
    assert_eq!(0, buffer.len());

    subject.step();
    
    subject.dequeue(&mut buffer).unwrap();
    assert_eq!(1, buffer.len());
    assert_eq!(1, buffer[0]);

    subject.step();
    
    subject.dequeue(&mut buffer).unwrap();
    assert_eq!(2, buffer.len());
    assert_eq!(2, buffer[0]);
    assert_eq!(3, buffer[1]);
}