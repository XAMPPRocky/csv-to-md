extern crate csv;
#[macro_use] extern crate clap;

use std::error::Error;
use std::io::Read;
use std::process;
use std::fs::File;

fn example() -> Result<(), Box<Error>> {
    let matches = clap_app!(csv_to_md =>
        (version: crate_version!())
        (author: "Aaron P. <theaaronepower@gmail.com> + Contributors")
        (about: crate_description!())
        (@arg title: -t --title
            +takes_value
            "Pick header column to be title size('h1').")
        (@arg input: ...  "The file to be parsed.")
    ).get_matches();

    let path = matches.value_of("input").unwrap();
    let title = matches.value_of("title");
    // Build the CSV reader and iterate over each record.
    let records = csv::Reader::from_path(path)?.into_records();
    let headers = csv::Reader::from_path(path)?.headers()?.clone();

    for result in records {
        let record = result?;
        for (header, value) in headers.iter().zip(record.iter()) {
            match title {
                Some(title) if title == header => println!("# {}", value),
                _ => {
                    println!("##### {}", header);
                    println!("{}", value);
                }
            }
        }
    }

    Ok(())
}

fn main() {
    if let Err(err) = example() {
        println!("error running example: {}", err);
        process::exit(1);
    }
}
