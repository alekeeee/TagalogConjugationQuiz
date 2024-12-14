use rand::seq::SliceRandom;
use serde_json::from_reader;
use std::collections::{HashMap, HashSet};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::File;
use std::io::{self, Write};

#[derive(Debug, Serialize, Deserialize)]
struct Verb {
    english: String,
    root: String,
    actor_completed: String,
    actor_incompleted: String,
    actor_contempated: String,
    object_completed: String,
    object_incompleted: String,
    object_emplated: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    // Load the dictionary from the JSON file
    let file = File::open("data.txt")?;
    let data: HashMap<String, Vec<String>> = from_reader(file)?;

    let mut verbs: Vec<Verb> = Vec::new();

    for (english, forms) in &data {
        if forms.len() == 7 {
            verbs.push(Verb{
                english: english.clone(),
                root: forms[0].clone(),
                actor_completed: forms[1].clone(),
                actor_incompleted: forms[2].clone(),
                actor_contempated: forms[3].clone(),
                object_completed: forms[4].clone(),
                object_incompleted: forms[5].clone(),
                object_emplated: forms[6].clone(),
            });
        }
    }

    let mut rng = rand::thread_rng();
    let mut root_words: Vec<_> = data.keys().collect();

    println!("Welcome to the Tagalog Conjugation Quiz Game!");
    println!("You will be given an english word, and you need to state the tagalog root word and its possible conjugations.");
    println!("If there is only an Actor or Object focus conjugation type for a root, leave the rest blank by pressing enter");
    println!("Type 'exit' at any time to quit the game.\n");

    loop {
        // Select a random root word
        if let Some(root_word) = root_words.choose(&mut rng) {
            let conjugations = &data[*root_word];

            println!("Root word: {}", root_word);
            println!("Please enter all {} conjugations:", conjugations.len());

            let mut user_conjugations = HashSet::new();
            for i in 0..conjugations.len() {
                print!("{}: ", i + 1);
                io::stdout().flush()?;

                let mut input = String::new();
                io::stdin().read_line(&mut input)?;
                let input = input.trim().to_string();

                // Allow the user to exit the game
                if input.eq_ignore_ascii_case("exit") {
                    println!("Thanks for playing!");
                    return Ok(());
                }

                user_conjugations.insert(input.to_lowercase());
            }

            // Check the user's answers
            let correct_answers: HashSet<_> = conjugations.iter().map(|s| s.to_lowercase()).collect();

            if correct_answers == user_conjugations {
                println!("Great job! All answers are correct.\n");
            } else {
                println!("Some answers are incorrect. Here are the correct conjugations:");
                for (i, conjugation) in conjugations.iter().enumerate() {
                    println!("{}. {}", i + 1, conjugation);
                }
                println!();
            }
        } else {
            println!("No root words found in the dictionary.");
            break;
        }
    }

    Ok(())
}
