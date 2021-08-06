use std::fs::{File};
use std::path::{Path};
use std::io::{BufReader, BufRead, BufWriter, Read};

fn main() {
    let log_path = match std::env::args().nth(1) {
        Some(lp) => lp,
        None => panic!("No log path specified")
    };

    let csv_path = match std::env::args().nth(2) {
        Some(cp) => cp,
        None => str::replace(&log_path, ".log", ".csv")
    };

    let log_file = Path::new(&log_path);
    let csv_file = Path::new(&csv_path);

    /*
    let reader = buffered_reader(log_path);

    for line in reader.lines() {
        let l = &line.unwrap();

        let t = parse_line(l);

        match t {
            Ok(entry) => { println!("{}", &entry); }
            Err(_) => { println!("Parse Error"); }
        }
    }
    */

    let file = match File::open(log_file) {
        Err(err) => panic!("Couldn't open {}: {}", log_file.display(), err.to_string()),
        Ok(file) => file,
    };

    let mut reader = BufReader::with_capacity(128 * 1024, file);

    reader.by_ref().lines().take(4).for_each(drop);

    let mut csv = csv::ReaderBuilder::new()
        .delimiter(b' ')
        .from_reader(reader);

    let out_path = Path::new(csv_file);

    let out_file = match File::create(out_path) {
        Err(err) => panic!("Couldn't create {}: {}", csv_file.display(), err.to_string()),
        Ok(file) => file,
    };

    let writer = BufWriter::with_capacity(128 * 1024, out_file);

    let mut wtr = csv::WriterBuilder::new()
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

    wtr.write_record(headers).expect("sss");

    for result in csv.records() {
        let record = result.expect("invalid");
        
        let c = format!("{} {}", &record[0], &record[1]);
        // let c = format!("{}{}", str::replace(&record[0], "-", ""), str::replace(&record[1], ":", ""));

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

        wtr.write_record(n).expect("sss");
    }
}
