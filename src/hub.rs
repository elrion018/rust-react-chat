use std::collections::HashMap;
use std::time::Duration;

use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

use crate::models::outputs::{ErrorOutput, Output, OutputParcel};
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
    sender: broadcast::Sender<OutputParcel>,
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

    /**
     * 유저들에게 메세지를 보내는 메소드
     */
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

    /**
     * 특정 유저에게만 메시지를 보내는 메소드
     */
    fn send_targeted(&self, client_id: Uuid, output: Output) {
        if self.sender.receiver_count() > 0 {
            self.sender
                .send(OutputParcel::new(client_id, output))
                .unwrap();
        }
    }

    /**
     * 특정 유저만 무시하고 모두에게 메시지를 보내는 메소드
     */
    async fn send_ignored(&self, ignored_client_id: Uuid, output: Output) {
        if self.sender.receiver_count() == 0 {
            return;
        }

        self.users
            .read()
            .await
            .keys()
            .filter(|user_id| **user_id != ignored_client_id)
            .for_each(|user_id| {
                self.sender
                    .send(OutputParcel::new(*user_id, output.clone()))
                    .unwrap();
            });
    }

    /**
     * 특정 유저에게 에러메시지를 보내는 메소드
     */
    fn send_error(&self, client_id: Uuid, error: ErrorOutput) {
        self.send_targeted(client_id, Output::Error(error));
    }
}
