use clap::{AppSettings, Parser, Subcommand};
use std::env;
use std::path::Path;
mod add;
mod conf;
mod download;
mod init;
mod judge;
mod login;
mod open;
use conf::config::ConfigIO;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
#[clap(global_setting(AppSettings::PropagateVersion))]
#[clap(global_setting(AppSettings::UseLongFormatForHelpSubcommand))]
struct Cli {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialization
    Init {
        // target directory
        path: Option<String>,
    },
    /// Configuration
    Conf {
        /// contest name. ex: abc123
        contest: Option<String>,
        /// problem name. ex: c
        problem: Option<String>,
        /// interactive mode
        #[clap(short, long)]
        ii: bool,
        /// Select language
        #[clap(short, long, arg_enum)]
        lang: Option<conf::Lang>,
    },
    /// Open problem description
    Open {},
    /// Login contest site
    Login {},
    /// Download system test cases
    Download {},
    /// Testing
    Test {
        #[clap(short, long, parse(from_occurrences))]
        verbose: usize,
    },
    /// Adds user test cases
    Add {},
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path } => {
            init::command(path);
        }
        Commands::Add {} => {
            let cwd = env::current_dir().unwrap();
            let judge_conf = conf::config::JudgeConf::load(&cwd).unwrap();
            add::command(&judge_conf.testdir);
        }
        Commands::Conf {
            contest,
            problem,
            ii,
            lang,
        } => {
            conf::command(contest, problem, ii, lang);
        }
        Commands::Open {} => open::command(),
        Commands::Login {} => login::command(),
        Commands::Download {} => {
            let cwd = env::current_dir().unwrap();
            let judge_conf = conf::config::JudgeConf::load(&cwd).unwrap();
            download::command(&judge_conf.URL, &Path::new(&judge_conf.testdir))
        }
        Commands::Test { verbose } => {
            judge::command(verbose);
        }
    }
}
