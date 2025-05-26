mod args;
mod config;
mod error;
mod placeholder;
mod render;
mod shell;

use std::io;

use args::{Args, Command};
use clap::Parser;
use crossterm::{
    ExecutableCommand,
    terminal::{self, ClearType},
};
use error::{PlatesError, PlatesResult};
use inquire::{Confirm, InquireError, Select};
use placeholder::PlaceholderValueMap;

fn try_main(args: Args) -> PlatesResult<()> {
    let templates = config::get_template_names()?;

    match args.command {
        Command::List => {
            for template_name in templates {
                println!("{template_name}");
            }
        }
        Command::Render {
            dest,
            template,
            overwrite,
        } => {
            (!templates.is_empty())
                .then_some(())
                .ok_or(PlatesError::NoTemplatesAvailable)?;

            template
                .as_ref()
                .is_none_or(|t| templates.contains(t))
                .then_some(())
                .ok_or(PlatesError::NonExistentTemplate)?;

            (!dest.is_file())
                .then_some(())
                .ok_or(PlatesError::PathIsFile)?;

            let is_dest_empty = dest
                .read_dir()
                .map(|mut items| items.next().is_none())
                .unwrap_or(true);

            if !is_dest_empty {
                Confirm::new("The destination directory is not empty. Continue?")
                    .with_default(false)
                    .prompt()?
                    .then_some(())
                    .ok_or(PlatesError::Inquire(InquireError::OperationCanceled))?;
            }

            let selected_template_name = template.map(Ok).unwrap_or_else(|| {
                Select::new("Which template should I generate?", templates).prompt()
            })?;

            let template_config =
                config::get_template_config(&selected_template_name)?.unwrap_or_default();

            let mut placeholder_values = PlaceholderValueMap::new();

            placeholder_values.insert("plates_dir".to_owned(), dest.to_string_lossy().into_owned());
            placeholder_values.insert(
                "plates_dir_basename".to_owned(),
                dest.file_name().unwrap().to_string_lossy().into_owned(),
            );

            placeholder::inquire_placeholders(
                template_config.placeholders,
                &mut placeholder_values,
            )?;

            println!("Rendering {selected_template_name}...");
            render::render_template(
                &selected_template_name,
                &dest,
                overwrite,
                &placeholder_values,
            )?;
            println!("Done!");
        }
    }

    Ok(())
}

fn main() {
    let args = Args::parse();
    let result = try_main(args);

    if let Err(err) = result {
        let _ = io::stdout().execute(terminal::Clear(ClearType::CurrentLine));

        match err {
            PlatesError::Inquire(
                InquireError::OperationCanceled | InquireError::OperationInterrupted,
            ) => println!("Operation cancelled!"),
            _ => eprintln!("Error: {}", err),
        }
    }
}
