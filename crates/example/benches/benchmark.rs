use example::App;
use toolkit::EventLoop;

fn main() {
    divan::main();
}

#[divan::bench]
fn event_loop_init() {
    EventLoop::new(App::default()).unwrap();
}

#[divan::bench]
fn run_main_afk() {
    let mut event_loop = EventLoop::new(App::default()).unwrap();
    event_loop.run().unwrap();
}
