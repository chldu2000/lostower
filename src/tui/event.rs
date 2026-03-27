use crossterm::event::{
    self, Event as CrosstermEvent, KeyEvent, KeyEventKind, MouseEvent,
};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub enum Event {
    Tick,
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
}

pub struct EventHandler {
    receiver: mpsc::Receiver<Event>,
}

impl EventHandler {
    pub fn new(tick_rate: Duration) -> Self {
        let (sender, receiver) = mpsc::channel();

        let event_sender = sender.clone();
        thread::spawn(move || {
            let mut last_tick = Instant::now();
            loop {
                let timeout = tick_rate
                    .checked_sub(last_tick.elapsed())
                    .unwrap_or(tick_rate);

                if event::poll(timeout).expect("no events available") {
                    match event::read().expect("unable to read event") {
                        CrosstermEvent::Key(e) if e.kind == KeyEventKind::Press => {
                            event_sender.send(Event::Key(e))
                        }
                        CrosstermEvent::Mouse(e) => event_sender.send(Event::Mouse(e)),
                        CrosstermEvent::Resize(w, h) => event_sender.send(Event::Resize(w, h)),
                        _ => Ok(()),
                    }
                    .expect("failed to send terminal event");
                }

                if last_tick.elapsed() >= tick_rate {
                    event_sender.send(Event::Tick).expect("failed to send tick event");
                    last_tick = Instant::now();
                }
            }
        });

        Self { receiver }
    }

    pub fn next(&self) -> Result<Event, mpsc::RecvError> {
        self.receiver.recv()
    }
}
