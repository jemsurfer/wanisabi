use text_io::read;
use wana_kana::ConvertJapanese;
use wanisabi::{
    client::Client,
    wrapper::{assignments::AssignmentsFilter, reviews::ReviewCreate},
};
use wanisabi_model::subject::{Meaning, Subject::*};

#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    let client = Client::default().await?;
    let reviews_assignments = client
        .get_assignments_filtered(vec![AssignmentsFilter::ImmediatelyAvailableForReview])
        .await?
        .data;
    if reviews_assignments.is_empty() {
        println!("No reviews available");
        return Ok(());
    }
    let first_review_assignment = reviews_assignments[0].clone();
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
    let mut ask_meaning = true;
    let mut ask_reading = !readings.is_empty();
    let (mut incorrect_meaning, mut incorrect_reading) = (0, 0);
    while ask_meaning || ask_reading {
        if ask_meaning {
            println!("Please enter meaning");
            let meaning: String = read!("{}\n");
            if meanings.contains(&meaning) {
                println!("Meaning correct!");
                ask_meaning = false;
            } else {
                incorrect_meaning += 1;
                println!("Meaning incorrect!");
                println!("Correct meanings: {:?}", meanings);
            }
        }
        if ask_reading {
            println!("please enter reading.");
            std::thread::sleep(std::time::Duration::from_millis(2));
            let mut reading: String = read!("{}\n");
            reading = reading.to_kana();
            if readings.contains(&reading) {
                println!("Reading correct!");
                ask_reading = false;
            } else {
                incorrect_reading += 1;
                println!("Reading incorrect");
                println!("Correct readings: {:?}", readings);
            }
        }
    }
    (incorrect_meaning, incorrect_reading)
}
