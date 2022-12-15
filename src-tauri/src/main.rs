#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

// use core::slice::SlicePattern;
use std::{fs, io::{Read, Write}};

pub mod crypt;

use tauri::{Window};
fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, file_pick, encrypt, decrypt])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
  format!("Hello, {}", name)
}

#[tauri::command]
fn file_pick() -> Vec<String> {
  let r = rfd::FileDialog::new()
      .set_directory("/")
      .pick_files();
  
  match r {
      Some(files) => {
        files.iter()
          .map(|f| f.display().to_string())
          .collect()
      },
      None => { vec![] }
  }
}



#[derive(Clone, serde::Serialize)]
struct Payload {
  index: usize,
  progress: u64,
  total: u64
}

#[tauri::command(async)]
fn encrypt(paths: Vec<&str>, window: Window) {
  for (index, path) in paths.iter().enumerate() {
    let mut buffer = [0u8; 1024];
    let mut file = fs::File::open(*path).unwrap();
    let mut new_file = fs::File::create(format!("{}.encrypt", path)).unwrap();
    let mut progress: u64 = 0;
    let total = fs::metadata(path).unwrap().len();
    let mut p = 0;
    loop {
      let nbytes = file.read(&mut buffer).unwrap();
      progress = progress + nbytes as u64;
      let en_data = crypt::aes256_encrypt(&buffer[..nbytes]);
      new_file.write(en_data.as_slice()).unwrap();

      if (progress * 100 / total) > p || progress == total {
        p = progress * 100 / total;
        window.emit("update-progress", Payload{index, progress, total}).unwrap();
      }

      if nbytes < buffer.len() {
        break;
      }
    }
  }
}

#[tauri::command(async)]
fn decrypt(paths: Vec<&str>, window: Window) {
  for (index, path) in paths.iter().enumerate() {
    let mut buffer = [0u8; 1040];
    let mut file = fs::File::open(*path).unwrap();
    let mut new_file = fs::File::create(format!("{}.decrypt", path)).unwrap();
    let mut progress: u64 = 0;
    let total = fs::metadata(path).unwrap().len();
    let mut p = 0;
    loop {
      let nbytes = file.read(&mut buffer).unwrap();
      progress = progress + nbytes as u64;
      let de_data = crypt::aes256_decrypt(&buffer[..nbytes]);
      new_file.write(de_data.as_slice()).unwrap();

      if (progress * 100 / total) > p || progress == total {
        // println!("{}", progress);
        p = progress * 100 / total;
        window.emit("update-progress", Payload{index, progress, total}).unwrap();
      }

      if nbytes < buffer.len() {
        break;
      }
    }
  }
}