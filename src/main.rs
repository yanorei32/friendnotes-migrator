#![warn(clippy::pedantic, clippy::nursery)]

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::thread;
use std::time::Duration;

#[derive(Deserialize, Debug)]
struct FNUser {
    #[serde(rename(deserialize = "Note"))]
    note: Option<String>,
}

#[derive(Serialize, Debug)]
struct UNReq {
    #[serde(rename(serialize = "targetUserId"))]
    target_user_id: String,
    note: String,
}

fn main() {
    let mut args: Vec<String> = env::args().collect();

    args.remove(0);

    if args.len() != 2 {
        eprintln!("Usage: friendnotes-migrator [authcookie] [JSON file]");
        return;
    }

    let auth = args.remove(0);
    let json = args.remove(0);

    let users: HashMap<String, FNUser> = serde_json::from_reader(BufReader::new(
        File::open(json).expect("Failed to open JSON"),
    ))
    .expect("Failed to parse JSON");

    let notes: HashMap<String, String> = users
        .into_iter()
        .filter(|(_, v)| v.note.is_some())
        .map(|(k, v)| (k, v.note.unwrap()))
        .collect();

    let note_count = notes.len();

    println!(
        "{} note(s) found. (remaining: {} min)",
        note_count,
        note_count / 2,
    );

    notes.into_iter().enumerate().for_each(|(i, (uuid, note))| {
        println!(
            "Process ({}/{}) remaining {}min: {}: {}",
            i + 1,
            note_count,
            (note_count - i) / 2,
            uuid,
            note
        );

        let req = UNReq {
            target_user_id: uuid,
            note,
        };

        let resp = reqwest::blocking::Client::new()
            .post("https://vrchat.com/api/1/userNotes?apiKey=JlE5Jldo5Jibnk5O5hTx6XVqsJu4WJ26")
            .json(&req)
            .header(reqwest::header::USER_AGENT, "friendnotes-migrator 1.0")
            .header(reqwest::header::COOKIE, format!("auth={}", auth))
            .send()
            .expect("Failed to send request");

        println!("APIResnponse: {}", resp.status());

        println!("Wait 30s...");
        thread::sleep(Duration::from_secs(30));
    });
}
