use clap::{Arg, ArgMatches, Command, arg, builder::{StringValueParser, TypedValueParser, ValueParserFactory}, command, value_parser};
use sitool::siq::Siq;
use uuid::Uuid;
use wildmatch::WildMatch;
use std::{env, fs::{self, File}, io::{self, BufRead, BufReader, BufWriter}, path::PathBuf};
use sitool::siq::types::{Package, Round, Theme};

fn filter_themes<F>(content: &mut Package, f: F)
where F: Fn(&Theme) -> bool {
    for round in content.rounds.iter_mut() {
        round.themes.retain(&f);
    }
}

fn filter_rounds<F>(content: &mut Package, f: F)
where F: Fn(&Round) -> bool {
    content.rounds.retain(&f);
}

fn show_themes(content: &Package) {
    for round in content.rounds.iter() {
        for theme in round.themes.iter() {
            println!("{}", theme.name);
        }
    }
}

fn filter(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let input_siq_path = matches.get_one::<PathBuf>("input")
        .expect("required");
    let mut input_siq = Siq::try_new(input_siq_path)?;

    let output_siq_file =  File::create(matches.get_one::<PathBuf>("output").expect("required"))
        .unwrap();
    let mut output_siq_file = BufWriter::new(output_siq_file);

    let themes_filter = matches.get_one::<Filter>("themes-filter");

    let show_themes_opt = matches.get_flag("show-themes");

    if let Some(themes_filter) = themes_filter {
        filter_themes(&mut input_siq.content, |t|
            !themes_filter.wordlist.iter().any(|pattern| pattern.matches(&t.name)) ^ themes_filter.inverted
        );
    }

    // We are not interested in empty rounds and themes
    filter_themes(&mut input_siq.content, |t|
        !t.questions.is_empty()
    );
    filter_rounds(&mut input_siq.content, |r|
        !r.themes.is_empty()
    );

    if show_themes_opt {
        show_themes(&input_siq.content);
    }
    
    input_siq.pack(&mut output_siq_file)?;

    Ok(())
}

fn repack(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let max_rounds: usize = matches.get_one::<u32>("max-rounds")
        .expect("has default")
        .to_owned()
        .try_into()
        .expect("we don't support weird architectures");

    let max_themes: usize = matches.get_one::<u32>("max-themes")
        .expect("has default")
        .to_owned()
        .try_into()
        .expect("we don't support weird architectures");

    // Clap has to pass only positive values
    assert!(max_themes > 0);
    assert!(max_rounds > 0);

    let logo_path = matches.get_one::<PathBuf>("logo");

    let dir_path = matches.get_one::<PathBuf>("input")
        .expect("required");
    let dir = fs::read_dir(dir_path)?;

    let out_dir_path = matches.get_one::<PathBuf>("output")
        .expect("required");

    let mut accumulator = Package::default();

    if let Some(logo) = logo_path {
        accumulator.set_logo(logo);
    }

    let mut tmp_round = Round::default();

    for entry in dir {
        let path = entry?.path();
        
        // We are interested only in siq files
        if path.extension().is_none_or(|ext| ext != "siq") {
            continue;
        }

        let siq = if let Ok(sq) = Siq::try_new(path) {
            sq
        } else {
            continue;
        };

        for round in siq.content.rounds.iter() {
            for theme in round.themes.iter() {
                if tmp_round.themes.len() < max_themes {
                    tmp_round.themes.push(theme.clone());
                } else {
                    accumulator.rounds.push(tmp_round);
                    tmp_round = Round::default();
                }

                if accumulator.rounds.len() == max_rounds {
                    // Generate name for the output siq file
                    let dest_filename = Uuid::new_v4()
                        .to_string();
                    let dest_filename = PathBuf::from(dest_filename)
                        .with_extension("siq");

                    let dest_path = out_dir_path.join(dest_filename);

                    let file = File::create(dest_path)?;
                    let mut file = BufWriter::new(file);

                    let old_accumulator = accumulator;
                    accumulator = Package::default();
                    
                    let output_siq: Siq = old_accumulator.into();
                    output_siq.pack(&mut file)?;
                }
            }
        }

    }

    if !tmp_round.themes.is_empty() {
        accumulator.rounds.push(tmp_round);
    }

    if !accumulator.rounds.is_empty() {
        let dest_filename = Uuid::new_v4()
            .to_string();
        let dest_filename = PathBuf::from(dest_filename)
            .with_extension("siq");

        let dest_path = out_dir_path.join(dest_filename);

        let file = File::create(dest_path)?;
        let mut file = BufWriter::new(file);

        let output_siq: Siq = accumulator.into();
        output_siq.pack(&mut file)?;
    }
    
    Ok(())
}

fn reprice(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let input_siq_path = matches.get_one::<PathBuf>("input")
        .expect("required");
    let mut input_siq = Siq::try_new(input_siq_path)?;

    let output_siq_path = matches.get_one::<PathBuf>("output")
        .expect("required");
    let mut output_siq = File::create(output_siq_path)?;

    let base_price = matches.get_one::<i32>("base-price")
        .expect("has default")
        .to_owned();

    let base_max_price = matches.get_one::<i32>("base-max-price")
        .expect("has default")
        .to_owned();

    let base_incr = matches.get_one::<i32>("base-increment")
        .expect("has default")
        .to_owned();

    let round_base_price_incr = matches.get_one::<i32>("round-base-price-increment")
        .expect("has default")
        .to_owned();

    let mut cur_base_price = base_price;
    let mut cur_base_max_price = base_max_price;

    for round in input_siq.content.rounds.iter_mut() {
        for theme in round.themes.iter_mut() {
            let mut cur_price = cur_base_price;

            for question in theme.questions.iter_mut() {
                question.price = cur_price;

                cur_price = Ord::min(cur_base_max_price, cur_price + base_incr);
            }
        }

        cur_base_price += round_base_price_incr;
        cur_base_max_price += round_base_price_incr;
    }

    input_siq.pack(&mut output_siq)?;

    Ok(())
}

#[derive(Clone)]
struct Filter {
    wordlist: Vec<WildMatch>,
    inverted: bool,
}

impl ValueParserFactory for Filter {
    type Parser = FilterValueParser;
    fn value_parser() -> Self::Parser {
        FilterValueParser
    }
}

#[derive(Clone)]
struct FilterValueParser;

impl TypedValueParser for FilterValueParser {
    type Value = Filter;

    fn parse_ref(
            &self,
            cmd: &Command,
            arg: Option<&Arg>,
            value: &std::ffi::OsStr,
        ) -> Result<Self::Value, clap::Error> {
        let inner = StringValueParser::new();
        let val = inner.parse_ref(cmd, arg, value)?;

        let (path, inverted) = if val.starts_with('!') {
            (PathBuf::from(&val[1..]), true)
        } else {
            (PathBuf::from(val), false)
        };

        let file = File::open(path)?;
        let file = BufReader::new(file);

        let wordlist = file.lines()
            .map(|r| r.map(|l| WildMatch::new(l.as_str())))
            .collect::<Result<Vec<WildMatch>, io::Error>>()?;

        let res = Self::Value {
            wordlist: wordlist,
            inverted: inverted,
        };

        Ok(res)
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
        .subcommand(
        Command::new("filter")
            .about("Filter out questions by key words")
            .arg(arg!(-i --input <FILE> "Input siq file")
                .value_parser(value_parser!(PathBuf))
                .required(true)
            )
            .arg(arg!(-o --output <FILE> "Output siq file")
                .value_parser(value_parser!(PathBuf))
                .required(true)
            )
            .arg(arg!(--"themes-filter" <FILTER> "Filter for themes")
                .value_parser(value_parser!(Filter))
            )
            .arg(arg!(--"show-themes"))
            .after_help("FILTER - [!]WORDLIST_FILE\nUse ! to invert filter")
        )
        .subcommand(
            Command::new("repack")
            .about("Repack Siq files")
            .arg(arg!(-i --input <DIR> "Dir with input Siqs")
                .value_parser(value_parser!(PathBuf))
                .required(true)
            )
            .arg(arg!(-o --output <DIR> "Dir for output Siqs")
                .value_parser(value_parser!(PathBuf))
                .required(true)
            )
            .arg(arg!(--"max-rounds" <NR> "Max rounds in output Siqs")
                .value_parser(value_parser!(u32).range(1..))
                .default_value("4")
            )
            .arg(arg!(--"max-themes" <NR> "Max themes in 1 round")
                .value_parser(value_parser!(u32).range(1..))
                .default_value("7")
            )
            .arg(arg!(--logo <FILE> "Logo for repacked packs")
                .value_parser(value_parser!(PathBuf))
            )
        )
        .subcommand(
        Command::new("reprice")
            .about("fix questions prices")
            .arg(arg!(-i --input <FILE> "Input siq file")
                .value_parser(value_parser!(PathBuf))
                .required(true)
            )
            .arg(arg!(-o --output <FILE> "Output siq file")
                .value_parser(value_parser!(PathBuf))
                .required(true)
            )
            .arg(arg!(--"base-price" <NR> "Base price")
                .value_parser(value_parser!(i32))
                .default_value("100")
            )
            .arg(arg!(--"base-max-price" <NR> "Base max price")
                .value_parser(value_parser!(i32))
                .default_value("800")
            )
            .arg(arg!(--"base-increment" <NR> "Base increment")
                .value_parser(value_parser!(i32))
                .default_value("100")
            )
            .arg(arg!(--"round-base-price-increment" <NR> "Per round base price increment")
                .value_parser(value_parser!(i32))
                .default_value("100")
            )
        )
        .get_matches();

    match matches.subcommand() {
        Some(("filter", sub_matches)) => filter(sub_matches),
        Some(("repack", sub_matches)) => repack(sub_matches),
        Some(("reprice", sub_matches)) => reprice(sub_matches),
        _ => {
            println!("Unknown command");
            Ok(())
        },
    }
}
