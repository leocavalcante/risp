use std::fs::File;

use clap::{App, Arg, SubCommand};
use csv::StringRecordsIter;
use rand::seq::IteratorRandom;

type Whoops<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn rand(records: StringRecordsIter<File>, value: usize) -> Whoops {
    let mut rng = rand::thread_rng();

    records
        .choose_multiple(&mut rng, value).iter()
        .flat_map(|result| result)
        .map(|record| record.as_slice())
        .for_each(|slice| println!("{}", slice));

    Ok(())
}

fn eval() -> Whoops {
    let matches = App::new("risp")
        .version("0.1.0")
        .about("(rust-based-tool (to-work-with (lists))")
        .author("Leo Cavalcante")
        .arg(Arg::with_name("input")
            .help("File path")
            .required(true)
            .index(1))
        .arg(Arg::with_name("delimiter")
            .help("File delimiter")
            .takes_value(true)
            .short("d").long("delimiter"))
        .subcommand(SubCommand::with_name("rand")
            .about("Gets random values from the list")
            .arg(Arg::with_name("amount").required(true).index(1)))
        .get_matches();

    let path = matches.value_of("input").ok_or("no input provided")?;
    let delimiter = matches.value_of("delimiter").unwrap_or(",");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .delimiter(*delimiter.as_bytes().get(0).unwrap())
        .from_path(path)?;

    match matches.subcommand() {
        ("rand", args) => {
            let args = args.unwrap();
            let amount = args.value_of("amount");
            let amount = amount.unwrap();
            rand(rdr.records(), amount.parse().unwrap())
        }
        _ => Err("command not found".into())
    }
}

fn main() {
    if let Err(e) = eval() {
        eprintln!("Something went wrong: {}", e);
        std::process::exit(1);
    }
}
