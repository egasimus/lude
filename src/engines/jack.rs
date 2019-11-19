use std::sync::{Arc, Mutex};
use std::sync::mpsc::channel;
use crate::Document;

use jack::{
    AudioOut,
    ClientOptions,
    Client,
    ClosureProcessHandler,
    Control,
    ProcessScope,
};

pub fn start_jack_engine (mut document: Document) {

    let (client, status) = Client::new("sequence", ClientOptions::empty())
        .unwrap_or_else(|e| panic!("Failed to open JACK client: {:?}", e));
    println!("JACK client {}, status {:?}", client.name(), status);
    let mut output = client.register_port("output", AudioOut::default())
        .unwrap_or_else(|e| panic!("Failed to register output port: {:?}", e));

    // create two channels for back-and-forth via main and jack threads
    let (req_tx, req_rx) = channel();
    let (res_tx, res_rx) = channel::<Arc<Vec<f32>>>();

    // apparently, the ends of the channels that are moved into
    // the closure that runs on the jack callback thread
    // need to be wrapped with some sort of sync primitive
    // all I know is Arc<Mutex<T>>, let's hope it's good enough
    let req = Arc::new(Mutex::new(req_tx));
    let res = Arc::new(Mutex::new(res_rx));
    let async_client = client.activate_async(
        (),
        ClosureProcessHandler::new(move |_: &Client, scope: &ProcessScope| {
            // jack handler requests data based on current time
            {
                req.lock().unwrap().send((
                    scope.last_frame_time(),
                    scope.n_frames(),
                    scope.frames_since_cycle_start()
                ));
            }
            // and waits for main engine loop to responds with
            // rendered data that should be output this  frame
            {
                let arc = res.lock().unwrap().recv().unwrap();
                let data = Arc::try_unwrap(arc)
                    .unwrap_or_else(|_| panic!("Failed to read response data"));
                let mut output_data = output.as_mut_slice(scope);
                for i in 0..1024 {
                    output_data[i] = data[i];
                }
                //output.as_mut_slice(scope).swap_with_slice(data.as_mut_slice());
            }
            Control::Continue
        })
    ).unwrap_or_else(|e| panic!("Failed to active JACK client: {:?}", e));

    println!("client active: {:#?}", &async_client);

    // main loop: waits for data to be requested.
    // generates it based on timing from jack thread,
    // sends it to be used to populate the output buffer.
    // Arc<T> is being used so that the data is not dropped
    // not cool: allocation on every cycle
    loop {
        let (start, size, skip) = req_rx.recv().unwrap();
        let mut output_data = Vec::with_capacity(size as usize);
        for i in 0..1024 {
            output_data.push(i as f32 / 1024 as f32);
        }
        //println!("tick {} {} {}", start, size, skip);
        let arc = Arc::new(output_data);
        res_tx.send(arc);
    }

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

/*

    let mut current_frames = 0;
    let mut current_usecs = 0;
    let mut next_usecs = 0;
    let mut period_usecs = 0.0;
*/

            /*
            let cycle_times = scope.cycle_times().unwrap();
            println!("cycle {:#?}", &cycle_times);
            println!("d current_frames {}", cycle_times.current_frames - current_frames);
            current_frames = cycle_times.current_frames;
            println!("d current_usecs {}", cycle_times.current_usecs - current_usecs);
            current_usecs = cycle_times.current_usecs;
            println!("d next_usecs {}", cycle_times.next_usecs - next_usecs);
            next_usecs = cycle_times.next_usecs;
            println!("d period_usecs {}", cycle_times.period_usecs - period_usecs);
            period_usecs = cycle_times.period_usecs;

            let d_usecs = next_usecs - current_usecs;
            println!("d usecs {}", d_usecs);*/

