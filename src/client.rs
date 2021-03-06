use config::Config;
use errors::*;

#[derive(Serialize, Debug)]
struct Payload<'a> {
    channel: String,
    code: &'a String,
    #[serde(rename = "crateType")]
    crate_type: String,
    mode: String,
    tests: bool,
}

#[derive(Deserialize, Debug)]
pub struct RunResult {
    pub stderr: String,
    pub stdout: String,
    pub success: bool,
}

pub struct Client {
    conf: Config,
}

const SITE_HOST: &str = "https://play.rust-lang.org";

impl Client {
    pub fn new(conf: Config) -> Self {
        Client { conf }
    }

    pub fn run(&self, code: &String) -> Result<RunResult> {
        use reqwest as r;
        use serde_json;

        let conf = &self.conf;
        let p = Payload {
            code,
            channel: conf.channel(),
            crate_type: "bin".to_string(),
            mode: conf.mode(),
            tests: false,
        };

        let body = serde_json::to_string(&p)?;

        let client = r::Client::new();
        let mut res = client
            .post(&format!("{}/execute", SITE_HOST))
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

        let encoded_code = urlencoding::encode(&code);
        let url = format!(
            "{}/?{}&code={}",
            SITE_HOST,
            conf_to_query(&self.conf),
            encoded_code
        );
        let _ = open::that(url)?;
        Ok(())
    }
}

fn conf_to_query(c: &Config) -> String {
    let qs = [("version", c.channel()), ("mode", c.mode())];
    qs.into_iter()
        .map(|&(ref n, ref v)| format!("{}={}", n, v))
        .collect::<Vec<_>>()
        .join("&")
}
