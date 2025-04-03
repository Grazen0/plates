mod args;
mod config;
mod error;
mod placeholder;
mod render;

use std::{collections::HashMap, io};

use args::{Args, Command};
use clap::{Parser, error::Result};
use config::get_templates_dir;
use crossterm::{
    ExecutableCommand,
    terminal::{self, ClearType},
};
use error::PlatesError;
use inquire::{Confirm, InquireError, Select};

fn try_main(args: Args) -> Result<(), PlatesError> {
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
                .ok_or(PlatesError::NoTemplates)?;

            template
                .as_ref()
                .is_none_or(|t| templates.contains(t))
                .then_some(())
                .ok_or(PlatesError::NonExistentTemplate)?;

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

            let placeholder_values: HashMap<_, _> = template_config
                .placeholders
                .into_iter()
                .map(|placeholder| {
                    placeholder
                        .inquire_value()
                        .map(|value| (placeholder.name.clone(), value))
                })
                .collect::<Result<_, _>>()?;

            let template_dir = get_templates_dir()?.join(&selected_template_name);

            println!("Rendering {selected_template_name}...");
            render::render_template(&template_dir, &dest, overwrite, &placeholder_values)?;
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
            PlatesError::Inquire(InquireError::OperationCanceled)
            | PlatesError::Inquire(InquireError::OperationInterrupted) => {
                println!("Operation cancelled!")
            }
            _ => eprintln!("{}", err),
        }
    }
}
