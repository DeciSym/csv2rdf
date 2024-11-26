use csv2rdf::*;
use oxrdf::Graph;
use oxrdfio::{RdfFormat, RdfParser};
use std::fs;
use std::fs::File;

#[test]
fn test_graph_writer() {
    let mut g = Graph::new();
    let mut w = writer::GraphWriter::new(&mut g);

    let res = convert::parse_csv(
        vec!["tests/resources/people.csv".to_string()],
        &mut w,
        "https://decisym.ai/xml2rdf/data",
    );
    assert!(res.is_ok());

    assert_eq!(g.len(), 23)
}

#[test]
fn test_file_writer() {
    let output = "out.nt".to_string();
    let _ = fs::remove_file(output.clone());

    let mut w = writer::FileWriter::to_file(output.clone()).expect("Failed to open output file");

    let res = convert::parse_csv(
        vec!["tests/resources/people.csv".to_string()],
        &mut w,
        "https://decisym.ai/csv2rdf/data",
    );
    assert!(res.is_ok());
    let f = File::open(output).expect("unable to open output file for result verification");
    let quads = RdfParser::from_format(RdfFormat::NTriples)
        .for_reader(f)
        .collect::<Result<Vec<_>, _>>()
        .expect("failed to parse generated output file");

    assert_eq!(quads.len(), 23)
}
