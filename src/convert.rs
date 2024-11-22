use std::collections::HashMap;

use crate::writer::RdfWriter;
use convert_case::{Case, Casing};
use csv::ReaderBuilder;
use log::error;
use oxrdf::{NamedNode, TermRef, TripleRef};

const C2R: &'static str = "https://decisym.ai/csv2rdf/model#";

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
        let mut rdr = ReaderBuilder::new()
            .has_headers(true)
            .from_path(file.to_string())?;

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
                else if field != "" {
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
