use rand::seq::IteratorRandom;

type Whoops<T = ()> = Result<T, Box<dyn std::error::Error>>;

fn rand(path: &String, value: &String) -> Whoops {
    let value: usize = value.parse()?;
    let mut rng = rand::thread_rng();

    rdr.records()
        .choose_multiple(&mut rng, value).iter()
        .flat_map(|result| result)
        .map(|record| record.as_slice())
        .for_each(|slice| println!("{}", slice));

    Ok(())
}

fn eval() -> Whoops {
    let args = std::env::args();
    let args: Vec<String> = args.collect();

    let input_path = args.get(1).ok_or("input file is missing")?;
    let command = args.get(2).ok_or("command not provided")?;

    match command.as_str() {
        "rand" => rand(input_path, args.get(3).ok_or("rand value not provided")?),
        _ => {
            return Err(format!("command {} not found", command).into());
        }
    }?;

    Ok(())
}

fn main() {
    if let Err(e) = eval() {
        eprintln!("Something went wrong: {}", e);
        std::process::exit(1);
    }
}
