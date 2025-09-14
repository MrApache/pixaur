use example::App;
use toolkit::{headless::HeadlessEventLoop};

fn main() {
    divan::main();
}

#[divan::bench]
fn event_loop_init() {
    HeadlessEventLoop::new(App::default());
}

#[divan::bench]
fn run_event_loop() {
    let mut event_loop = HeadlessEventLoop::new(App::default());
    event_loop.run_logic();
    event_loop.run_draw();
}
