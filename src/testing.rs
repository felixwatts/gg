use std::time::Duration;
use crate::context::TimerService;
use crate::engine::Engine;
use crate::network::sim::SimServerContainer;

pub struct MockContext{
    pub average_delta: Duration,
    pub time_since_start: Duration
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
    pub fn step(&mut self){
        self.time_since_start += self.average_delta;
    }
}

pub struct MockSetup{
    pub context: MockContext,
    network: SimServerContainer,
    pub server_engine: Engine<MockContext>,
    pub client1_engine: Engine<MockContext>,
    // pub client2_engine: Engine<MockContext>
}

impl MockSetup{
    pub fn new(network_latency: u32, is_latency_compensation_enabled: bool) -> MockSetup{
        let mut context = MockContext{
            average_delta: Duration::from_millis(15),
            time_since_start: Duration::from_millis(0)
        };
    
        let network = crate::network::sim::SimServerContainer::new();
        let mut server = network.get_server(network_latency);
        let client1_network = server.connect();
        // let client2_network = server.connect();
    
        let server_engine: Engine::<MockContext> = crate::engine::Engine::new(vec![
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::gorilla::GorillaSystem{is_latency_compensation_enabled}),
            Box::new(crate::system::server::ServerSystem::new(server, is_latency_compensation_enabled).unwrap())
        ], None, &mut context).unwrap();

        let client1_engine: Engine::<MockContext> = crate::engine::Engine::new(vec![
            Box::new(crate::system::physics::PhysicsSystem{}),
            Box::new(crate::system::keyboard::KeyboardSystem{}),
            Box::new(crate::system::client::ClientSystem::new(client1_network))
        ], None, &mut context).unwrap();

        // let client2_engine: Engine::<MockContext> = crate::engine::Engine::new(vec![
        //     Box::new(crate::system::physics::PhysicsSystem{}),
        //     Box::new(crate::system::keyboard::KeyboardSystem{}),
        //     Box::new(crate::system::client::ClientSystem::new(client2_network))
        // ], None, &mut context).unwrap();

        let mut result = MockSetup{
            context,
            network,
            server_engine,
            client1_engine,
            // client2_engine
        };

        // process new clients
        result.step();

        result
    }

    pub fn step(&mut self){
        self.context.step();
        
        self.client1_engine.update(&mut self.context).unwrap();
        // self.client2_engine.update(&mut self.context).unwrap();
        self.server_engine.update(&mut self.context).unwrap();
        self.network.step();
        std::thread::sleep(Duration::from_millis(5));
    }
}

pub fn assert_roughly_eq(name: &'static str, expected: f32, actual: f32) {
    assert!((expected - actual).abs() < 0.0001, "{}: {} != {}", name, expected, actual);
}