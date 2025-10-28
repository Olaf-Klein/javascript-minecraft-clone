#![allow(dead_code)]
use anyhow::{Context, Result};
use rhai::{Engine, AST, Dynamic, Scope};
use std::collections::HashMap;
use std::path::Path;
use std::sync::mpsc::{self, Sender};
use std::time::{SystemTime, UNIX_EPOCH};

/// Commands sent from mods into the game. `GetBlock` carries a responder so the
/// caller can wait for the reply synchronously.
pub enum ModCommand {
    GetBlock { x: i32, y: i32, z: i32, responder: Sender<i64> },
    SetBlock { x: i32, y: i32, z: i32, id: u16, responder: Sender<bool> },
    SpawnItem { id: String, count: u16, responder: Sender<bool> },
    GetPlayerPos { responder: Sender<(f32, f32, f32)> },
    SetTimeOfDay { time: f64, responder: Sender<bool> },
    SpawnEntity { ty: String, x: f32, y: f32, z: f32, responder: Sender<bool> },
}

pub struct ModManager {
    engine: Engine,
    scripts: HashMap<String, AST>,
    cmd_sender: Option<Sender<ModCommand>>,
}

impl ModManager {
    pub fn new() -> Self {
        let mut engine = Engine::new();

        // Register a simple `log` function for mods to call
        engine.register_fn("log", |s: &str| {
            println!("[mod] {}", s);
        });

        // Provide a small time helper
        engine.register_fn("now", || {
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .map(|d| d.as_secs())
                .unwrap_or(0)
        });

        Self {
            engine,
            scripts: HashMap::new(),
            cmd_sender: None,
        }
    }

    /// Attach a Host instance so mods can call into the game.
    pub fn set_command_sender(&mut self, sender: Sender<ModCommand>) {
        // Register functions that forward into the command channel. We must clone
        // the sender into the closures so they have a 'static lifetime.
        let tx_get = sender.clone();
        self.engine.register_fn("host_get_block", move |x: i64, y: i64, z: i64| {
            // Create a responder channel and block waiting for a reply (short timeout is handled by caller)
            let (resp_tx, resp_rx) = mpsc::channel::<i64>();
            let _ = tx_get.send(ModCommand::GetBlock { x: x as i32, y: y as i32, z: z as i32, responder: resp_tx });
            // Wait for response (blocking). If the game doesn't reply, return 0.
            resp_rx.recv().unwrap_or(0)
        });

        let tx_set = sender.clone();
        self.engine.register_fn("host_set_block", move |x: i64, y: i64, z: i64, id: i64| {
            // responder channel so the closure can wait for the game to confirm
            let (resp_tx, resp_rx) = mpsc::channel::<bool>();
            let _ = tx_set.send(ModCommand::SetBlock { x: x as i32, y: y as i32, z: z as i32, id: id as u16, responder: resp_tx });
            resp_rx.recv().unwrap_or(false)
        });

        let tx_spawn = sender.clone();
        self.engine.register_fn("host_spawn_item", move |id: &str, count: i64| {
            let (resp_tx, resp_rx) = mpsc::channel::<bool>();
            let _ = tx_spawn.send(ModCommand::SpawnItem { id: id.to_string(), count: count as u16, responder: resp_tx });
            resp_rx.recv().unwrap_or(false)
        });

        let tx_player = sender.clone();
        self.engine.register_fn("host_get_player_pos", move || {
            let (resp_tx, resp_rx) = mpsc::channel::<(f32, f32, f32)>();
            let _ = tx_player.send(ModCommand::GetPlayerPos { responder: resp_tx });
            resp_rx.recv().unwrap_or((0.0, 0.0, 0.0))
        });

        let tx_time = sender.clone();
        self.engine.register_fn("host_set_time", move |time: rhai::Dynamic| {
            // Accept numeric input and forward as f64
            let t = time.as_float().unwrap_or(0.0);
            let (resp_tx, resp_rx) = mpsc::channel::<bool>();
            let _ = tx_time.send(ModCommand::SetTimeOfDay { time: t, responder: resp_tx });
            resp_rx.recv().unwrap_or(false)
        });

        let tx_entity = sender.clone();
        self.engine.register_fn("host_spawn_entity", move |ty: &str, x: f64, y: f64, z: f64| {
            let (resp_tx, resp_rx) = mpsc::channel::<bool>();
            let _ = tx_entity.send(ModCommand::SpawnEntity { ty: ty.to_string(), x: x as f32, y: y as f32, z: z as f32, responder: resp_tx });
            resp_rx.recv().unwrap_or(false)
        });

        self.cmd_sender = Some(sender);
    }

    /// Load and compile a Rhai script storing the AST under the file stem key.
    pub fn load_script(&mut self, path: &Path) -> Result<()> {
        let contents = std::fs::read_to_string(path)
            .with_context(|| format!("failed to read script: {}", path.display()))?;
        let ast = self
            .engine
            .compile(&contents)
            .map_err(|e| anyhow::anyhow!("rhai parse error: {}", e))?;
        if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
            self.scripts.insert(stem.to_string(), ast);
        }
        Ok(())
    }

    /// Execute all loaded scripts' `on_tick(host)` function if present.
    pub fn execute_tick(&self) {
        for (name, ast) in &self.scripts {
            let mut scope = Scope::new();
            // Call on_tick() without args; mods should use host_* functions to interact
            let result = self.engine.call_fn::<()>(&mut scope, ast, "on_tick", ());

            if let Err(e) = result {
                eprintln!("mod '{}' on_tick error: {}", name, e);
            }
        }
    }

    /// Trigger a named event in all mods: it will call `on_event(event_name, args...)` if defined.
    pub fn trigger_event(&self, event: &str, args: Vec<Dynamic>) {
        for (name, ast) in &self.scripts {
            let mut scope = Scope::new();
            // Call on_event(name, args) and ignore its return value. If the function
            // is missing or errors, log the error.
            let call_result = self
                .engine
                .call_fn::<()>(&mut scope, ast, "on_event", (event.to_string(), args.clone()));
            if let Err(e) = call_result {
                let msg = format!("mod '{}' on_event error: {}", name, e);
                eprintln!("{}", msg);
            }
        }
    }
}
