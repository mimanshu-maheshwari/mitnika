use core::{ProjectDetails, Storage};
use std::collections::HashSet;

use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Clone, Debug)]
#[command(name = "mitnika", about = "A CLI tool to manage to your secrets.")]
struct Cli {
    #[command(subcommand)]
    command: EntityCommand,
}

#[derive(Debug, Clone, ValueEnum)]
enum Action {
    Create,
    Update,
    Delete,
    Find,
}

#[derive(Subcommand, Clone, Debug)]
enum EntityCommand {
    Project(ProjectArgs),
    File(FileArgs),
    Environment(EnvironmentArgs),
    Version(VersionArgs),
}

#[derive(Parser, Clone, Debug)]
struct ProjectArgs {
    #[arg(value_enum)]
    action: Action,

    #[arg(short, long)]
    id: Option<String>,

    #[arg(short, long)]
    name: Option<String>,
}

#[derive(Parser, Clone, Debug)]
struct FileArgs {
    #[arg(value_enum)]
    action: Action,

    #[arg(short, long)]
    id: Option<String>,

    #[arg(short, long)]
    name: Option<String>,

    #[arg(short, long)]
    project_id: Option<String>,
}

#[derive(Parser, Clone, Debug)]
struct EnvironmentArgs {
    #[arg(value_enum)]
    action: Action,

    #[arg(short, long)]
    id: Option<String>,

    #[arg(short, long)]
    name: Option<String>,

    #[arg(short, long)]
    file_id: Option<String>,
}

#[derive(Parser, Clone, Debug)]
struct VersionArgs {
    #[arg(value_enum)]
    action: Action,

    #[arg(short, long)]
    id: Option<String>,

    #[arg(short, long)]
    name: Option<String>,

    #[arg(short, long)]
    environment_id: Option<String>,

    #[arg(short, long)]
    content: Option<String>,
}

fn main() {
    let arguments = Cli::parse();
    println!("{arguments:?}");
    let storage = Storage::new();

    match arguments.command {
        EntityCommand::Project(args) => match args.action {
            Action::Create => {
                if let Some(name) = args.name {
                    if let Some(project) = storage.add_project(&name) {
                        println!("{project:?}");
                    }
                } else {
                    eprintln!("Require a name for project");
                }
            }
            Action::Update => {}
            Action::Delete => {}
            Action::Find => match (args.name, args.id) {
                (Some(name), None) => {
                    let projects = storage.search_projects(&name, false);
                    println!("{projects:?}");
                }
                (Some(name), Some(id)) => {
                    let mut projects = storage
                        .search_projects(&name, false)
                        .into_iter()
                        .collect::<HashSet<ProjectDetails>>();

                    if let Some(project) = storage.project_by_id(&id) {
                        projects.insert(project);
                    }
                    println!("{projects:?}");
                }
                (None, Some(id)) => {
                    if let Some(project) = storage.project_by_id(&id) {
                        println!("{project:?}");
                    }
                }
                (None, None) => {
                    eprintln!("Illeagle arguments. Provide a id or name.");
                }
            },
        },
        EntityCommand::File(args) => {
            unimplemented!()
        }
        EntityCommand::Environment(args) => {
            unimplemented!()
        }
        EntityCommand::Version(args) => {
            unimplemented!()
        }
    }
}
