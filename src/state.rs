use recs::EntityId;
use recs::Ecs;

pub struct State {
    pub ecs: Ecs,
    pub rx_queue: Option<EntityId>,
    pub tx_queue: Option<EntityId>
}