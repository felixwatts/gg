use std::time::Duration;
use crate::network::ClientMsg;
use crate::network::RxChannel;
use crate::network::ServerMsg;
use crate::network::TxChannel;

pub struct Client<TNetwork>where TNetwork: TxChannel<ServerMsg> + RxChannel<ClientMsg> {
    pub network: TNetwork,
    pub latency: Duration
}