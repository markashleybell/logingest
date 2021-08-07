use std::fs::{File};
use std::path::{Path};
use std::io::{BufReader, BufRead, BufWriter, Read};

fn main() {
    let source_log_param = match std::env::args().nth(1) {
        Some(lp) => lp,
        None => panic!("No log path specified"),
    };

    let output_csv_param = std::env::args().nth(2)
        .unwrap_or(str::replace(&source_log_param, ".log", ".csv"));

    let source_log_path = Path::new(&source_log_param);
    let output_csv_path = Path::new(&output_csv_param);

    let source_log_file = match File::open(source_log_path) {
        Ok(file) => file,
        Err(err) => panic!("Couldn't open {}: {}", source_log_path.display(), err.to_string()),
    };

    let mut reader = BufReader::with_capacity(128 * 1024, source_log_file);

    // Skip the IIS log header (4 lines)
    reader.by_ref().lines().take(4).for_each(drop);

    let mut csv = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .from_reader(reader);

    let output_csv_file = match File::create(output_csv_path) {
        Ok(file) => file,
        Err(err) => panic!("Couldn't create {}: {}", output_csv_path.display(), err.to_string()),
    };

    let writer = BufWriter::with_capacity(128 * 1024, output_csv_file);

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

    csv_writer.write_record(headers).expect("Couldn't write header row");

    for result in csv.records() {
        let record = result.expect("Couldn't read row");
        
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

        csv_writer.write_record(n).expect("Couldn't write row");
    }
}
