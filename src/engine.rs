use crate::sampler::Sampler;
use crate::sequencer::Sequencer;

use jack::{
    AudioOut,
    ClientOptions, Client, ClosureProcessHandler,
    Control, ProcessScope,
};

pub fn start_engine (mut sampler: Sampler, mut sequencer: Sequencer) {

    let options = ClientOptions::empty();
    let (client, status) = Client::new("sequence", options)
        .unwrap_or_else(|e| panic!("Failed to open JACK client: {:?}", e));
    println!("JACK client {}, status {:?}", client.name(), status);

    let output = client.register_port("output", AudioOut::default())
        .unwrap_or_else(|e| panic!("Failed to register output port"));

    let handler = ClosureProcessHandler::new(
        move |_: &Client, _: &ProcessScope| {
            Control::Continue
        }
    );

    let active_client = client.activate_async((), handler)
        .unwrap_or_else(|e| panic!("Failed to active JACK client: {:?}", e));
    println!("Activated JACK client");

    sequencer.play();

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
