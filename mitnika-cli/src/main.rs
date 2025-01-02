// use clap::{Args, Parser, Subcommand};
use commons::Result;
// use commons::{
//     Environment, FileHandlerBuilder, Mitnika, MitnikaError, Project, Result, Version,
//     DEFAULT_STRING,
// };
//
// /// Project Details
// #[derive(Debug, Clone, PartialEq, PartialOrd, Parser)]
// #[command(version, about, long_about = None)]
// struct ProjectArgs {
//     /// Name of the Collection/Project
//     #[arg(short = 'n', long)]
//     project_name: String,
//
//     #[command(subcommand)]
//     file: Option<FileHandlerCommands>,
// }
//
// #[derive(Subcommand, Debug, Clone, PartialEq, PartialOrd)]
// enum FileHandlerCommands {
//     Add(AddFileHandlerArgs),
//     Modify(ModifyFileHandlerArgs),
//     Delete(DeleteFileHandlerArgs),
// }
//
// /// File details
// #[derive(Debug, Clone, PartialEq, PartialOrd, Args)]
// struct AddFileHandlerArgs {
//     /// name of file to create
//     #[arg(short = 'f', long)]
//     file_name: String,
//
//     /// file path from root where it will be stored
//     #[arg(short = 'p', long)]
//     file_creation_path: String,
//
//     /// name of the version of the file
//     #[arg(short, long)]
//     version_name: Option<String>,
//
//     /// name of the environment to create file with
//     #[arg(short, long)]
//     environment_name: Option<String>,
//
//     #[arg(short = 'c', long)]
//     file_content: Option<String>,
// }
//
// /// File details
// #[derive(Debug, Clone, PartialEq, PartialOrd, Args)]
// struct DeleteFileHandlerArgs {
//     /// name of file to delete
//     #[arg(short = 'f', long)]
//     file_name: Option<String>,
//
//     /// name of the version to delete
//     #[arg(short, long)]
//     version_name: Option<String>,
//
//     /// name of the environment to delete
//     #[arg(short, long)]
//     environment_name: Option<String>,
// }
//
// /// File details
// #[derive(Debug, Clone, PartialEq, PartialOrd, Args)]
// struct ModifyFileHandlerArgs {
//     /// name of file to create
//     #[arg(short = 'f', long)]
//     file_name: String,
//
//     /// file path from root where it will be stored
//     #[arg(short = 'p', long)]
//     file_creation_path: Option<String>,
//
//     /// name of the version of the file
//     #[arg(short, long)]
//     version_name: Option<String>,
//
//     /// name of the environment to create file with
//     #[arg(short, long)]
//     environment_name: Option<String>,
//
//     #[arg(short = 'c', long)]
//     file_content: Option<String>,
//
//     /// insert if it doesn't exist
//     #[arg(short, long)]
//     upsert: bool,
// }
//
fn main() -> Result<()> {
    // let args = ProjectArgs::parse();
    // let mut mitnika = Mitnika::new();
    // if let Some(file_args) = args.file {
    //     match file_args {
    //         FileHandlerCommands::Add(file_data) => {
    //             let project = match mitnika.get_project_mut(&args.project_name) {
    //                 Some(project) => project,
    //                 None => mitnika.add_project(&args.project_name),
    //             };
    //             handle_add(project, file_data);
    //         }
    //         FileHandlerCommands::Modify(file_data) => {
    //             let project = mitnika
    //                 .get_project_mut(&args.project_name)
    //                 .ok_or(MitnikaError::ProjectNotFound(args.project_name))?;
    //             handle_modify(project, file_data)?;
    //         }
    //         FileHandlerCommands::Delete(file_data) => {
    //             let project = mitnika
    //                 .get_project_mut(&args.project_name)
    //                 .ok_or(MitnikaError::ProjectNotFound(args.project_name))?;
    //             handle_delete(project, file_data);
    //         }
    //     }
    // }
    Ok(())
}
//
// // TODO: separate content into different structures based on structs
// fn handle_delete(project: &mut Project, file_data: DeleteFileHandlerArgs) {
//     // if file name is present
//     if let Some(file_name) = file_data.file_name {
//         if let Some(file) = project.get_file_mut(&file_name) {
//             // if environment name is present
//             if let Some(env_name) = file_data.environment_name {
//                 if let Some(env) = file.get_environment_mut(&env_name) {
//                     // if version name is present
//                     if let Some(version_name) = file_data.version_name {
//                         env.delete_version(&version_name);
//                     }
//                     // else delete environment
//                     else {
//                         file.delete_environment(&env_name);
//                     }
//                 }
//             }
//             // else delete file
//             else {
//                 project.delete_file(&file_name);
//             }
//         }
//     }
// }
//
// // TODO: separate content into different structures based on structs
// fn handle_modify(project: &mut Project, file_data: ModifyFileHandlerArgs) -> Result<()> {
//     let upsert = file_data.upsert;
//     let file_handler = if let Some(file_handler) = project.get_file_mut(&file_data.file_name) {
//         file_handler
//     } else if upsert {
//         if let Some(file_creation_path) = file_data.file_creation_path {
//             project.add_file(
//                 FileHandlerBuilder::new(&file_data.file_name)
//                     .creation_path(&file_creation_path)
//                     .build(),
//             );
//             // Safety: added the file in previous line
//             project
//                 .get_file_mut(&file_data.file_name)
//                 .expect("File was created and added in the previous line")
//         } else {
//             return Err(MitnikaError::FileCreationPathRequired(file_data.file_name));
//         }
//     } else {
//         return Err(MitnikaError::FileNotFound(file_data.file_name));
//     };
//
//     let env = if let Some(env_name) = &file_data.environment_name {
//         if let Some(env) = file_handler.get_environment_mut(env_name) {
//             env
//         } else if upsert {
//             file_handler.add_environment(env_name);
//             // Safety: environment is added in previous line so it should not fail.
//             file_handler
//                 .get_environment_mut(env_name)
//                 .expect("Environment created in previous line but not found.")
//         } else {
//             return Err(MitnikaError::EnvironmentNotFound(env_name.clone()));
//         }
//     }
//     // use default
//     else {
//         file_handler.add_environment(DEFAULT_STRING);
//         file_handler.get_default_environment_mut().unwrap()
//     };
//
//     let version = if let Some(version_name) = &file_data.version_name {
//         if let Some(ver) = env.get_version_mut(version_name) {
//             ver
//         } else if upsert {
//             env.add_version(version_name);
//             // Safety: environment is added in previous line so it should not fail.
//             env.get_version_mut(version_name)
//                 .expect("Version created in previous line but not found.")
//         } else {
//             return Err(MitnikaError::VersionNotFound(version_name.clone()));
//         }
//     }
//     // use default
//     else {
//         env.add_version(DEFAULT_STRING);
//         env.get_default_version_mut().unwrap()
//     };
//
//     if let Some(content) = file_data.file_content {
//         version.set_content(&content);
//     }
//
//     Ok(())
// }
//
// // TODO: separate content into different structures based on structs
// fn handle_add(project: &mut Project, file_data: AddFileHandlerArgs) {
//     let file_builder =
//         FileHandlerBuilder::new(&file_data.file_name).creation_path(&file_data.file_creation_path);
//
//     let mut environment = if let Some(env) = file_data.environment_name {
//         Environment::new(&env)
//     } else {
//         Environment::default()
//     };
//
//     let mut version = if let Some(version) = file_data.version_name {
//         Version::new(&version)
//     } else {
//         Version::default()
//     };
//
//     if let Some(content) = file_data.file_content {
//         version.set_content(&content);
//     }
//
//     environment.version(version);
//     project.add_file(file_builder.environment(environment).build());
// }
