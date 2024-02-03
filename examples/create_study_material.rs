use text_io::read;
use wanisabi::{wanikani_client::WanikaniClient, wrapper::study_materials::StudyMaterialCreate};
use wanisabi_model::subject::Subject::*;
#[tokio::main]
async fn main() -> Result<(), wanisabi::Error> {
    println!("Enter a subject ID you wish to add a note to.");
    let client = WanikaniClient::default();
    let id: i64 = read!();
    let s = client.get_subject(id).await?.data;
    let (meaning_note, meaning_synonyms, reading_note) = match s {
        Radical(r) => {
            println!("Please enter a meaning note for {r:?}");
            let meaning_note: String = read!();
            let mut synonyms = vec![];
            loop {
                println!("Enter a synonym or type /stop to stop");
                let inp: String = read!();
                if inp == "/stop" {
                    break;
                }
                synonyms.push(inp);
            }
            (meaning_note, synonyms, "".to_owned())
        }
        KanaVocabulary(k) => {
            println!("Please enter a ")
        }
    };
    Ok(())
}
