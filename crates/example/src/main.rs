use example::Root;
use toolkit::{app::App, EventLoop};

fn main() {
    let mut app = App::new();
    app.add_window(Root::default());

    let mut event_loop = EventLoop::new(app).unwrap();
    event_loop.run().unwrap();
}
