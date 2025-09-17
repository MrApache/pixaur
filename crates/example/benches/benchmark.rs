use example::Root;
use toolkit::{app::App, headless::HeadlessEventLoop, FontHandle};

fn main() {
    divan::main();
}

#[divan::bench]
fn event_loop_init() {
    let mut app = App::new();
    app.add_window(Root::default());
    let _ = HeadlessEventLoop::new(app);
}

#[divan::bench]
fn run_logic(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut app = App::new();
            app.add_window(Root::default());
            HeadlessEventLoop::new(app)
        })
        .bench_values(|mut event_loop| {
            event_loop.run_logic();
        });
}

#[divan::bench]
fn run_draw(bencher: divan::Bencher) {
    bencher
        .with_inputs(|| {
            let mut app = App::new();
            app.add_window(Root::default());
            HeadlessEventLoop::new(app)
        })
        .bench_values(|mut event_loop| {
            event_loop.run_draw();
        });
}

#[divan::bench]
fn default_font_load() {
    FontHandle::default();
}
