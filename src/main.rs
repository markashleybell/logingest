use std::fs::{self, File};
use std::path::{Path, PathBuf};
use std::io::{BufReader, BufRead, BufWriter, Read};

use anyhow::{Context, Result};
use csv::Writer;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Arguments {
    /// The source log file
    #[structopt(short, long, parse(from_os_str))]
    source: PathBuf,
    /// The output CSV file
    #[structopt(short, long, parse(from_os_str))]
    output: PathBuf,
}

fn rewrite_csv(path: &Path, writer: &mut Writer<BufWriter<File>>) -> Result<(), Box<dyn std::error::Error>> {
    let source_log_file = File::open(path)
        .with_context(|| format!("Could not read file `{}`", path.display()))?;
    
    let mut reader = BufReader::with_capacity(128 * 1024, source_log_file);

    // Skip the IIS log header (4 lines)
    reader.by_ref().lines().take(4).for_each(drop);

    let mut csv = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .from_reader(reader);

    for result in csv.records() {
        let record = result
            .with_context(|| format!("Couldn't read source row in `{}`", path.display()))?;
        
        let c = format!("{} {}", &record[0], &record[1]);

        // SURELY there must be a cleaner way to do this??
        let n = [
            &c,
            &record[2],
            &record[3],
            &record[4],
            &record[5],
            &record[6],
            &record[7],
            &record[8],
            &record[9],
            &record[10],
            &record[11],
            &record[12],
            &record[13],
            &record[14]
        ];

        writer.write_record(n)
            .with_context(|| format!("Couldn't write row to `{}`", path.display()))?;
    }

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arguments = Arguments::from_args();

    let source_log_path = arguments.source.as_path();

    let output_csv_path = arguments.output.as_path();

    let output_csv_file = File::create(output_csv_path)
        .with_context(|| format!("Could not open or create file `{}`", output_csv_path.display()))?;

    let writer = BufWriter::with_capacity(128 * 1024, output_csv_file);

    // We write this out as a *caret* separated file, because SQL BULK INSERT
    // is terrible at handling CSVs, and some of the user agent strings contain
    // all kinds of weird characters (including double quotes)
    let mut csv_writer = csv::WriterBuilder::new()
        .delimiter(b'^')
        .quote_style(csv::QuoteStyle::Never)
        .from_writer(writer);

    let headers = [
        "timestamp", 
        "server_ip", 
        "method", 
        "uri", 
        "query",
        "server_port", 
        "client_username",
        "client_ip", 
        "user_agent", 
        "referer", 
        "status", 
        "substatus", 
        "win32_status",
        "time_taken"
    ];

    csv_writer.write_record(headers)
        .with_context(|| format!("Couldn't write header row to `{}`", output_csv_path.display()))?;

    let csv_writer_ref = &mut csv_writer;

    if source_log_path.is_dir() {
        let entries = fs::read_dir(source_log_path)
            .with_context(|| format!("Couldn't read files from `{}`", source_log_path.display()))?;

        for e in entries {
            let entry = e?;
            let path = entry.path();

            if path.is_file() {
                rewrite_csv(path.as_path(), csv_writer_ref)?;
            }
        }
    } else {
        rewrite_csv(source_log_path, csv_writer_ref)?;
    }

    Ok(())
}
