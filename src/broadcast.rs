#![allow(dead_code)]

use std::thread;

pub struct Broadcast {
    channels: Vec<std::sync::mpsc::Sender<String>>,
}

impl Broadcast {
    pub fn new() -> Self {
        return Broadcast {
            channels: Vec::new(),
        };
    }
    pub fn subscribe(&mut self, cb: fn(String)) {
        let (sender, receiver) = std::sync::mpsc::channel::<String>();

        self.channels.push(sender);

        thread::spawn(move || {
            for msg in receiver {
                cb(msg);
            }
        });
    }

    pub fn broadcast(&self, msg: String) {
        self.channels.iter().for_each(|sender| {
            sender.send(msg.clone()).unwrap();
        })
    }
}

#[cfg(test)]
mod test {
    use super::Broadcast;

    #[test]
    fn main() {
        let mut broadcast_channel = Broadcast::new();

        broadcast_channel.subscribe(|msg| {
            println!("[RECEIVER 1]: {msg}");
        });

        broadcast_channel.subscribe(|msg| {
            println!("[RECEIVER 2]: {msg}");
        });

        broadcast_channel.subscribe(|msg| {
            println!("[RECEIVER 3]: {msg}");
        });

        broadcast_channel.subscribe(|msg| {
            println!("[RECEIVER 4]: {msg}");
        });

        broadcast_channel.broadcast("First Message".to_string());
        broadcast_channel.broadcast("Second Message".to_string());
    }
}
