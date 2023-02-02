use std::collections::HashMap;
use std::time::Duration;

use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use crate::models::outputs::{Output, OutputParcel};
use crate::models::room::Room;
use crate::models::user::User;

#[derive(Clone, Copy, Default)]
pub struct HubOptions {
    pub alive_check_interval: Option<Duration>,
}
pub struct Hub {
    alive_check_interval: Option<Duration>,
    users: RwLock<HashMap<Uuid, User>>,
    room: RwLock<Room>,
    sender: broadcast::Sender<Output>,
}

impl Hub {
    pub fn new(options: HubOptions) -> Self {
        let (sender, _) = broadcast::channel(16);
        Hub {
            alive_check_interval: options.alive_check_interval,
            users: Default::default(),
            room: Default::default(),
            sender,
        }
    }

    async fn send(&self, output: Output) {
        if self.sender.receiver_count() == 0 {
            return;
        }
        self.users.read().await.keys().for_each(|user_id| {
            self.sender
                .send(OutputParcel::new(*user_id, output.clone()))
                .unwrap();
        });
    }
}
