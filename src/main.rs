mod mmv;

use mmv::{ActionWhenRenamedFilePathExists, TemplateFileRenamer};

use crate::mmv::TfrError;
use chrono::offset::Local;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input file path template
    ///
    /// To capture, use the asterisks: '*'.
    /// Asterisks are allowed only on the last part of the path. Double asterisk are not allowed
    ///
    /// Example: example/input/template/path_*.*
    input_file_template: String,

    /// Output file path template.
    ///
    /// To insert capture, use flag #<capture_index|int>. Multiple use of the same flag is allowed. All captures must be covered with at least one flag
    ///
    /// Example: example/output/template/new_#1_path_#1.#2
    output_file_template: String,

    /// Use the force flag to overwrite the path to the output file, if it exists
    #[arg(short, long, action)]
    force: bool,
}

fn main() {
    let args = Args::parse();

    let input_file_template = args.input_file_template;
    let output_file_template = args.output_file_template;

    let start_time = Local::now();
    let callback_handler =
        |processed: usize, total: usize, old_filepath: Option<&str>, new_filepath: Option<&str>| {
            if processed == 0 {
                return match total {
                    0 => {
                        println!("Files for pattern '{input_file_template}' not found");
                        std::process::exit(1);
                    }
                    _ => {
                        println!(
                            "Started with params: {} -> {}. Files to rename: {}",
                            input_file_template, output_file_template, total
                        )
                    }
                };
            }

            println!("{} -> {}", old_filepath.unwrap(), new_filepath.unwrap());

            if processed == total {
                println!(
                    "Finished in {}ms.",
                    (Local::now() - start_time).num_milliseconds()
                )
            }
        };
    let mut tfr = TemplateFileRenamer::new(ActionWhenRenamedFilePathExists::Terminate);
    tfr.set_callback_handler(callback_handler);

    if let Err(tfr_error) = tfr.rename(&input_file_template, &output_file_template) {
        match tfr_error {
            TfrError::IncorrectInputTemplate(description) => {
                eprintln!("IncorrectInputTemplate error occurred: {description}")
            }
            TfrError::IncorrectOutputTemplate(description) => {
                eprintln!("IncorrectOutputTemplate error occurred: {description}")
            }
            TfrError::ExistingPath(existing_filepath, is_file) => {
                eprintln!(
                    "Not able to replace existing {}: {}",
                    if is_file { "file" } else { "path" },
                    existing_filepath
                )
            }
            TfrError::StdError(error) => {
                eprintln!("Some error occurred: {:?}", error.as_ref())
            }
        }
        std::process::exit(1);
    }
}
