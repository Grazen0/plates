use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Clone, Subcommand)]
pub enum Command {
    /// Renders a template
    Render {
        /// Place to render the template at
        dest: PathBuf,

        /// Template to generate
        #[arg(short, long)]
        template: Option<String>,

        /// Whether to overwrite existing files
        #[arg(short, long)]
        overwrite: bool,
    },

    /// Lists all available templates
    List,
}

#[derive(Debug, Clone, Parser)]
pub struct Args {
    #[command(subcommand)]
    pub command: Command,
}
