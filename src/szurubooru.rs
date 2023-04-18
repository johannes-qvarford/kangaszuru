use std::path::Path;

use reqwest::Url;
use ::reqwest::{header::{CONTENT_TYPE, AUTHORIZATION, ACCEPT}, blocking::multipart::Part, StatusCode};
use serde::{Serialize, Deserialize};

pub(crate) struct SzurubooruContext {
    token: String
}

const SZURUBOORU_BASE_URL: &str = "https://szurubooru.privacy.qvarford.net/api/";
//const SZURUBOORU_BASE_URL: &str = "http://localhost:8080/api/";

impl SzurubooruContext {
    pub(crate) fn new(token: String) -> Self {
        Self { token }
    }

    // TODO: Remove some duplication between methods that take a url vs a path

    pub(crate) fn post_is_already_uploaded(&self, url: &Url) -> bool {
        use reqwest::blocking as reqwest;
        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct ReverseSearchRequest {
            content_url: String
        }

        #[derive(Deserialize, Debug)]
        struct Placeholder {}

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct ReverseSearchResponse {
            exact_post: Option<Placeholder>,
            similar_posts: Vec<Placeholder>
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
        println!("{status}");
        assert!(status.is_success());
        let text = response.text().unwrap();
        let deserialized: ReverseSearchResponse = serde_json::from_str(&text).unwrap();
    
        return deserialized.exact_post.is_some() || !deserialized.similar_posts.is_empty()
    }

    pub(crate) fn post_file_is_already_uploaded(&self, path: &Path) -> bool {
        use reqwest::blocking as reqwest;

        #[derive(Deserialize, Debug)]
        struct Placeholder {}

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct ReverseSearchResponse {
            exact_post: Option<Placeholder>,
            similar_posts: Vec<Placeholder>
        }

        let files = reqwest::multipart::Form::new()
            .file("content", path).unwrap();

        let token = &self.token;
    
        let client = reqwest::Client::new();
        let req = client.post(format!("{SZURUBOORU_BASE_URL}posts/reverse-search"))
        .header(CONTENT_TYPE, "multipart/form-data")
        .header(AUTHORIZATION, format!("Token {token}"))
        // required: https://github.com/rr-/szurubooru/blob/780b7dc6fd1830244a6236905a6e8ce9afcfb993/server/szurubooru/rest/app.py#L77
        .header(ACCEPT, "application/json")
        .multipart(files);
    
        let response = req
            .send()
            .unwrap();
        let status = response.status();
        println!("{status}");
        assert!(status.is_success());
        let text = response.text().unwrap();
        let deserialized: ReverseSearchResponse = serde_json::from_str(&text).unwrap();
    
        return deserialized.exact_post.is_some() || !deserialized.similar_posts.is_empty()
    }
    
    pub(crate) fn upload_post(&self, link: Url) {
        use reqwest::blocking as reqwest;
        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct Request {
            content_url: String,
            tags: Vec<String>,
            safety: String
        }
        let request = Request {
            content_url: link.to_string(),
            tags: vec!["tags_needed".to_owned()],
            safety: "safe".to_owned()
        };

        let body = serde_json::to_string(&request).unwrap();
        let token = &self.token;
    
        let client = reqwest::Client::new();
        let req = client.post(format!("{SZURUBOORU_BASE_URL}posts"))
        .header(CONTENT_TYPE, "text/json")
        .header(AUTHORIZATION, format!("Token {token}"))
        // required: https://github.com/rr-/szurubooru/blob/780b7dc6fd1830244a6236905a6e8ce9afcfb993/server/szurubooru/rest/app.py#L77
        .header(ACCEPT, "application/json")
        .body(body);

        let response = req
            .send()
            .unwrap();
        let status = response.status();
        println!("{status}");
        assert!(status.is_success());
    }

    pub(crate) fn upload_post_file(&self, path: &Path, mut tags: Vec<String>) {
        use reqwest::blocking as reqwest;
        #[derive(Serialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct Request {
            tags: Vec<String>,
            safety: String
        }

        let mut request = Request {
            tags: vec!["tags_needed".to_owned()],
            safety: "safe".to_owned()
        };
        request.tags.append(&mut tags);

        let body = serde_json::to_string(&request).unwrap();
        let files = reqwest::multipart::Form::new()
            .file("content", path).unwrap()
            .part("metadata", Part::text(body).file_name("metadata").mime_str("application/json").unwrap());

        
        let token = &self.token;
    
        let client = reqwest::Client::new();
        let req = client.post(format!("{SZURUBOORU_BASE_URL}posts"))
        .header(CONTENT_TYPE, "multipart/form-data")
        .header(AUTHORIZATION, format!("Token {token}"))
        // required: https://github.com/rr-/szurubooru/blob/780b7dc6fd1830244a6236905a6e8ce9afcfb993/server/szurubooru/rest/app.py#L77
        .header(ACCEPT, "application/json")
        .multipart(files);

        let response = req
            .send()
            .unwrap();
        let status = response.status();
        println!("{status}");
        assert!(status.is_success(), "Failed to upload {path:?}");
    }

    pub(crate) fn resolve_tag(&self, tag: String) -> Vec<String> {
        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct Tag {
            implications: Vec<MicroTag>,
        }

        #[derive(Deserialize, Debug)]
        #[serde(rename_all = "camelCase")]
        struct MicroTag {
            names: Vec<String>
        }

        use reqwest::blocking as reqwest;

        let token = &self.token;
    
        let client = reqwest::Client::new();
        let req = client.get(format!("{SZURUBOORU_BASE_URL}tag/{tag}"))
            .header(AUTHORIZATION, format!("Token {token}"))
            // required: https://github.com/rr-/szurubooru/blob/780b7dc6fd1830244a6236905a6e8ce9afcfb993/server/szurubooru/rest/app.py#L77
            .header(ACCEPT, "application/json");

        let response = req
            .send()
            .unwrap();
        let status = response.status();
        println!("{status}");

        // New tag, no implications yet
        if status == StatusCode::NOT_FOUND {
            println!("New tag '{tag}', no implications yet");
            return vec![tag];
        }

        assert!(status.is_success());

        let text = response.text().unwrap();
        let deserialized: Tag = serde_json::from_str(&text).unwrap();

        let mut implications: Vec<_> = deserialized.implications.into_iter().map(|mt| mt.names.into_iter().next().unwrap()).collect();
        implications.push(tag);

        implications
    }
}