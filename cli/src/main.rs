use clap::{App, Arg, SubCommand};
use sighashdb::GlobalSighashDB;

fn main() {
    let app =  App::new("sighashdb-cli")
    .subcommands(vec![
        SubCommand::with_name("parse")
        .about("parse the encoded instruction data to determine what instruction is being called")
        .arg(
            Arg::with_name("input")
            .long("input")
            .long_help("this is the hexadecimal encoded anchor instruction data")
            .help("encoded instruction data")
            .takes_value(true)
            .value_name("IX_DATA")
            .required(true)
        ),
        SubCommand::with_name("calculate")
        .about("given the name, calculate the sighash")
        .arg(
            Arg::with_name("input")
            .long("input")
            .long_help("this is the name of the instruction as is written in the rust code")
            .takes_value(true)
            .value_name("IX_NAME")
            .required(true)
        )
        .arg(
            Arg::with_name("v6")
            .long("v6")
            .long_help("if enabled, use anchorv6 instruction name spacing which is 2 colons :: instead of 1 :")
            .takes_value(false)
            .required(false)
        )
        .arg(
            Arg::with_name("account")
            .long("account")
            .long_help("if enabled, use the account namespace")
            .takes_value(false)
            .required(false)
        )
    ])
;
    let matches = app.get_matches();
    match matches.subcommand() {
        ("parse", Some(parse)) => {
            let input = parse.value_of("input").unwrap();
            let parsed = GlobalSighashDB.parse_ix_data(input);
            let name = &parsed.0;
            let sighash = &parsed.1;
            match name {
                Some(name) => println!(
                    "found known instruction {}. sighash {:?}",
                    name,
                    sighash.unwrap(),
                ),
                None => {
                    if let Some(sighash) = sighash {
                        println!("found unknown instruction. sighash {:?}", sighash)
                    } else {
                        println!("failed to parse input");
                    }
                }
            }
        }
        ("calculate", Some(calculate)) => {
            let spacer = if calculate.is_present("v6") && !calculate.is_present("account") {
                "::"
            } else {
                ":"
            };
            let namespace = if calculate.is_present("account") {
                "account"
            } else {
                "global"
            };
            let msg_to_hash = format!("{namespace}{spacer}{}", calculate.value_of("input").unwrap());
            {
                use ring::digest::{Context, SHA256};
                let mut context = Context::new(&SHA256);
                context.update(msg_to_hash.as_bytes());
                let digest = context.finish();
                let sighash = format!("{:?}", &digest.as_ref()[0..8]);
                println!("{} | {}", calculate.value_of("input").unwrap(), sighash);
            }
        }
        _ => panic!("invalid command, run --help for more information"),
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_extract_sighash_from_ix_data() {
        let got_data = "8e1eb763fd2f23a6";
        let parsed = GlobalSighashDB.parse_ix_data(got_data);
        let (name, _sighash) = (parsed.0.unwrap(), parsed.1.unwrap());
        assert_eq!(name, "ix1");
    }
}
