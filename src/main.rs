mod api;
mod config;
mod connection;
mod executor;
mod issue;
mod logbackend;
mod mailbox;
mod roles;

use std::{collections::HashMap, path::PathBuf, thread};

use anyhow::{anyhow, Result};
use api::api_server_init;
use clap::{Parser, Subcommand};
use config::{load_config, Config, LogType};
use logbackend::{FileLogBackend, HeapLogBackend, LogBackend};
use roles::{Master, Worker};

/// A Simple Paxos Algorithm Implement.
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Start a Role
    #[command(subcommand)]
    role: Role,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Role Instance Name
    name: Option<String>,
}

#[derive(Subcommand)]
enum Role {
    Master,
    Worker,
}

fn start_master(cfg: Config) -> Result<()> {
    let api_endpoint = cfg.api();

    let mut address_book = HashMap::new();
    let mut woker_list = Vec::with_capacity(5);
    for v in cfg.address_book().values() {
        woker_list.push(v.clone());
    }
    address_book.insert("woker".to_string(), woker_list);

    let logbackend: Box<dyn LogBackend> = match cfg.log_backend() {
        LogType::Heap => Box::new(HeapLogBackend::new()),
        LogType::File(file_name) => Box::new(FileLogBackend::new(&file_name)),
    };

    let _master = Master::new(cfg.address(), address_book, logbackend)?;

    let api_handler = thread::Builder::new()
        .name("master_api_interface".to_string())
        .spawn(move || api_server_init(api_endpoint).ok())?;

    // todo: 如何解决 Master 和 API service 之间相互通信调用的问题。

    api_handler
        .join()
        .map_err(|_| anyhow!("Can't finishing thread API-servcie."))?;

    Ok(())
}

fn start_worker(cfg: Config) -> Result<()> {
    let mut address_book = HashMap::new();
    let mut master = Vec::with_capacity(1);
    for v in cfg.address_book().values() {
        master.push(v.clone());
    }
    address_book.insert("master".to_string(), master);

    let _worker = Worker::new(cfg.address(), address_book);

    Ok(())
}

fn main() -> Result<()> {
    let args = Args::parse();

    let role = args.role;
    let name = args.name;
    let config = args.config;

    let _ = match role {
        Role::Master => start_master(
            config
                .map(|config_path| load_config(config_path, Some("master".to_string())).ok())
                .flatten()
                .unwrap_or_default(),
        ),
        Role::Worker => start_worker(
            config
                .map(|config_path| load_config(config_path, name).ok())
                .flatten()
                .unwrap_or_default(),
        ),
    }?;

    Ok(())
}
