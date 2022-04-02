use sighashdb::GlobalSighashDB;
use clap::{App, Arg, SubCommand};

fn main() {
    let app =  App::new("sighashdb-cli")
    .arg(
        Arg::with_name("input")
        .long_help("this is the hexadecimal encoded anchor instruction data")
        .help("encoded instruction data")
        .takes_value(true)
        .value_name("IX_DATA")
        .required(true)
    );
    let matches = app.get_matches();
    let input = matches.value_of("input").unwrap();
    let parsed =  GlobalSighashDB.parse_ix_data(input);
    let name = &parsed.0;
    let sighash = &parsed.1;
    match name {
        Some(name) => println!(
            "found known instruction {}. sighash {:?}",
            name, sighash.unwrap(),
        ),
        None => if let Some(sighash) = sighash {
            println!(
                "found unknown instruction. sighash {:?}",
                sighash
            )
        } else {
            println!("failed to parse input");
        }
    }
}