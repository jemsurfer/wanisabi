use text_io::read;
use wana_kana::ConvertJapanese;
use wanikani_rs::{
    wanikani_client::WanikaniClient,
    wrapper::{assignments::AssignmentsFilter, reviews::ReviewCreate},
};
use wanikani_rs_model::subject::{Meaning, Subject::*};
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = WanikaniClient::default();
    let first_lesson_assignment = client
        .get_assignments_filtered(vec![AssignmentsFilter::ImmediatelyAvailableForLessons])
        .await?
        .data[0]
        .clone();
    let lesson_subj = client
        .get_subject(first_lesson_assignment.data.subject_id)
        .await?
        .data;
    let r = match lesson_subj {
        Radical(r) => {
            println!(
                "Radical {:?}: 
            Meanings: {:?}.
            Meaning mnemonic: {:?}
        ",
                r.characters, r.meanings, r.meaning_mnemonic
            );
            let (m, r) = meaning_reading(r.meanings, vec![]);
            ReviewCreate {
                subject_id: first_lesson_assignment.data.subject_id,
                incorrect_meaning_answers: m,
                incorrect_reading_answers: 0,
                created_at: Some(chrono::Utc::now()),
            }
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
            let (m, r) = meaning_reading(
                k.meanings,
                k.readings.iter().map(|x| x.reading.clone()).collect(),
            );
            ReviewCreate {
                subject_id: first_lesson_assignment.data.subject_id,
                incorrect_meaning_answers: m,
                incorrect_reading_answers: r,
                created_at: ,
            }
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
            ReviewCreate {
                subject_id: todo!(),
                incorrect_meaning_answers: todo!(),
                incorrect_reading_answers: todo!(),
                created_at: todo!(),
            }
        }
        KanaVocabulary(kv) => {
            println!(
                "Kana Vocabulary: {}
                     Meanings: {:?}
                     Meaning Mnemonic: {}",
                kv.characters, kv.meanings, kv.meaning_mnemonic
            );
            ReviewCreate {
                subject_id: todo!(),
                incorrect_meaning_answers: todo!(),
                incorrect_reading_answers: todo!(),
                created_at: todo!(),
            }
        }
    };
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
