use super::RunResult;
use errors::*;
use config::Config;

#[derive(Serialize, Debug)]
struct Payload<'a> {
    channel: String,
    code: &'a String,
    #[serde(rename = "crateType")]
    crate_type: String,
    mode: String,
    tests: bool,
}

pub fn new(conf: Config) -> Client {
    Client{conf}
}

pub struct Client {
    conf: Config,
}

impl Client {
    pub fn run(&self, code: &String) -> Result<RunResult> {
        use serde_json;
        use reqwest as r;

        let p = Payload{
            code,
            channel: self.conf.channel(),
            crate_type: "bin".to_string(),
            mode: "debug".to_string(),
            tests: false,
        };

        let body = serde_json::to_string(&p)?;

        let client = r::Client::new();
        let mut res = client.post("https://play.rust-lang.org/execute")
            .header(r::header::ContentType::json())
            .body(body)
            .send()?;

        let ret: RunResult = res.json()?;
        if ret.success {
            Ok(ret)
        } else {
            Err(ret.stderr.into())
        }
    }

    pub fn open(&self, code: &String) -> Result<()> {
        use open;
        use urlencoding;

        let query = urlencoding::encode(&code);
        let url = format!("https://play.rust-lang.org/?code={}", query);
        let _ = open::that(url)?;
        Ok(())
    }
}
