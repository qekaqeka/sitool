use std::io::{self, prelude::*, BufReader};
use std::{fs::File, path::PathBuf};
use std::env;
use sishuffle::siq::Siq;
use sishuffle::siq::types::mstns::PackageRoundsRoundThemesThemeElementType;
use std::collections::HashSet;
use clap::{Arg, ArgMatches, Command, arg, command, value_parser};

fn theme_is_empty(theme: &PackageRoundsRoundThemesThemeElementType) -> bool {
    if let Some(questions) = &theme.questions {
        questions.question.is_empty()
    } else {
        true
    }
}

fn filter(matches: &ArgMatches) {
    let input_siq_path = File::open(matches.get_one::<PathBuf>("input").unwrap()).unwrap();
    let mut input_siq = Siq::try_new(input_siq_path).unwrap();

    let mut themes_wordlist: HashSet<String> = HashSet::new();
    let mut questions_answers_wordlist: HashSet<String> = HashSet::new();
    let mut questions_questions_wordlist: HashSet<String> = HashSet::new();

    if let Some(themes_wordlist_path) = matches.get_one::<PathBuf>("themes-wordlist") {
        let themes_wordlist_file = BufReader::new(File::open(themes_wordlist_path).unwrap());
        themes_wordlist = themes_wordlist_file.lines().collect::<Result<HashSet<String>, io::Error>>().unwrap();
    }

    if let Some(questions_answers_wordlist_path) = matches.get_one::<PathBuf>("questions-answers-wordlist") {
        let questions_answers_wordlist_file = BufReader::new(File::open(questions_answers_wordlist_path).unwrap());
        questions_answers_wordlist = questions_answers_wordlist_file.lines().collect::<Result<HashSet<String>, io::Error>>().unwrap();
    }

    if let Some(questions_questions_wordlist_path) = matches.get_one::<PathBuf>("questions-questions-worlist") {
        let questions_questions_wordlist_file = BufReader::new(File::open(questions_questions_wordlist_path).unwrap());
        questions_questions_wordlist = questions_questions_wordlist_file.lines().collect::<Result<HashSet<String>, io::Error>>().unwrap();
    }

    for round in input_siq.package.rounds.as_mut().unwrap().round.iter_mut() {
        let themes = &mut round.themes.as_mut().unwrap().theme;
        themes.retain(|theme| !themes_wordlist.contains(&theme.name));

        for theme in themes.iter_mut() {
            let questions = &mut theme.questions.as_mut().unwrap().question;
        }

        themes.retain(|theme| !theme_is_empty(theme));
    }

    dbg!(input_siq.package);
}

fn main() {
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
                .arg(arg!(-n "Invert filters")
                )
                .arg(arg!(--"themes-wordlist" <FILE> "Words to filter themes")
                    .value_parser(value_parser!(PathBuf))
                )
                .arg(arg!(--"questions-answers-wordlist" <FILE> "Words to filter questions by answers")
                    .value_parser(value_parser!(PathBuf))
                )
                .arg(arg!(--"questions-questions-worlist" <FILE> "Words to filter questions by questions")
                    .value_parser(value_parser!(PathBuf))
                )
        ).get_matches();

    match matches.subcommand() {
        Some(("filter", sub_matches)) => filter(sub_matches),
        _ => unreachable!(),
    }

}
