use reqwest::Url;
use ::reqwest::header::{CONTENT_TYPE, AUTHORIZATION, ACCEPT};
use serde::{Serialize, Deserialize};

pub(crate) struct SzurubooruContext {
    token: String
}

//const SZURUBOORU_BASE_URL: &str = "https://szurubooru.privacy.qvarford.net/api/";
const SZURUBOORU_BASE_URL: &str = "http://localhost:8080/api/";

impl SzurubooruContext {
    pub(crate) fn new(token: String) -> Self {
        Self { token }
    }

    pub(crate) fn post_is_already_uploaded(&self, url: &Url) -> bool {
        use reqwest::blocking as reqwest;
        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct ReverseSearchRequest {
            content_url: String
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct ReverseSearchResponse {
            exact_post: Option<String>,
            similar_posts: Vec<String>
        }

        let body = serde_json::to_string(&ReverseSearchRequest { content_url: url.to_string() }).unwrap();
        let token = &self.token;
    
        let client = reqwest::Client::new();
        let req = client.post(format!("{SZURUBOORU_BASE_URL}posts/reverse-search"))
        .header(CONTENT_TYPE, "text/json")
        .header(AUTHORIZATION, format!("Token {token}"))
        // required: https://github.com/rr-/szurubooru/blob/780b7dc6fd1830244a6236905a6e8ce9afcfb993/server/szurubooru/rest/app.py#L77
        .header(ACCEPT, "application/json")
        .body(body);
    
        let response = req
            .send()
            .unwrap();
        let status = response.status();
        let text = response.text().unwrap();
        assert!(status.is_success());
        let deserialized: ReverseSearchResponse = serde_json::from_str(&text).unwrap();
    
        return deserialized.exact_post.is_some() || !deserialized.similar_posts.is_empty()
    }
    
    pub(crate) fn upload_post(&self, link: Url) {
        // add needs_tagging tag
        todo!()
    }
}