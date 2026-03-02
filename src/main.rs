use std::collections::VecDeque;
use std::io::{self, BufReader, BufWriter, prelude::*};
use std::path::Path;
use std::{fs::File, path::PathBuf};
use std::env;
use sishuffle::siq::Siq;
use sishuffle::siq::types::{Package, Param, Question, Round, Theme};
use wildmatch::WildMatch;
use clap::{ArgAction, ArgMatches, Command, arg, command, value_parser};

fn filter_questions<F>(content: &mut Package, f: F)
where F: Fn(&Question) -> bool {
    if let Some(rounds) = content.rounds.as_mut() {
        for round in rounds.round.iter_mut() {
            if let Some(themes) = round.themes.as_mut() {
                for theme in themes.theme.iter_mut() {
                    if let Some(questions) = theme.questions.as_mut() {
                        questions.question.retain(&f);
                    }
                }
            }
        }
    }
}

fn filter_themes<F>(content: &mut Package, f: F)
where F: Fn(&Theme) -> bool {
    if let Some(rounds) = content.rounds.as_mut() {
        for round in rounds.round.iter_mut() {
            if let Some(themes) = round.themes.as_mut() {
                themes.theme.retain(&f);
            }
        }
    }
}

fn filter_rounds<F>(content: &mut Package, f: F)
where F: Fn(&Round) -> bool {
    if let Some(rounds) = content.rounds.as_mut() {
        rounds.round.retain(&f);
    }
}

fn wordlist_from_path<P: AsRef<Path>>(path: P) -> Result<Vec<WildMatch>, io::Error> {
    let file = BufReader::new(File::open(path).unwrap());
    let res = file.lines()
        .collect::<Result<Vec<String>, io::Error>>()?
        .into_iter()
        .map(|s| WildMatch::new_case_insensitive(s.as_str()))
        .collect();

    Ok(res)
}

fn question_question_match_pattern(question: &Question, pattern: &WildMatch) -> bool {
    let mut params_queue: VecDeque<&Param> = VecDeque::new();

    if let Some(params) = question.params.as_ref() {
        params_queue.extend(params.param.iter());

        while let Some(param) = params_queue.pop_front() {
            params_queue.extend(param.param.iter().map(|param| &param.value));

            if param.name.as_ref().is_some_and(|name| name == "question") &&
                param.type_.as_ref().is_some_and(|type_| type_ == "content")
            {
                for item in param.item.iter() {
                    if !item.is_ref.as_ref().is_some_and(|is_ref| is_ref == "True") &&
                        item.type_.is_none() 
                    {
                        if pattern.matches(&item.content) {
                            return true;
                        }
                    }
                }
            }
        }
    }

    false
}

fn filter(matches: &ArgMatches) -> Result<(), Box<dyn std::error::Error>> {
    let input_siq_file = File::open(matches.get_one::<PathBuf>("input").expect("required"))?;
    let mut input_siq = Siq::try_new(input_siq_file).unwrap();

    let themes_wordlist = matches.get_one::<PathBuf>("themes-wordlist")
        .map_or(Ok(Vec::new()), wordlist_from_path)?;

    let questions_answers_wordlist = matches.get_one::<PathBuf>("questions-answers-wordlist")
        .map_or(Ok(Vec::new()), wordlist_from_path)?;

    let questions_questions_wordlist= matches.get_one::<PathBuf>("questions-questions-wordlist")
        .map_or(Ok(Vec::new()), wordlist_from_path)?;

    let invert = matches.get_flag("invert");


    filter_themes(&mut input_siq.content, |t|
        !themes_wordlist.iter().any(|pattern| pattern.matches(&t.name)) ^ invert
    );

    filter_questions(&mut input_siq.content, |q|
        !(q.right.answer.iter().any(|answer| 
            questions_answers_wordlist.iter().any(|pattern| pattern.matches(answer))
        ) &&
        q.wrong.as_ref().map_or(true, |answers| 
            answers.answer.iter().any(|answer| 
                questions_answers_wordlist.iter().any(|pattern| pattern.matches(answer))
            )
        )) ^ invert
    );
    filter_questions(&mut input_siq.content, |q|
        !questions_questions_wordlist.iter().any(|pattern| question_question_match_pattern(q, pattern))
    );


    // We are not interested in empty rounds and themes
    filter_themes(&mut input_siq.content, |t|
        t.questions.as_ref().is_some_and(|qs| !qs.question.is_empty())
    );
    filter_rounds(&mut input_siq.content, |r|
        r.themes.as_ref().is_some_and(|ts| !ts.theme.is_empty())
    );


    let output_siq_file =  File::create(matches.get_one::<PathBuf>("output").expect("required")).unwrap();
    let mut output_siq_file = BufWriter::new(output_siq_file);
    
    input_siq.pack(&mut output_siq_file).unwrap();

    Ok(())
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
                .arg(arg!(-n --invert "Invert filters")
                    .action(ArgAction::SetTrue)
                )
                .arg(arg!(--"themes-wordlist" <FILE> "Words to filter themes")
                    .value_parser(value_parser!(PathBuf))
                )
                .arg(arg!(--"questions-answers-wordlist" <FILE> "Words to filter questions by answers")
                    .value_parser(value_parser!(PathBuf))
                )
                .arg(arg!(--"questions-questions-wordlist" <FILE> "Words to filter questions by questions")
                    .value_parser(value_parser!(PathBuf))
                )
        ).get_matches();

    match matches.subcommand() {
        Some(("filter", sub_matches)) => filter(sub_matches),
        _ => unreachable!(),
    }
}
