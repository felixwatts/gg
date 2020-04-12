use std::cell::Cell;
use std::rc::Rc;
use std::time::Duration;
use crate::context::TimerService;
use crate::engine::Engine;

pub struct MockContext{
    pub average_delta: Duration,
    pub time_since_start: Duration,
    network_time: Rc::<Cell::<Duration>>
}

impl TimerService for MockContext {
    fn average_delta(&self) -> Duration { 
        self.average_delta
    }

    fn time_since_start(&self) -> Duration {
        self.time_since_start
    }
}

impl MockContext {
    pub fn new(average_delta: Duration) -> MockContext {
        MockContext{
            average_delta: average_delta,
            time_since_start: Duration::from_millis(0u64),
            network_time: Rc::new(Cell::new(Duration::from_millis(0u64)))
        }
    }

    pub fn step(&mut self){
        self.time_since_start += self.average_delta;
        self.network_time().set(self.time_since_start());
    }

    fn network_time(&self) -> Rc::<Cell::<Duration>> {
        Rc::clone(&self.network_time)
    }
}

pub struct MockSetup{
    pub context: MockContext,
    pub server_engine: Engine<MockContext>,
    pub client1_engine: Engine<MockContext>,
}

impl MockSetup{
    pub fn new(network_latency: Duration, event_loop_period: Duration, is_latency_compensation_enabled: bool) -> MockSetup{
        let mut context = MockContext::new(event_loop_period);

        let mut server = crate::network::sim::SimServer::new(network_latency, context.network_time());
        let client1_network = server.connect();
    
        let server_engine: Engine::<MockContext> = crate::engine::Engine::new(vec![
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::gorilla::GorillaSystem{is_latency_compensation_enabled}),
            Box::new(crate::system::server::ServerSystem::new(server, is_latency_compensation_enabled).unwrap())
        ], None, &mut context).unwrap();

        let client1_engine: Engine::<MockContext> = crate::engine::Engine::new(vec![
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::keyboard::KeyboardSystem{}),
            Box::new(crate::system::client::ClientSystem::new(client1_network, crate::input::default_key_mapping()))
        ], None, &mut context).unwrap();

        let mut result = MockSetup{
            context,
            server_engine,
            client1_engine
        };

        // process new clients
        result.step();

        result
    }

    pub fn step(&mut self){
        self.client1_engine.update(&mut self.context).unwrap();
        self.server_engine.update(&mut self.context).unwrap();
        self.context.step();
    }
}

pub fn assert_roughly_eq(name: &'static str, expected: f32, actual: f32) {
    assert!((expected - actual).abs() < 0.002, "{}: {} != {}", name, expected, actual);
}