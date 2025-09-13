use panels::App;
use toolkit::EventLoop;

fn main() {
    let mut event_loop = EventLoop::new(App::default()).unwrap();
    event_loop.run().unwrap();
}
