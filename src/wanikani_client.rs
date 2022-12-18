#[derive(Clone, Debug, Default)]
struct WanikaniClient {
    pub key: String
}

impl WanikaniClient {
    pub fn new(key: String) -> Self {
        Self { key }
    }
    // add code here
}
