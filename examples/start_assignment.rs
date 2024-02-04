use chrono::Utc;
use text_io::read;
use wana_kana::ConvertJapanese;
use wanisabi::{
    client::Client,
    wrapper::assignments::{AssignmentsFilter, StartAssignment},
};
use wanisabi_model::subject::{Meaning, Subject::*};

#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    let client = Client::default();
    let lesson_assignments = client
        .get_assignments_filtered(vec![AssignmentsFilter::ImmediatelyAvailableForLessons])
        .await?
        .data;
    if lesson_assignments.len() == 0 {
        println!("No lessons available");
        return Ok(());
    }
    let first_lesson_assignment = lesson_assignments[0].clone();
    let lesson_subj = client
        .get_subject(first_lesson_assignment.data.subject_id)
        .await?
        .data;
    match lesson_subj {
        Radical(r) => {
            println!(
                "Radical {:?}: 
            Meanings: {:?}.
            Meaning mnemonic: {:?}
        ",
                r.characters, r.meanings, r.meaning_mnemonic
            );
            meaning_reading(r.meanings, vec![]);
        }
        Kanji(k) => {
            println!(
                "Kanji {}:
            Meanings: {:?}
            Meaning mnemonic: {}
            Readings: {:?}
            Reading mnemonic: {}
                     ",
                k.characters, k.meanings, k.meaning_mnemonic, k.readings, k.reading_mnemonic
            );
            meaning_reading(
                k.meanings,
                k.readings.iter().map(|x| x.reading.clone()).collect(),
            );
        }
        Vocabulary(v) => {
            println!(
                "
                Vocabulary {}:
                Meanings: {:?}
                Meaning Mnemonic: {}
                Reading: {:?}
                Reading mnemonic: {}
            ",
                v.characters, v.meanings, v.meaning_mnemonic, v.readings, v.reading_mnemonic
            );
            meaning_reading(
                v.meanings,
                v.readings.iter().map(|x| x.reading.clone()).collect(),
            );
        }
        KanaVocabulary(kv) => {
            println!(
                "Kana Vocabulary: {}
                     Meanings: {:?}
                     Meaning Mnemonic: {}",
                kv.characters, kv.meanings, kv.meaning_mnemonic
            );
            meaning_reading(kv.meanings, vec![]);
        }
    };
    client
        .start_assignment(
            &StartAssignment {
                started_at: Some(Utc::now()),
            },
            first_lesson_assignment.id,
        )
        .await?;
    Ok(())
}

fn meaning_reading(meanings: Vec<Meaning>, readings: Vec<String>) -> (i64, i64) {
    let meanings: Vec<String> = meanings.iter().map(|x| x.meaning.to_lowercase()).collect();
    let (mut meaning, mut reading) = ("".to_owned(), "".to_owned());
    let mut ask_meaning = true;
    let mut ask_reading = if readings.len() > 0 { true } else { false };
    let (mut incorrect_meaning, mut incorrect_reading) = (0, 0);
    while !meanings.contains(&meaning.to_lowercase()) && !readings.contains(&reading.to_lowercase())
    {
        if ask_meaning {
            println!("Please enter meaning");
            meaning = read!();
            if meaning.len() > 0 && meanings.contains(&meaning) {
                println!("Meaning correct!");
                ask_meaning = !ask_meaning;
            } else {
                incorrect_meaning += 1;
                println!("Meaning incorrect!");
                println!("Correct meanings: {:?}", meanings);
            }
        }
        if ask_reading {
            println!("please enter reading.");
            reading = read!();
            reading = reading.to_kana();
            if reading.len() > 0 && readings.contains(&reading) {
                println!("Reading correct!");
                ask_reading = !ask_reading;
            } else {
                incorrect_reading += 1;
                println!("Reading incorrect");
                println!("Correct readings: {:?}", readings);
            }
        }
    }
    (incorrect_meaning, incorrect_reading)
}
