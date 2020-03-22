use recs::EntityId;
use recs::Ecs;

pub struct State {
    pub ecs: Ecs,
    pub rx_queue: EntityId,
    pub tx_queue: EntityId
}