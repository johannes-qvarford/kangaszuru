use chrono::Days;
use ::reqwest::header::ACCEPT;
use reqwest::{Url, header::{CONTENT_TYPE, AUTHORIZATION}};
use serde::{Deserialize, Serialize};

pub fn perform(miniflux_token: &str, szurubooru_token: &str) {
    let posts = download_starred_miniflux_posts(miniflux_token);

    let media_links: Vec<_> = posts.iter().flat_map(|post| post.media_links()).collect();
    println!("Media links are: {media_links:?}");

    let new_media_links: Vec<_> = media_links.into_iter().filter(|link| !post_is_already_uploaded(szurubooru_token, link)).collect();
    println!("New Media links are: {new_media_links:?}");

    for link in new_media_links {
        upload_szurubooru_post(szurubooru_token, link);
    }

    for id in posts.iter().map(|post| post.id) {
        unfavorite_miniflux_post(miniflux_token, id);
    }
}

#[derive(Deserialize, Debug)]
struct EntriesResponse {
    entries: Vec<Post>
}

#[derive(Deserialize, Debug)]
struct Post {
    id: i64,
    content: String
}

impl Post {
    fn media_links(&self) -> Vec<Url> {
        use scraper::{Selector, Html};

        let document = Html::parse_fragment(&self.content);
        let image_selector = Selector::parse("img").unwrap();
        let image_links = document.select(&image_selector).map(|elem| elem.value().attr("src"));

        let video_selector = Selector::parse("video source").unwrap();
        let video_links = document.select(&video_selector).map(|elem| elem.value().attr("src"));
        
        image_links.chain(video_links)
            .map(|x| Url::try_from(x.unwrap()).unwrap())
            .collect()
    }
}

fn download_starred_miniflux_posts(miniflux_token: &str) -> Vec<Post> {
    use reqwest::blocking as reqwest;

    let earliest = chrono::offset::Utc::now().checked_sub_days(Days::new(7)).unwrap().timestamp();
    let client = reqwest::Client::new();
    let response = client.get(format!("https://miniflux.privacy.qvarford.net/v1/entries?after={earliest}&starred=true"))
        .header("X-Auth-Token", miniflux_token)
        .send()
        .unwrap();
    assert!(response.status().is_success());

    let text = response.text().unwrap();
    let deserialized: EntriesResponse = serde_json::from_str(&text).unwrap();
    
    println!("deserialized: {deserialized:?}");

    deserialized.entries
}
//const SZURUBOORU_BASE_URL: &str = "https://szurubooru.privacy.qvarford.net/api/";
const SZURUBOORU_BASE_URL: &str = "http://localhost:8080/api/";

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

fn post_is_already_uploaded(szurubooru_token: &str, url: &Url) -> bool {
    use reqwest::blocking as reqwest;

    let body = serde_json::to_string(&ReverseSearchRequest { content_url: url.to_string() }).unwrap();

    let client = reqwest::Client::new();
    let req = client.post(format!("{SZURUBOORU_BASE_URL}posts/reverse-search"))
    .header(CONTENT_TYPE, "text/json")
    .header(AUTHORIZATION, format!("Token {szurubooru_token}"))
    // required: https://github.com/rr-/szurubooru/blob/780b7dc6fd1830244a6236905a6e8ce9afcfb993/server/szurubooru/rest/app.py#L77
    .header(ACCEPT, "application/json")
    .body(body);
    dbg!(&req);
    let response = req
        .send()
        .unwrap();
    let status = response.status();
    let text = response.text().unwrap();
    assert!(status.is_success());
    let deserialized: ReverseSearchResponse = serde_json::from_str(&text).unwrap();

    return deserialized.exact_post.is_some() || !deserialized.similar_posts.is_empty()
}

fn upload_szurubooru_post(szurubooru_token: &str, link: Url) {
    // add needs_tagging tag
    todo!()
}

fn unfavorite_miniflux_post(miniflux_token: &str, id: i64) {
    todo!()
}