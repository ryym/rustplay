#[macro_use]
extern crate serde_derive;

#[derive(Serialize, Debug)]
pub struct Payload {
    pub channel: String,
    pub code: String,
    #[serde(rename = "crateType")]
    pub crate_type: String,
    pub mode: String,
    pub tests: bool,
}

#[derive(Deserialize, Debug)]
pub struct RunResult {
    pub stderr: String,
    pub stdout: String,
    pub success: bool,
}
