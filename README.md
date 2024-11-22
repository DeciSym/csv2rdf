# csv2rdf
Library for converting CSV files into RDF

This Rust-based tool converts CSV data into RDF format, utilizing the `oxrdf` crate for RDF graph handling and `csv` for efficient CSV parsing. Generated triples can either be added to an `oxrdf::Graph` or written directly to file.