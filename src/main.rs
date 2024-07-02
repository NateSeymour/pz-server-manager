use rust_apt;
use rust_apt::raw::progress::{AptAcquireProgress, AptInstallProgress};
use clap::{Parser};
use std::process::{Command, Stdio};
use std::io::Write;
use std::fs;
use std::env;
use std::fmt::format;

fn init() {
    // Initialize all directories
    let home = env::var("HOME").unwrap();

    fs::create_dir_all(format!("{}/.pz-manager/bin", home)).unwrap();
}

fn install_steamcmd() {
    let cache = rust_apt::new_cache!().unwrap();
    let steamcmd = cache.get("steamcmd:i386").unwrap();

    let mut acquire_progress = AptAcquireProgress::new_box();
    let mut install_progress = AptInstallProgress::new_box();

    steamcmd.mark_install(true, true);
    steamcmd.protect();
    cache.resolve(true).unwrap();

    cache.commit(&mut acquire_progress, &mut install_progress).unwrap();
}

fn install_pz() {
    let mut steamcmd = Command::new("/usr/games/steamcmd")
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to start `steamcmd`. It can be installed with '--install-steam-cmd'.");

    let mut stdin = steamcmd.stdin.take().unwrap();

    let home = env::var("HOME").unwrap();
    stdin.write_all(format!("force_install_dir {}/.pz-manager/bin/\n", home).as_bytes()).unwrap();
    stdin.write_all(b"login anonymous\n").unwrap();
    stdin.write_all(b"app_update 380870 validate\n").unwrap();
    stdin.write_all(b"quit\n").unwrap();

    steamcmd.wait();
}

fn launch_pz(name: String) {
    let home = env::var("HOME").unwrap();
    let mut pz = Command::new(format!("{home}/.pz-manager/bin/start-server.sh"))
        .args(&[format!("-servername {name}").as_str(), "-adminpassword password", format!("-cachedir={home}/.pz-manager").as_str()])
        .spawn()
        .unwrap();

    pz.wait();
}

#[derive(clap::Parser)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(clap::Subcommand)]
enum Commands {
    InstallSteamCMD,
    Init {
        #[arg(long, default_value = "true")]
        install_pz: bool,
    },
    Start {
        #[arg(long, default_value = "MyServer")]
        name: String,
    },
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::InstallSteamCMD) => {
            install_steamcmd();
        },
        Some(Commands::Init { install_pz }) => {
            init();

            if install_pz {
                crate::install_pz();
            }
        },
        Some(Commands::Start { name }) => {
            launch_pz(name);
        },
        None => {}
    }
}
