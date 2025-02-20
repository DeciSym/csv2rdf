# csv2rdf
Library for converting CSV files into RDF

This Rust-based tool converts CSV data into RDF format, utilizing the `oxrdf` crate for RDF graph handling and `csv` for efficient CSV parsing. Generated triples can either be added to an `oxrdf::Graph` or written directly to file.

## Using the csv2rdf CLI

This library includes a CLI utility for parsing CSV and generating N-Triple RDF using the `convert` subcommand. The binary can be built using `cargo build`.

```bash
$ csv2rdf convert --help
Convert CSV to RDF format.

The `convert` command parses a CSV file, converts it to RDF triples using `csv` for parsing and `oxrdf` to construct the graph, and saves the output.

Usage: csv2rdf convert [OPTIONS]

Options:
  -n, --namespace <NAMESPACE>
          Namespace for RDF graph generation.

          A custom namespace to prefix RDF resources created from CSV columns and rows.

          [default: https://decisym.ai/csv2rdf/data]

  -i, --input <INPUT>...
          Path to input CSV file(s).

          Provide the path to one or more CSV files that will be parsed and converted.

  -o, --output <OUTPUT>
          Path to output file.

          Optional: Specify the path to save the generated RDF data. If not provided, data will be written to stdout

  -h, --help
          Print help (see a summary with '-h')
```

## Using the convert library

The conversion functionality can also be called directly in Rust. The library supports writing results to a file or building an in-memory `oxrdf::Graph`.

```rust
use csv2rdf::convert::parse_csv;
use csv2rdf::writer;
use oxrdf::Graph;

// capture conversion results to file
let mut w = writer::FileWriter::to_file("output.nt".to_string()).unwrap();
parse_csv(vec!["data.csv".to_string()], &mut w, "https://decisym.ai/csv2rdf/data");

// capture conversion results to an oxrdf::Graph
let mut g = Graph::new();
let mut w = writer::GraphWriter::new(&mut g);
parse_csv(vec!["data.csv".to_string()], &mut w, "https://decisym.ai/csv2rdf/data");
```

## License
This project is licensed under the BSD 3-Clause License - see the [LICENSE](LICENSE) file for details.