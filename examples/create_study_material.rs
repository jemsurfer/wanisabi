use text_io::read;
use wanisabi::{client::Client, wrapper::study_materials::StudyMaterialCreate};
use wanisabi_model::subject::Subject::*;
#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    println!("Enter a subject ID you wish to add a note to.");
    let client = Client::default().await?;
    let subject_id: i64 = read!("{}\n");
    let s = client.get_subject(subject_id).await?.data;
    let (meaning_note, meaning_synonyms, reading_note) = match s {
        Radical(r) => {
            println!("Please enter a meaning note for Radical {:?}", r.characters);
            meaning_reading_note_and_synonyms(false)
        }
        KanaVocabulary(kv) => {
            println!(
                "Please enter a meaning note for Kana Vocab {}",
                kv.characters
            );
            meaning_reading_note_and_synonyms(false)
        }
        Kanji(k) => {
            println!("Please enter a meaning note for Kanji {}", k.characters);
            meaning_reading_note_and_synonyms(true)
        }
        Vocabulary(v) => {
            println!(
                "Please enter a meaning note for Vocabulary {}",
                v.characters
            );
            meaning_reading_note_and_synonyms(true)
        }
    };
    if meaning_note.is_some() || reading_note.is_some() || meaning_synonyms.is_some() {
        let s_create = StudyMaterialCreate {
            subject_id,
            meaning_note,
            meaning_synonyms,
            reading_note,
        };
        let a = client.create_study_material(s_create).await?;
        println!("Successfully created study material: {:?}", a.data);
    }
    Ok(())
}

fn meaning_reading_note_and_synonyms(
    collect_reading: bool,
) -> (Option<String>, Option<Vec<String>>, Option<String>) {
    let meaning_note: String = read!("{}\n");
    let meaning_note = if !meaning_note.is_empty() {
        Some(meaning_note)
    } else {
        None
    };
    let mut synonyms = vec![];
    loop {
        println!("Enter a synonym or type /stop to stop");
        let inp: String = read!("{}\n");
        if inp == "/stop" {
            break;
        }
        synonyms.push(inp);
    }
    let synonyms = if !synonyms.is_empty() {
        Some(synonyms)
    } else {
        None
    };
    let reading_note = if collect_reading {
        println!("Enter a reading note");
        Some(read!("{}\n"))
    } else {
        None
    };
    (meaning_note, synonyms, reading_note)
}
