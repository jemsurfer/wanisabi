[![Continuous Integration](https://github.com/jemsurfer/wanisabi/actions/workflows/rust.yml/badge.svg)](https://github.com/jemsurfer/wanisabi/actions/workflows/rust.yml)
[![MIT License](https://img.shields.io/badge/License-MIT-green.svg)](https://choosealicense.com/licenses/mit/)
[![Crate version](https://img.shields.io/crates/v/wanisabi)](https://crates.io/crates/wanisabi)

# Wanisabi

An API wrapper for the [wanikani API](https://docs.api.wanikani.com/) written in rust.

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
