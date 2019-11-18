use crate::sampler::Sampler;
use crate::sequencer::parser::Document;

use jack::{
    AudioOut,
    ClientOptions,
    Client,
    ClosureProcessHandler,
    Control,
    ProcessScope,
};

pub fn start_engine (
    mut document: Document,
    mut sampler: Sampler
) {

    let (client, status) = Client::new("sequence", ClientOptions::empty())
        .unwrap_or_else(|e| panic!("Failed to open JACK client: {:?}", e));

    println!("JACK client {}, status {:?}", client.name(), status);

    let mut output = client.register_port("output", AudioOut::default())
        .unwrap_or_else(|e| panic!("Failed to register output port: {:?}", e));

    let handler = ClosureProcessHandler::new(
        move |_: &Client, scope: &ProcessScope| {
            let cycle_times = scope.cycle_times().unwrap();
            println!("{:#?}", cycle_times);
            for element in output.as_mut_slice(scope) {
                *element = 0.0;
            }
            Control::Continue
        }
    );

    client.activate_async((), handler)
        .unwrap_or_else(|e| panic!("Failed to active JACK client: {:?}", e));

    println!("Activated JACK client");

}

/*

pub struct Engine {
    output: Port<AudioOut>
}

impl NotificationHandler for Engine {

    fn thread_init (&self, _client: &Client) {}

    fn shutdown (&mut self, _status: ClientStatus, _reason: &str) {}

    fn freewheel (&mut self, _client: &Client, _enabled: bool) {}

    fn buffer_size (&mut self, _client: &Client, _size: Frames) -> Control {
        Control::Continue
    }

    fn sample_rate (&mut self, _client: &Client, _rate: Frames) -> Control {
        Control::Continue
    }

    fn port_registration (
        &mut self, _client: &Client, _id: PortId, _reg: bool
    ) {}

    fn port_rename (
        &mut self, _client: &Client, _id: PortId, _old: &str, _new: &str
    ) -> Control {
        Control::Continue
    }

    fn ports_connected (
        &mut self, _client: &Client, _id1: PortId, _id2: PortId, _conn: bool
    ) {}

    fn graph_reorder (&mut self, _client: &Client) -> Control {
        Control::Continue
    }

    fn xrun (&mut self, _client: &Client) -> Control {
        Control::Continue
    }

    fn latency (&mut self, _client: &Client, _mode: LatencyType) {}

}

impl ProcessHandler for Engine {
    fn process (
        &mut self,
        _client: &Client,
        process_scope: &ProcessScope
    ) -> Control{
        Control::Continue
    }
}

*/
