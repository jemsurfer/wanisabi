[![Continuous Integration](https://github.com/GitBluub/wanikani-rs/actions/workflows/ci.yml/badge.svg)](https://github.com/GitBluub/wanikani-rs/actions/workflows/ci.yml)
[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)
[![Crate version](https://img.shields.io/crates/v/wanikani-rs)](https://crates.io/crates/wanikani-rs)

# wanikani-rs
An API wrapper for the [wanikani API](https://wanikani.com/api) written in rust.


## Usage/Examples

```rust
#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = WanikaniClient::new("<WANIKANI_API_KEY>");
    let params = vec![
        AssignmentsFilter::ImmediatelyAvailableForLessons,
    ];

    let assignments = client.get_assignments_filtered(params).await?;
}
```


## License

[MIT](https://choosealicense.com/licenses/mit/)


