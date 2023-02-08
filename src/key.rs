use crossterm::event::{self, Event, KeyCode, KeyEvent};
use tokio::{
    sync::mpsc::{self, UnboundedReceiver},
    task,
};

pub const EXIT_KEY: KeyCode = KeyCode::Char('q');

/// A small event handler that reads crossterm input events on a blocking tokio
/// task and forwards them via a mpsc channel to the main task.
/// This can be used to drive the application.
pub struct Keys {
    /// Input keys are sent to this channel.
    pub rx: UnboundedReceiver<KeyCode>,
}

impl Keys {
    pub fn new(exit_key: KeyCode) -> Self {
        let (tx, rx) = mpsc::unbounded_channel();
        task::spawn(async move {
            loop {
                if let Event::Key(KeyEvent { code: key, .. }) = event::read().unwrap() {
                    if let Err(e) = tx.send(key) {
                        eprintln!("{e}");
                        return;
                    }
                    if key == exit_key {
                        return;
                    }
                }
            }
        });
        Self { rx }
    }
}
