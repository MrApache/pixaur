use example::App;
use toolkit::{headless::HeadlessEventLoop, FontHandle};

fn main() {
    divan::main();
}

#[divan::bench]
fn event_loop_init() {
    HeadlessEventLoop::new(App::default());
}

#[divan::bench]
fn run_logic(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            HeadlessEventLoop::new(App::default())
        })
        .bench_values(|mut event_loop| {
            event_loop.run_logic();
        });
}

#[divan::bench]
fn run_draw(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            HeadlessEventLoop::new(App::default())
        })
        .bench_values(|mut event_loop| {
            event_loop.run_draw();
        });
}

#[divan::bench]
fn default_font_load() {
    FontHandle::default();
}
