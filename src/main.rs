use std::{collections::HashMap, path::PathBuf, sync::mpsc::channel, thread, time::Duration};

use anyhow::{anyhow, Result};
use clap::{Parser, Subcommand};

use api::api_server_init;
use config::{load_config, Config, LogType};
use logbackend::{FileLogBackend, HeapLogBackend, LogBackend};
use roles::{Master, Worker};

mod api;
mod config;
mod connection;
mod executor;
mod issue;
mod logbackend;
mod mailbox;
mod roles;

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
    let (tx, rx) = channel();
    let api_handler = thread::Builder::new()
        .name("master_api_interface".to_string())
        .spawn(move || api_server_init(api_endpoint, tx).ok())?;

    let address = cfg.address();
    let mut address_book = HashMap::new();
    let mut worker_list = Vec::with_capacity(5);
    for v in cfg.address_book().values() {
        worker_list.push(v.clone());
    }
    address_book.insert("worker".to_string(), worker_list);
    let log_type = cfg.log_backend();

    let service_handler = thread::Builder::new()
        .name("master_interface".to_string())
        .spawn(move || {
            let logbackend: Box<dyn LogBackend> = match log_type {
                LogType::Heap => Box::new(HeapLogBackend::new()),
                LogType::File(file_name) => Box::new(FileLogBackend::new(&file_name)),
            };

            let master = Master::new(address, address_book, logbackend);

            if let Ok(master) = master {
                loop {
                    if let Ok(cmd) = rx.recv() {
                        let _ = match cmd {
                            api::CmdType::Log(log_command) => {
                                let _ = master.emmit_new_proposal(log_command.clone());
                                Ok(log_command)
                            }
                            api::CmdType::Query(id) => master.get_log(id),
                        };
                    }
                    let _ = master.process_vote();

                    thread::sleep(Duration::from_secs(1));
                }
            }
        })?;

    api_handler
        .join()
        .map_err(|_| anyhow!("Can't finishing thread API-service."))?;
    service_handler
        .join()
        .map_err(|_| anyhow!("Can't finishing thread Master-service."))?;

    Ok(())
}

fn start_worker(cfg: Config) -> Result<()> {
    let mut address_book = HashMap::new();
    let mut master = Vec::with_capacity(1);
    for v in cfg.address_book().values() {
        master.push(v.clone());
    }
    address_book.insert("master".to_string(), master);

    let service_handler = thread::Builder::new()
        .name("worker_service".to_string())
        .spawn(move || {
            if let Ok(worker) = Worker::new(cfg.address(), address_book) {
                loop {
                    let _ = worker.vote();
                }
            };
        })?;

    service_handler.join().map_err(|_| anyhow!("ERROR"))
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
