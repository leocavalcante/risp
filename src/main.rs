use std::fs::File;

use clap::{App, Arg, SubCommand};
use csv::{StringRecord, StringRecordsIter};
use rand::seq::IteratorRandom;

type Whoops<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn rand(records: StringRecordsIter<File>, value: usize) -> Whoops {
    let mut rng = rand::thread_rng();
    let mut wrt = csv::Writer::from_writer(vec![]);

    records
        .choose_multiple(&mut rng, value).iter()
        .flat_map(|result| result)
        .for_each(|record| wrt.write_record(record).unwrap());

    print!("{}", String::from_utf8(wrt.into_inner()?)?);

    Ok(())
}

fn pick(records: StringRecordsIter<File>, index: usize) -> Whoops {
    let mut wrt = csv::Writer::from_writer(vec![]);

    records
        .flat_map(|result| result)
        .flat_map(|record| record.get(index).map(|str| str.to_string()))
        .for_each(|value| wrt.write_record(&[value]).unwrap());

    print!("{}", String::from_utf8(wrt.into_inner()?)?);

    Ok(())
}

fn split(records: StringRecordsIter<File>, by: usize, step: usize) -> Whoops {
    let mut wrt = csv::Writer::from_writer(vec![]);

    records
        .flat_map(|result| result)
        .enumerate()
        .for_each(|(index, record)| {
            if index % by == step {
                wrt.write_record(record.iter()).unwrap()
            }
        });

    print!("{}", String::from_utf8(wrt.into_inner()?)?);

    Ok(())
}

fn chunk(records: StringRecordsIter<File>, size: usize, prefix: &str, headers: StringRecord) -> Whoops {
    let chunks = records.collect::<Vec<_>>();
    let chunks = chunks
        .chunks(size)
        .enumerate()
        .map(|(index, records)| {
            let filename = format!("{}_{}.csv", prefix, index + 1);
            let mut wtr = csv::Writer::from_path(&filename)?;

            wtr.write_record(headers.iter()).unwrap();

            records
                .iter()
                .flat_map(|result| result)
                .for_each(|record| wtr.write_record(record).unwrap());

            wtr.flush()
        })
        .collect::<Vec<_>>();

    println!("{} chunks", chunks.len());

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
        .subcommand(SubCommand::with_name("pick")
            .about("Pick a single column from the list")
            .arg(Arg::with_name("index").required(true).index(1)))
        .subcommand(SubCommand::with_name("split")
            .about("Splits the list [by] return a [step]")
            .arg(Arg::with_name("by").required(true).index(1))
            .arg(Arg::with_name("step").required(true).index(2)))
        .subcommand(SubCommand::with_name("chunk")
            .about("Chunks the list by [size] write to [prefix]")
            .arg(Arg::with_name("size").required(true).index(1))
            .arg(Arg::with_name("prefix").required(true).index(2)))
        .get_matches();

    let path = matches.value_of("input").ok_or("no input provided")?;
    let delimiter = matches.value_of("delimiter").unwrap_or(",");

    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(true)
        .delimiter(*delimiter.as_bytes().get(0).unwrap())
        .from_path(path)?;

    let headers = rdr.headers()?.clone();

    match matches.subcommand() {
        ("rand", args) => {
            let args = args.unwrap();
            let amount = args.value_of("amount");
            let amount = amount.unwrap();

            rand(rdr.records(), amount.parse().unwrap())
        }
        ("pick", args) => {
            let args = args.unwrap();
            let index = args.value_of("index");
            let index = index.unwrap();

            pick(rdr.records(), index.parse().unwrap())
        }
        ("split", args) => {
            let args = args.unwrap();
            let by = args.value_of("by");
            let by = by.unwrap();
            let step = args.value_of("step");
            let step = step.unwrap();

            split(rdr.records(), by.parse().unwrap(), step.parse().unwrap())
        }
        ("chunk", args) => {
            let args = args.unwrap();
            let size = args.value_of("size");
            let size = size.unwrap();
            let prefix = args.value_of("prefix");
            let prefix = prefix.unwrap();

            chunk(rdr.records(), size.parse().unwrap(), prefix, headers)
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
