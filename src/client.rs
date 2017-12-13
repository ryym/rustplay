use super::RunResult;

#[derive(Serialize, Debug)]
struct Payload<'a> {
    channel: String,
    code: &'a String,
    #[serde(rename = "crateType")]
    crate_type: String,
    mode: String,
    tests: bool,
}

pub fn new() -> Client {
    Client{}
}

pub struct Client;

impl Client {
    pub fn run(&self, code: &String) -> RunResult {
        use serde_json;
        use reqwest as r;

        let p = Payload{
            code,
            channel: "stable".to_string(),
            crate_type: "bin".to_string(),
            mode: "debug".to_string(),
            tests: false,
        };

        let body = serde_json::to_string(&p).unwrap();

        let client = r::Client::new();
        let mut res = client.post("https://play.rust-lang.org/execute")
            .header(r::header::ContentType::json())
            .body(body)
            .send().unwrap();

        res.json().unwrap()
    }

    pub fn open(&self, code: &String) {
        use open;
        use urlencoding;

        let query = urlencoding::encode(&code);
        let url = format!("https://play.rust-lang.org/?code={}", query);
        open::that(url).unwrap();
    }
}
