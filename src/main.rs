use std::collections::HashMap;
use std::env;
use std::error::Error;
use std::fs::File;
use std::io;
use std::process;

mod handle;
mod types;
use types::*;

fn process_csv() -> Result<Accounts, Box<dyn Error>> {
    let mut accounts: Accounts = HashMap::new();
    let mut transactions: Disputes = HashMap::new();

    let args: Vec<String> = env::args().collect();
    let f = File::open(&args[1])?;
    let reader = io::BufReader::new(f);

    let mut rdr = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_reader(reader);
    for result in rdr.deserialize() {
        let record: Record = result?;
        match record.r#type.as_str() {
            "deposit" => handle::deposit(&mut accounts, &mut transactions, record),
            "withdrawal" => handle::withdrawal(&mut accounts, &mut transactions, record),
            "dispute" => handle::dispute(&mut accounts, &mut transactions, record),
            "resolve" => handle::resolve(&mut accounts, &mut transactions, record),
            "chargeback" => handle::chargeback(&mut accounts, &mut transactions, record),
            _ => {}
        }
    }
    Ok(accounts)
}

fn main() {
    let mut wtr = csv::Writer::from_writer(io::stdout());
    match process_csv() {
        Ok(accounts) => {
            for (client, account) in accounts {
                let Account {
                    frozen,
                    amount,
                    held,
                } = account;
                let account_output = AccountOutput {
                    client,
                    frozen,
                    amount,
                    held,
                    total: amount + held,
                };
                wtr.serialize(account_output).unwrap();
            }
            wtr.flush().unwrap();
        }
        Err(err) => {
            println!("error running process_csv: {}", err);
            process::exit(1);
        }
    }
}
