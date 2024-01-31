use text_io::read;
use wana_kana::ConvertJapanese;
use wanisabi::{
    response::Error,
    wanikani_client::WanikaniClient,
    wrapper::{assignments::AssignmentsFilter, reviews::ReviewCreate},
};
use wanisabi_model::subject::{Meaning, Subject::*};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let client = WanikaniClient::default();
    let first_review_assignment = client
        .get_assignments_filtered(vec![AssignmentsFilter::ImmediatelyAvailableForReview])
        .await?
        .data[0]
        .clone();
    let first_available_review = client
        .get_subject(first_review_assignment.data.subject_id)
        .await?
        .data;
    let r = match first_available_review {
        Radical(r) => {
            //In practice you'd need to get the character's image if characters was none
            println!("Radical: {:?}", r.characters);
            let (m, _) = meaning_reading(r.meanings, vec![]);
            ReviewCreate {
                subject_id: first_review_assignment.data.subject_id,
                incorrect_meaning_answers: m,
                incorrect_reading_answers: 0,
                created_at: None,
            }
        }
        Kanji(k) => {
            println!("Kanji: {}", k.characters);
            let (m, r) = meaning_reading(
                k.meanings,
                k.readings.iter().map(|x| x.reading.clone()).collect(),
            );
            ReviewCreate {
                subject_id: first_review_assignment.data.subject_id,
                created_at: None,
                incorrect_reading_answers: r,
                incorrect_meaning_answers: m,
            }
        }
        Vocabulary(v) => {
            println!("Vocab: {}", v.characters);
            let (m, r) = meaning_reading(
                v.meanings,
                v.readings.iter().map(|x| x.reading.clone()).collect(),
            );
            ReviewCreate {
                subject_id: first_review_assignment.data.subject_id,
                created_at: None,
                incorrect_reading_answers: r,
                incorrect_meaning_answers: m,
            }
        }
        KanaVocabulary(kv) => {
            println!("Kana Vocab: {}", kv.characters);
            let (m, _) = meaning_reading(kv.meanings, vec![]);
            ReviewCreate {
                subject_id: first_review_assignment.data.subject_id,
                created_at: None,
                incorrect_meaning_answers: m,
                incorrect_reading_answers: 0,
            }
        }
    };
    client.create_review(r).await?;
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
