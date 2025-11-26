use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::net::TcpStream;
use std::sync::{Arc, Mutex};

use crate::vault::{Item, Vault, VaultError};

pub fn handle_client(stream: TcpStream, vault: Arc<Mutex<Vault>>) {
    let mut writer = stream.try_clone().expect("failed to clone stream");
    let mut reader = BufReader::new(stream);

    let _ = writer.write_all(b"Welcome to the Vault!\n");
    let _ = writer.flush();

    let mut line = String::new();
    loop {
        line.clear();
        match reader.read_line(&mut line) {
            Ok(0) => {
                return;
            }
            Ok(_) => {
                let input = line.trim();
                if input.is_empty() {
                    let _ = writer.flush();
                    continue;
                }

                let mut parts = input.split_whitespace();
                let response = match parts.next() {
                    Some("PUT") => {
                        let id = parts.next().and_then(|s| s.parse::<u32>().ok());
                        let name = parts.next();
                        let size = parts.next().and_then(|s| s.parse::<u32>().ok());

                        if let (Some(id), Some(name), Some(size)) = (id, name, size) {
                            let item = Item {
                                name: name.to_string(),
                                size,
                            };
                            let mut v = vault.lock().unwrap();
                            match v.put(id, item, 100) {
                                Ok(_) => "OK: item stored\n".to_string(),
                                Err(VaultError::VaultFull) => "ERROR: vault full\n".to_string(),
                                Err(VaultError::CellFull) => "ERROR: cell full\n".to_string(),
                                _ => "ERROR: unknown\n".to_string(),
                            }
                        } else {
                            "ERROR: usage PUT <id> <name> <size>\n".to_string()
                        }
                    }

                    Some("GET") => {
                        if let Some(id_str) = parts.next() {
                            if let Ok(id) = id_str.parse::<u32>() {
                                let v = vault.lock().unwrap();
                                match v.get(id) {
                                    Ok(Some(items)) => items,
                                    Ok(None) => "Cell is empty\n".to_string(),
                                    Err(VaultError::CellNotFound) => {
                                        "ERROR: cell not found\n".to_string()
                                    }
                                    _ => "ERROR: unknown\n".to_string(),
                                }
                            } else {
                                "ERROR: invalid id\n".to_string()
                            }
                        } else {
                            "ERROR: usage GET <id>\n".to_string()
                        }
                    }

                    Some("LIST") => {
                        let v = vault.lock().unwrap();
                        v.list().unwrap_or_else(|| "Vault is empty\n".to_string())
                    }

                    Some("EXIT") => {
                        let _ = writer.write_all(b"BYE\n");
                        let _ = writer.flush();
                        return;
                    }

                    Some("TAKE") => {
                     return 
                    }

                    _ => "ERROR: unknown command\n".to_string(),
                };

                let _ = writer.write_all(response.as_bytes());
                let _ = writer.flush();
            }
            Err(_) => return,
        }
    }
}
