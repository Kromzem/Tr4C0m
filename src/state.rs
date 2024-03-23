use anyhow::{anyhow, Result};
use std::{
    collections::{hash_map::Entry, HashMap, VecDeque},
    rc::Rc,
    sync::{Arc, Mutex, OnceLock},
    time::Instant,
};
use tokio::sync::RwLock;

use crate::views::view::View;

// static USER_STATES: Mutex<HashMap<u64, UserState>> = Mutex::new(HashMap::new());

struct UserState {
    space_traders_token: Option<String>,
    view_token: Option<String>,
    view_stack: VecDeque<Arc<RwLock<View>>>,
}

pub fn save_view_token(user_id: u64, token: &str) {
    // let mut user_states = get_user_states().lock().unwrap();

    // let entry = user_states.entry(user_id).or_insert(UserState {
    //     space_traders_token: None,
    //     view_token: None,
    // });

    // entry.view_token = Some(token.to_string());

    use_user_state(user_id, |state| {
        state.view_token = Some(token.to_string());
    })

    // *entry.view_token = token;
}

pub fn get_view_token(user_id: u64) -> Option<String> {
    use_user_state(user_id, |state| state.view_token.clone())

    // get_user_states()
    //     .lock()
    //     .unwrap()
    //     .get(&user_id)?
    //     .view_token
    //     .clone()
}

pub fn save_space_traders_token(user_id: u64, token: &str) {
    use_user_state(user_id, |state| {
        state.space_traders_token = Some(token.to_string())
    })
}

pub fn get_space_traders_token(user_id: u64) -> Result<String> {
    use_user_state(user_id, |state| state.space_traders_token.clone())
        .ok_or(anyhow!("User {} not logged in yet!", user_id))
}

pub fn get_current_view(user_id: u64) -> Result<Arc<RwLock<View>>> {
    use_user_state(user_id, |state| {
        state.view_stack.back().map(|v| Arc::clone(v))
    })
    .ok_or(anyhow!("User {} has no active view!", user_id))
}

pub fn push_view(user_id: u64, view: View) -> Arc<RwLock<View>> {
    use_user_state(user_id, |state| {
        let view_ref = Arc::new(RwLock::new(view));
        let view_ref_clone = view_ref.clone();

        state.view_stack.push_back(view_ref);

        view_ref_clone
    })
}

pub fn return_to_previous_view(user_id: u64) -> Option<Arc<RwLock<View>>> {
    use_user_state(user_id, |state| {
        state.view_stack.pop_back();

        state.view_stack.back().map(|v| Arc::clone(v))
        //     return Some(view.clone());
        // }

        // None
    })
}

pub fn has_view_before(user_id: u64) -> bool {
    use_user_state(user_id, |state| state.view_stack.len() > 1)
}

pub fn reset_view_stack(user_id: u64) {
    use_user_state(user_id, |state| state.view_stack.clear())
}

fn use_user_state<T, F>(user_id: u64, mut state_logic: F) -> T
where
    F: FnOnce(&mut UserState) -> T,
{
    let now = Instant::now();

    let mut user_states = get_user_states().lock().unwrap();

    let state = user_states.entry(user_id).or_insert(UserState {
        space_traders_token: None,
        view_token: None,
        view_stack: VecDeque::new(),
    });

    let result = state_logic(state);
    println!("State logic took: {}ms", now.elapsed().as_millis());
    result
}

fn get_user_states() -> &'static Mutex<HashMap<u64, UserState>> {
    static STATES: OnceLock<Mutex<HashMap<u64, UserState>>> = OnceLock::new();

    STATES.get_or_init(|| {
        let map: HashMap<u64, UserState> = HashMap::new();

        Mutex::new(map)
    })
}
