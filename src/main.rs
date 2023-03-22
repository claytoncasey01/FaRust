mod config;

use crate::config::Config;
use anyhow::{Context, Result};
use clap::Parser;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::fs;
use std::fs::{read_to_string, File};
use std::io::Write;
use std::path::Path;
use tera::Tera;
use thiserror::Error;

// Define the error type for the application
#[derive(Error, Debug)]
enum FaRustError {
    #[error("Template rendering failed: {0}")]
    TemplateRenderError(#[from] tera::Error),

    #[error("JSON parsing failed: {0}")]
    JSONParseError(#[from] serde_json::Error),

    #[error("I/O error: {0}")]
    IOError(#[from] std::io::Error),
}

#[derive(Parser, Debug)]
#[command(
    name = "FaRust",
    version = "1.0.0",
    author = "Casey Clayton <claytoncasey01@gmail.com>",
    about = "Generates icon wrapper components for Font Awesome Icons"
)]
struct Args {
    #[arg(
        short,
        long,
        help = "The path to the config file to be used when generating icons"
    )]
    config: String,
}

const ICON_TEMPLATE: &str = "icon_template.tsx";

fn load_template(template_name: &str) -> Result<String, std::io::Error> {
    let path = Path::new("templates").join(template_name);
    read_to_string(path)
}

fn generate_component(
    icon_name: &str,
    icon_path: &str,
    component_name: &str,
    output_path: &str,
    tera: &Tera,
) -> Result<()> {
    let mut context = tera::Context::new();
    context.insert("icon_name", icon_name);
    context.insert("icon_path", icon_path);
    context.insert("component_name", component_name);

    let rendered = tera
        .render(ICON_TEMPLATE, &context)
        .map_err(FaRustError::TemplateRenderError)?;

    // Create the output directory if it doesn't exist
    fs::create_dir_all(output_path).map_err(FaRustError::IOError)?;

    // Create the file and write the rendered template to it
    let file_name = format!("{}Icon.tsx", component_name);
    let file_path = Path::new(output_path).join(&file_name);
    let mut file = File::create(&file_path).map_err(FaRustError::IOError)?;
    file.write_all(rendered.as_bytes())
        .map_err(FaRustError::IOError)?;

    println!("Generated component: {}", file_name);

    Ok(())
}

fn main() -> Result<()> {
    // Parse the command line arguments
    let args = Args::parse();

    if !args.config.is_empty() {
        let config_file = read_to_string(args.config).context("Failed to read the config file")?;
        let config: Config =
            serde_json::from_str(&config_file).context("Failed to parse the config file")?;

        let mut tera = Tera::default();
        let template = load_template(ICON_TEMPLATE)?;
        tera.add_raw_template(ICON_TEMPLATE, &template)?;

        config.icons.par_iter().try_for_each(|icon| {
            generate_component(
                icon.name.as_str(),
                icon.path.as_str(),
                icon.component_name.as_str(),
                &config.output,
                &tera,
            )
        })?;
    } else {
        anyhow::bail!("Error: No config file specified, please provide one with the --config flag");
    }

    Ok(())
}
