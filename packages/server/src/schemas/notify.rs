use crate::db::root::Pool;

use super::{
    friend::get_friend_ids, friend::ScFriend, game::ScGame, invite::ScInvite, message::ScMessage,
    user::get_user_basic, user::ScUserBasic,
};
use juniper::GraphQLObject;
use std::collections::HashMap;
use std::sync::RwLock;
use tokio::sync::broadcast::{self, Receiver, Sender};

#[derive(GraphQLObject, Debug, Clone)]
pub struct ScNotifyMessage {
    new_message: Option<ScMessage>,
    new_game: Option<ScGame>,
    delete_game: Option<i32>,
    new_invite: Option<ScInvite>,
    apply_friend: Option<ScFriend>,
    accept_friend: Option<ScFriend>,
    delete_friend: Option<i32>,
    update_user: Option<ScUserBasic>,
}

impl ScNotifyMessage {
    pub fn new() -> Self {
        ScNotifyMessage {
            new_message: None,
            new_game: None,
            delete_game: None,
            new_invite: None,
            apply_friend: None,
            accept_friend: None,
            delete_friend: None,
            update_user: None,
        }
    }
    pub fn new_message(data: ScMessage) -> ScNotifyMessage {
        let mut msg = ScNotifyMessage::new();
        msg.new_message = Some(data);
        msg
    }
    pub fn new_game(data: ScGame) -> ScNotifyMessage {
        let mut msg = ScNotifyMessage::new();
        msg.new_game = Some(data);
        msg
    }
    pub fn delete_game(data: i32) -> ScNotifyMessage {
        let mut msg = ScNotifyMessage::new();
        msg.delete_game = Some(data);
        msg
    }
    pub fn new_invite(data: ScInvite) -> ScNotifyMessage {
        let mut msg = ScNotifyMessage::new();
        msg.new_invite = Some(data);
        msg
    }
    pub fn apply_friend(data: ScFriend) -> ScNotifyMessage {
        let mut msg = ScNotifyMessage::new();
        msg.apply_friend = Some(data);
        msg
    }
    pub fn accept_friend(data: ScFriend) -> ScNotifyMessage {
        let mut msg = ScNotifyMessage::new();
        msg.accept_friend = Some(data);
        msg
    }
    pub fn delete_friend(data: i32) -> ScNotifyMessage {
        let mut msg = ScNotifyMessage::new();
        msg.delete_friend = Some(data);
        msg
    }
    pub fn update_user(data: ScUserBasic) -> ScNotifyMessage {
        let mut msg = ScNotifyMessage::new();
        msg.update_user = Some(data);
        msg
    }
}

lazy_static! {
    static ref NOTIFY_MAP: RwLock<HashMap<i32, Sender<ScNotifyMessage>>> = {
        let m = HashMap::new();
        RwLock::new(m)
    };
}

pub fn notify(user_id: i32, msg: ScNotifyMessage) {
    let map = NOTIFY_MAP.read().unwrap();
    map.get(&user_id).and_then(|tx| tx.send(msg).ok());
}

pub fn notify_ids(ids: Vec<i32>, msg: ScNotifyMessage) {
    let map = NOTIFY_MAP.read().unwrap();
    for user_id in ids {
        map.get(&user_id).and_then(|tx| tx.send(msg.clone()).ok());
    }
}

pub fn notify_all(msg: ScNotifyMessage) {
    let map = NOTIFY_MAP.read().unwrap();
    for (user_id, _) in map.iter() {
        map.get(&user_id).and_then(|tx| tx.send(msg.clone()).ok());
    }
}

pub fn get_receiver(user_id: i32) -> Receiver<ScNotifyMessage> {
    NOTIFY_MAP
        .write()
        .unwrap()
        .entry(user_id)
        .or_insert_with(|| {
            log::debug!("{} is online", user_id);
            broadcast::channel(5).0
        })
        .subscribe()
}

pub fn has_user(user_id: i32) -> bool {
    let map = NOTIFY_MAP.read().unwrap();
    map.contains_key(&user_id)
}

pub fn check_user(pool: &Pool) {
    let mut ids = Vec::new();

    // Temporary value release write lock
    NOTIFY_MAP.write().unwrap().retain(|user_id, tx| {
        let is_ok = tx.receiver_count() > 0;
        if !is_ok {
            log::debug!("{} is offline", user_id);
            ids.push(*user_id);
        }
        is_ok
    });

    let conn = pool.get().unwrap();
    for id in ids {
        notify_ids(
            get_friend_ids(&conn, id),
            ScNotifyMessage::update_user(get_user_basic(&conn, id)),
        );
    }
}