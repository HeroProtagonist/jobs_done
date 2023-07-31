use clap::{Parser, ValueEnum};
use std::process::exit;
use std::process::Command;

mod sound;
use sound::play_sound;

#[derive(ValueEnum, Clone, Debug)]
pub enum Sound {
    JobsDone,
    WorkComplete,
    ZugZug,
}

/// Run a command and play an audio file when it's done
#[derive(Debug, Parser)]
#[command(version, arg_required_else_help(true))]
struct Cli {
    /// Just play the sound without supplying a command
    #[clap(long, short)]
    play: bool,
    /// Sound to play
    #[arg(short, long)]
    #[clap(value_enum, default_value_t=Sound::JobsDone)]
    sound: Sound,
    #[clap(allow_hyphen_values = true)]
    /// Command to run
    command: Vec<String>,
}

fn main() {
    let args = Cli::parse();

    if args.play {
        play_sound(args.sound).unwrap();
        exit(0);
    }

    let mut command = Command::new(&args.command[0]);

    if let Ok(mut child) = command.args(&args.command[1..]).spawn() {
        child.wait().expect("command wasn't running");

        play_sound(args.sound).unwrap();
    } else {
        println!("Faild to run");
        exit(1)
    }
}
