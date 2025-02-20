// Copyright (c) 2024-2025, Decisym, LLC
// Licensed under the BSD 3-Clause License (see LICENSE file in the project root).

//! # CSV2RDF Converter Library
//!
//! This library provides functionality for converting CSV data into RDF format.
//! It uses `csv` for CSV parsing and `oxrdf` to build and manage RDF graphs.
//!
//! ## Overview
//! - Converts CSV data structures into RDF triples, generating a graph representation.
//!
//! ## Features
//! - Converts nested CSV Objects into RDF triples.
//! - Allows specifying a custom RDF namespace for generated predicates and objects.
//! - Outputs the RDF data to a specified file.

use std::collections::HashMap;

use crate::writer::RdfWriter;
use convert_case::{Case, Casing};
use csv::ReaderBuilder;
use log::error;
use oxrdf::{NamedNode, TermRef, TripleRef};

const C2R: &str = "https://decisym.ai/csv2rdf/model#";

/// Converts CSV data to RDF format.
///
/// This function reads CSV data from the specified file, processes it into RDF triples,
/// and outputs the RDF graph. Users can specify a namespace to use for RDF predicates and
/// an output file for saving the generated RDF data.
///
/// # Arguments
/// - `files`: Path to the CSV files.
/// - `namespace`: Optional custom namespace for RDF predicates.
/// - `output`: use RdfWriter trait to add generated triples to desired format (File or Graph)
///
/// # Example
/// ```rust
/// use csv2rdf::convert::parse_csv;
/// use csv2rdf::writer;
/// use oxrdf::Graph;
///
/// let mut w = writer::FileWriter::to_file("output.nt".to_string()).unwrap();
/// parse_csv(vec!["data.csv".to_string()], &mut w, "https://decisym.ai/csv2rdf/data");
///
/// let mut g = Graph::new();
/// let mut w = writer::GraphWriter::new(&mut g);
/// parse_csv(vec!["data.csv".to_string()], &mut w, "https://decisym.ai/csv2rdf/data");
/// ```
pub fn parse_csv(
    files: Vec<String>,
    output: &mut dyn RdfWriter,
    namespace: &str,
) -> std::io::Result<()> {
    // ensure namespace is ready for appending
    let ns = if namespace.ends_with("/") {
        namespace
    } else {
        &([namespace, "/"].join(""))
    };

    for file in files.into_iter() {
        let mut rdr = ReaderBuilder::new().has_headers(true).from_path(&file)?;

        let mut headers: HashMap<i32, String> = HashMap::new();
        let mut column_index = 0;
        match rdr.headers() {
            Ok(h) => {
                for val in h.iter() {
                    headers.insert(column_index, val.to_case(Case::Camel));
                    column_index += 1;
                }
            }
            Err(e) => {
                error!("expected first row of CSV data to contain headers");
                return Err(e.into());
            }
        }

        for result in rdr.records() {
            let record = result.unwrap();
            let mut row_id = "".to_string();
            column_index = 0;
            for field in record.iter() {
                if column_index == 0 {
                    row_id = field.trim().replace(" ", "");
                }
                // do not append empty cells
                else if !field.is_empty() {
                    let column_id = headers.get(&column_index).unwrap();
                    let subject = NamedNode::new(format!("{}{}", ns, row_id)).unwrap();
                    let predicate = NamedNode::new(format!("{}{}", C2R, column_id)).unwrap();
                    let object = TermRef::Literal(field.into());
                    let csv_triple = TripleRef::new(subject.as_ref(), predicate.as_ref(), object);
                    output.add_triple(csv_triple)?
                }
                column_index += 1;
            }
        }
    }
    Ok(())
}
