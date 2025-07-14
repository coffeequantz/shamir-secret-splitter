#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use base64::{engine::general_purpose, Engine as _};
use sharks::{Share, Sharks};
use tauri::command;
use std::convert::TryFrom;

#[command]
fn split(secret: String, shares: usize, threshold: usize) -> Result<Vec<String>, String> {
    if threshold > shares {
        return Err("Threshold cannot be greater than number of shares".into());
    }
    if threshold < 3 {
        return Err("Threshold must be at least 3".into());
    }

    let sharks = Sharks(threshold as u8);
    let dealer = sharks.dealer(secret.as_bytes());

    let parts: Vec<String> = dealer
        .take(shares)
        .map(|share| {
            // Convert share to bytes using built-in conversion
            let bytes: Vec<u8> = Vec::from(&share);
            general_purpose::STANDARD.encode(&bytes)
        })
        .collect();

    Ok(parts)
}

#[command]
fn combine(shares: Vec<String>) -> Result<String, String> {
    if shares.len() < 3 {
        return Err("At least 3 shares are required".into());
    }

    let parsed_shares: Result<Vec<Share>, String> = shares.into_iter().map(|share_str| {
        let bytes = general_purpose::STANDARD
            .decode(&share_str)
            .map_err(|e| e.to_string())?;

        // Convert bytes back to Share using built-in conversion
        Share::try_from(bytes.as_slice())
            .map_err(|e| format!("Failed to parse share: {}", e))
    }).collect();

    let parsed_shares = parsed_shares?;

    let sharks = Sharks(parsed_shares.len() as u8);

    match sharks.recover(&parsed_shares) {
        Ok(secret_bytes) => String::from_utf8(secret_bytes)
            .map_err(|e| format!("Recovered secret is not valid UTF-8: {}", e)),
        Err(e) => Err(format!("Failed to recover secret: {}", e)),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![split, combine])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}