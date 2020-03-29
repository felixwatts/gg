use crate::network::ClientMsg;
use crate::network::RxChannel;
use crate::network::ServerMsg;
use crate::network::TxChannel;
use std::sync::{Arc, Mutex};
use crate::err::GgResult;

enum NetworkEvent{
    ChannelClosed(usize),
    ChannelOpened(usize)
}

struct NewtworkState<TChannel> where TChannel: TxChannel<ServerMsg> + RxChannel<ClientMsg> {
    channels: Vec::<Option<TChannel>>,
    events: Vec::<NetworkEvent>
}

struct NetworkDelta {
    events: Vec::<NetworkEvent>,
    msgs: Vec::<Vec::<ClientMsg>>
}

struct ChannelVec<TChannel> {
    channels: Arc::<Mutex::<Vec::<Option<TChannel>>>>
}

impl<TChannel> ChannelVec<TChannel> {
    pub fn new(capacity: usize) -> ChannelVec<TChannel> {
        ChannelVec{
            channels: Arc::new(Mutex::new((0..capacity).map(|_| None).collect::<Vec::<_>>())),            
        }
    }

    pub fn open_channel(&mut self, channel: TChannel) -> GgResult<usize> {
        let mut channels = self.channels.lock().unwrap();

        let empty_slot = channels.iter().position(|c| match c { Some(_) => false, _ =>  true });

        match empty_slot {
            Some(slot) => {
                channels[slot] = Some(channel);
                Ok(slot)
            },
            None => Err("server is full".into())
        }
    }

    pub fn close_channel(id: usize) -> GgResult {
        unimplemented!();
    }

    pub fn read(buffer: &mut NetworkDelta) -> GgResult {
        unimplemented!();
    }
}

#[test]
fn test_channel_vec() {
    let mut subject = ChannelVec::<String>::new(2);

    subject.add("Channel 1".into()).unwrap();
    subject.add("Channel 1".into()).unwrap();
}