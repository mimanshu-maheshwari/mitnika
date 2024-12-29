use clap::{Args, Parser, Subcommand};
use commons::{Environment, FileHandler, FileHandlerBuilder, Project, ProjectBuilder, Version};

/// Project Details
#[derive(Debug, Clone, PartialEq, PartialOrd, Parser)]
#[command(version, about, long_about = None)]
struct ProjectArgs {
    /// Name of the Collection/Project
    #[arg(short = 'n', long)]
    project_name: String,

    #[command(subcommand)]
    file: Option<FileHandlerCommands>,
}

#[derive(Subcommand, Debug, Clone, PartialEq, PartialOrd)]
enum FileHandlerCommands {
    FileHandler(FileHandlerArgs),
}

/// File details
#[derive(Debug, Clone, PartialEq, PartialOrd, Args)]
struct FileHandlerArgs {
    /// name of file to create
    #[arg(short = 'f', long)]
    file_name: String,

    /// file path from root where it will be stored
    #[arg(short = 'p', long)]
    file_creation_path: String,

    /// name of the version of the file
    #[arg(short, long)]
    version_name: Option<String>,

    /// name of the environment to create file with
    #[arg(short, long)]
    environment_name: Option<String>,

    #[arg(short = 'c', long)]
    file_content: Option<String>,
}

fn main() {
    let args = ProjectArgs::parse();
    let mut project = create_project(&args.project_name);
    if let Some(FileHandlerCommands::FileHandler(file_data)) = args.file {
        let mut file = create_file(&file_data.file_name, &file_data.file_creation_path);

        let mut environment = if let Some(env) = file_data.environment_name {
            Environment::new(&env)
        } else {
            Environment::default()
        };

        let mut version = if let Some(version) = file_data.version_name {
            Version::new(&version)
        } else {
            Version::default()
        };

        if let Some(content) = file_data.file_content {
            version.set_content(&content);
        }
        environment.set_version(Some(version));
        file.set_environment(Some(environment));

        project.add_file(file);
    }
    // dbg!(project);
}

fn create_project(project_name: &str) -> Project {
    ProjectBuilder::new(project_name).build()
}

fn create_file(file_name: &str, creation_path: &str) -> FileHandler {
    FileHandlerBuilder::new(file_name)
        .creation_path(creation_path)
        .build()
}
