use std::{
    collections::HashMap,
    rc::Rc,
    sync::{Arc, Mutex, OnceLock},
};

// static USER_STATES: Mutex<HashMap<u64, UserState>> = Mutex::new(HashMap::new());

struct UserState {
    space_traders_token: Option<String>,
    view_token: Option<String>,
}

pub fn save_view_token(user_id: u64, token: &str) {
    let mut user_states = get_user_states().lock().unwrap();

    let entry = user_states.entry(user_id).or_insert(UserState {
        space_traders_token: None,
        view_token: None,
    });

    entry.space_traders_token = Some(token.to_string());

    // *entry.view_token = token;
}

pub fn get_view_token(user_id: u64) -> Option<String> {
    get_user_states()
        .lock()
        .unwrap()
        .get(&user_id)?
        .view_token
        .clone()
}

fn get_user_states() -> &'static Mutex<HashMap<u64, UserState>> {
    static STATES: OnceLock<Mutex<HashMap<u64, UserState>>> = OnceLock::new();

    STATES.get_or_init(|| {
        let map: HashMap<u64, UserState> = HashMap::new();

        Mutex::new(map)
    })
}
