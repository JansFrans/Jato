use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc;
use std::fs;

use crate::bot::{Socks5Config};
use crate::bot_state::{BotCommand, BotState, BotStatus, CmdSender};
use crate::events::{WsEvent, WsTx};
use crate::items::ItemsDat;
use serde::{Serialize, Deserialize};

// --- STRUKTUR AUTO-SAVE KE DISK ---
#[derive(Serialize, Deserialize, Clone)]
pub struct SavedProxy {
    pub addr: String,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SavedBot {
    pub is_ltoken: bool,
    pub username: String,
    pub credential: String, // Bisa berisi Password atau Ltoken string
    pub proxy: Option<SavedProxy>,
}
// ----------------------------------

pub struct BotEntry {
    pub username:         String,
    pub stop_flag:        Arc<AtomicBool>,
    pub state:            Arc<RwLock<BotState>>,
    pub cmd_tx:           CmdSender,
}

pub struct BotManager {
    next_id:   u32,
    pub bots:  HashMap<u32, BotEntry>,
    pub saved_bots: HashMap<u32, SavedBot>, // Menyimpan referensi akun untuk di-save
    pub items_dat: Arc<ItemsDat>,
    pub ws_tx: WsTx,
}

#[derive(serde::Serialize)]
pub struct BotInfo {
    pub id:       u32,
    pub username: String,
    pub status:  String,
    pub world:   String,
    pub pos_x:   f32,
    pub pos_y:   f32,
    pub gems:    i32,
    pub ping_ms: u32,
}

impl BotManager {
    pub fn new(ws_tx: WsTx) -> Self {
        let mut mgr = Self { 
            next_id: 0, 
            bots: HashMap::new(), 
            saved_bots: HashMap::new(),
            items_dat: Arc::new(ItemsDat::load()), 
            ws_tx 
        };
        // Muat otomatis semua bot dari file saat server menyala!
        mgr.load_from_disk();
        mgr
    }

    // --- FUNGSI I/O DISK ---
    fn save_to_disk(&self) {
        let list: Vec<SavedBot> = self.saved_bots.values().cloned().collect();
        if let Ok(json) = serde_json::to_string_pretty(&list) {
            let _ = fs::write("accounts.json", json);
        }
    }

    fn load_from_disk(&mut self) {
        if let Ok(data) = fs::read_to_string("accounts.json") {
            if let Ok(list) = serde_json::from_str::<Vec<SavedBot>>(&data) {
                println!("[System] Auto-loading {} bots dari accounts.json...", list.len());
                for saved in list {
                    // Reconstruct proxy configuration
                    let proxy = saved.proxy.clone().and_then(|p| {
                        p.addr.parse().ok().map(|proxy_addr| Socks5Config {
                            proxy_addr,
                            username: p.username,
                            password: p.password,
                        })
                    });

                    // Jalankan bot tanpa memicu save file berulang-ulang
                    if saved.is_ltoken {
                        self.spawn_ltoken_internal(saved.credential.clone(), proxy, false);
                    } else {
                        self.spawn_internal(saved.username.clone(), saved.credential.clone(), proxy, false);
                    }
                }
            }
        }
    }
    // -----------------------

    pub fn spawn(&mut self, username: String, password: String, proxy: Option<Socks5Config>) -> u32 {
        self.spawn_internal(username, password, proxy, true)
    }

    fn spawn_internal(&mut self, username: String, password: String, proxy: Option<Socks5Config>, save_to_disk: bool) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let stop_flag   = Arc::new(AtomicBool::new(false));
        let stop_clone  = stop_flag.clone();
        let uname       = username.clone();
        let pass        = password.clone();

        // 1. Simpan konfigurasi ke meJato
        let saved_config = SavedBot {
            is_ltoken: false,
            username: username.clone(),
            credential: password.clone(),
            proxy: proxy.as_ref().map(|p| SavedProxy {
                addr: p.proxy_addr.to_string(),
                username: p.username.clone(),
                password: p.password.clone(),
            })
        };
        self.saved_bots.insert(id, saved_config);
        
        // 2. Tulis ke file JSON jika flag-nya aktif
        if save_to_disk {
            self.save_to_disk();
        }

        let state = Arc::new(RwLock::new(BotState {
            status: BotStatus::Connecting,
            ..Default::default()
        }));
        let state_clone = state.clone();

        let (cmd_tx, cmd_rx) = mpsc::channel::<BotCommand>();

        let items_dat = self.items_dat.clone();
        let ws_tx_clone = self.ws_tx.clone();

        std::thread::spawn(move || {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut bot = crate::bot::Bot::new(&uname, &pass, proxy, state_clone, cmd_rx, items_dat, id, Some(ws_tx_clone));
                bot.run(stop_clone);
            })) {
                Ok(_)  => println!("[Bot:{id}] Stopped."),
                Err(_) => println!("[Bot:{id}] Crashed."),
            }
        });

        self.bots.insert(id, BotEntry { username: username.clone(), stop_flag, state, cmd_tx });
        let _ = self.ws_tx.send(WsEvent::BotAdded { bot_id: id, username });
        id
    }

    pub fn spawn_ltoken(&mut self, ltoken_str: String, proxy: Option<Socks5Config>) -> u32 {
        self.spawn_ltoken_internal(ltoken_str, proxy, true)
    }

    fn spawn_ltoken_internal(&mut self, ltoken_str: String, proxy: Option<Socks5Config>, save_to_disk: bool) -> u32 {
        let id = self.next_id;
        self.next_id += 1;

        let stop_flag  = Arc::new(AtomicBool::new(false));
        let stop_clone = stop_flag.clone();

        let saved_config = SavedBot {
            is_ltoken: true,
            username: String::new(),
            credential: ltoken_str.clone(),
            proxy: proxy.as_ref().map(|p| SavedProxy {
                addr: p.proxy_addr.to_string(),
                username: p.username.clone(),
                password: p.password.clone(),
            })
        };
        self.saved_bots.insert(id, saved_config);
        if save_to_disk {
            self.save_to_disk();
        }

        let state = Arc::new(RwLock::new(BotState {
            status: BotStatus::Connecting,
            ..Default::default()
        }));
        let state_clone = state.clone();

        let (cmd_tx, cmd_rx) = mpsc::channel::<BotCommand>();

        let items_dat = self.items_dat.clone();
        let ws_tx_clone = self.ws_tx.clone();

        std::thread::spawn(move || {
            match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let mut bot = crate::bot::Bot::new_ltoken(&ltoken_str, proxy, state_clone, cmd_rx, items_dat, id, Some(ws_tx_clone));
                bot.run(stop_clone);
            })) {
                Ok(_)  => println!("[Bot:{id}] Stopped."),
                Err(_) => println!("[Bot:{id}] Crashed."),
            }
        });

        self.bots.insert(id, BotEntry { username: String::new(), stop_flag, state, cmd_tx });
        let _ = self.ws_tx.send(WsEvent::BotAdded { bot_id: id, username: String::new() });
        id
    }

    pub fn stop(&mut self, id: u32) -> bool {
        if let Some(entry) = self.bots.remove(&id) {
            // Hentikan proses bot
            entry.stop_flag.store(true, Ordering::Relaxed);
            let _ = self.ws_tx.send(WsEvent::BotRemoved { bot_id: id });
            
            // Hapus dari data Auto-Save dan perbarui file JSON
            self.saved_bots.remove(&id);
            self.save_to_disk();
            
            true
        } else {
            false
        }
    }

    pub fn list(&self) -> Vec<BotInfo> {
        self.bots.iter().map(|(id, e)| {
            let s = e.state.read().unwrap();
            BotInfo {
                id:       *id,
                username: e.username.clone(),
                status:  s.status.to_string(),
                world:   s.world_name.clone(),
                pos_x:   s.pos_x,
                pos_y:   s.pos_y,
                gems:    s.gems,
                ping_ms: s.ping_ms,
            }
        }).collect()
    }

    pub fn get_state(&self, id: u32) -> Option<BotState> {
        self.bots.get(&id).map(|e| e.state.read().unwrap().clone())
    }

    pub fn send_cmd(&self, id: u32, cmd: BotCommand) -> bool {
        self.bots.get(&id).map(|e| e.cmd_tx.send(cmd).is_ok()).unwrap_or(false)
    }

    pub fn run_script(&self, id: u32, content: String) -> bool {
        self.send_cmd(id, BotCommand::RunScript { content })
    }

    pub fn find_by_name(&self, name: &str) -> Option<(Arc<RwLock<BotState>>, CmdSender)> {
        self.bots.values()
            .find(|e| e.username.eq_ignore_ascii_case(name))
            .map(|e| (e.state.clone(), e.cmd_tx.clone()))
    }

    pub fn stop_by_name(&mut self, name: &str) -> bool {
        if let Some(id) = self.bots.iter()
            .find(|(_, e)| e.username.eq_ignore_ascii_case(name))
            .map(|(id, _)| *id)
        {
            self.stop(id)
        } else {
            false
        }
    }
}