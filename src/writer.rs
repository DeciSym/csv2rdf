//! # XML2RDF Writer Library
//!
//! This library provides functionality for writing covnerted XML2RDF data.
//! It uses `oxrdf` to build and manage RDF graphs or output the data direct to a file.
//!
//! ## Overview
//! - Adds XML RDF triples to a graph or file.

use oxrdf::{Graph, TripleRef};
use std::fs::File;
use std::fs::OpenOptions;
use std::io::{self, BufWriter, Write};

pub trait RdfWriter {
    fn add_triple(&mut self, triple: TripleRef) -> std::io::Result<()>;
}

pub struct FileWriter<W: Write> {
    writer: BufWriter<W>,
}

impl FileWriter<io::Stdout> {
    pub fn to_stdout() -> Self {
        FileWriter {
            writer: BufWriter::new(io::stdout()),
        }
    }
}

impl FileWriter<File> {
    pub fn to_file(output_file: String) -> io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(output_file)?;
        Ok(FileWriter {
            writer: BufWriter::new(file),
        })
    }
}

impl<W: Write> RdfWriter for FileWriter<W> {
    fn add_triple(&mut self, triple: TripleRef) -> std::io::Result<()> {
        self.writer.write_all(triple.to_string().as_bytes())?;
        self.writer.write_all(b" .\n")?;
        let _ = self.writer.flush();
        Ok(())
    }
}

pub struct GraphWriter<'a> {
    graph: &'a mut Graph,
}

impl<'a> GraphWriter<'a> {
    pub fn new(graph: &'a mut Graph) -> Self {
        Self { graph }
    }
}

impl<'a> RdfWriter for GraphWriter<'a> {
    fn add_triple(&mut self, triple: TripleRef) -> std::io::Result<()> {
        self.graph.insert(triple.clone());
        Ok(())
    }
}
