use crate::network::ClientMsg;
use crate::network::RxChannel;
use crate::network::ServerMsg;
use crate::network::TxChannel;

pub struct Latency(pub f32);

pub struct Client<TNetwork>(pub TNetwork) where TNetwork: TxChannel<ServerMsg> + RxChannel<ClientMsg>;