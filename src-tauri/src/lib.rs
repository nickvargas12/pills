// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
// use serde::{Deserialize, Serialize};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Clone)]
struct PlayerData {
    buyins: i32,
    final_amount: f64,
}

static mut BUYIN_AMOUNT: f64 = 0.0;

#[tauri::command]
fn set_buyin(amount: f64) {
    unsafe {
        BUYIN_AMOUNT = amount;
    }
}

#[tauri::command]
fn increment_buyins(player_name: &str) -> HashMap<String, PlayerData> {
    unsafe {
        if let Some(players) = PLAYERS.as_mut() {
            if let Some(player) = players.get_mut(player_name) {
                player.buyins += 1;
            }
        }
        PLAYERS.as_ref().unwrap().clone()
    }
}

#[tauri::command]
fn get_buyin(number: &str) -> f64 {
    let integer: f64 = match number.parse::<f64>() {
        Ok(num) => num,
        Err(_) => 0.0,
    };
    integer
}

// Create a static state to store the players
static mut PLAYERS: Option<HashMap<String, PlayerData>> = None;

#[tauri::command]
fn list_players() -> HashMap<String, PlayerData> {
    unsafe {
        if PLAYERS.is_none() {
            PLAYERS = Some(HashMap::new());
        }
        PLAYERS.as_ref().unwrap().clone()
    }
}

#[tauri::command]
fn add_player(name: &str) -> HashMap<String, PlayerData> {
    unsafe {
        if PLAYERS.is_none() {
            PLAYERS = Some(HashMap::new());
        }
        PLAYERS.as_mut().unwrap().insert(
            String::from(name),
            PlayerData {
                buyins: 1,
                final_amount: 0.0,
            },
        );
        PLAYERS.as_ref().unwrap().clone()
    }
}

#[tauri::command]
fn calculate_payouts() -> (f64, HashMap<String, f64>, f64) {
    unsafe {
        let players = PLAYERS.as_ref().unwrap();
        let mut payouts = HashMap::new();

        // Calculate total pot
        let total_pot = players
            .iter()
            .map(|(_, data)| data.buyins as f64 * BUYIN_AMOUNT)
            .sum::<f64>();

        // Sum of all final amounts
        let total_final = players
            .iter()
            .map(|(_, data)| data.final_amount)
            .sum::<f64>();

        // Calculate each player's net position
        for (name, data) in players.iter() {
            let player_invested = data.buyins as f64 * BUYIN_AMOUNT;
            let net_position = data.final_amount - player_invested;
            payouts.insert(name.clone(), net_position);
        }

        (total_pot, payouts, total_final - total_pot)
    }
}

#[tauri::command]
fn update_final_amount(player_name: &str, amount: f64) -> HashMap<String, PlayerData> {
    unsafe {
        if let Some(players) = PLAYERS.as_mut() {
            if let Some(player) = players.get_mut(player_name) {
                player.final_amount = amount;
            }
        }
        PLAYERS.as_ref().unwrap().clone()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![
            get_buyin,
            add_player,
            list_players,
            increment_buyins,
            set_buyin,
            calculate_payouts,
            update_final_amount
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
