//! # XML2RDF Converter
//!
//! This is a Rust-based tool that converts XML data into RDF format. It uses the `xml-rs` crate
//! for XML parsing and the `oxrdf` crate to construct RDF triples and graphs.
//!
//! ## Features
//! - Parses XML input and converts it to RDF triples
//! - Supports specifying a custom namespace for generated RDF nodes
//! - Outputs RDF data to a specified file, oxrdf::Graph or stdout
//!
//! ## Usage
//! Run the XML2RDF converter from the command line. For detailed usage information, run:
//! ```
//! xml2rdf --help
//! ```
//!
//! ## Example
//! To convert a XML file to RDF format with a specified namespace and output file:
//! ```
//! xml2rdf convert --namespace http://example.com/ns# --xml data.xml --output-file output.nt
//! ```
//! This will take `data.xml`, apply the specified namespace, and save the RDF output in `output.nt`.

use clap::{Parser, Subcommand};
use csv2rdf::*;

/// Command-line interface for XML2RDF Converter
///
/// This struct defines the command-line interface (CLI) for interacting with the XML2RDF converter.
#[derive(Parser)]
#[command(version, about = "Converts XML data into RDF format.")]
struct Cli {
    /// CLI command selection
    #[command(subcommand)]
    command: Option<Commands>,
}

/// Supported Commands
///
/// Contains the available commands for the XML2RDF converter.
#[derive(Subcommand)]
enum Commands {
    /// Convert XML to RDF format.
    ///
    /// The `convert` command parses a XML file, converts it to RDF triples using `xml-rs` for parsing
    /// and `oxrdf` to construct the graph, and saves the output.
    Convert {
        /// Namespace for RDF graph generation.
        ///
        /// A custom namespace to prefix RDF resources created from XML keys and values.
        #[arg(short, long, default_value = "https://decisym.ai/csv2rdf/data")]
        namespace: String,

        /// Path to input XML file(s).
        ///
        /// Provide the path to one or more XML files that will be parsed and converted.
        #[arg(short, long, num_args = 1..)]
        input: Vec<String>,

        /// Path to output file.
        ///
        /// Optional: Specify the path to save the generated RDF data. If not provided, data will be written
        /// to stdout
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Some(Commands::Convert {
            namespace,
            input,
            output,
        }) => {
            let mut w: Box<dyn writer::RdfWriter> = if let Some(file) = output {
                match writer::FileWriter::to_file(file.clone()) {
                    Err(e) => {
                        eprintln!("Error opening file for writing: {e}");
                        return;
                    }
                    Ok(v) => Box::new(v),
                }
            } else {
                Box::new(writer::FileWriter::to_stdout())
            };

            match convert::parse_csv(input.clone(), w.as_mut(), namespace) {
                Ok(_) => {}
                Err(e) => eprintln!("Error writing: {}", e),
            }
        }
        None => {}
    }
}
