use chrono::Days;
use reqwest::Url;
use serde::Deserialize;

pub(crate) struct MinifluxContext {
    token: String
}

impl MinifluxContext {
    pub(crate) fn new(token: String) -> Self {
        Self { token }
    }

    pub(crate) fn download_starred_posts(&self) -> Vec<MinifluxPost> {
        use reqwest::blocking as reqwest;
        #[derive(Deserialize, Debug)]
        struct EntriesResponse {
            entries: Vec<MinifluxPost>
        }

        let earliest = chrono::offset::Utc::now().checked_sub_days(Days::new(7)).unwrap().timestamp();
        let client = reqwest::Client::new();
        let response = client.get(format!("https://miniflux.privacy.qvarford.net/v1/entries?after={earliest}&starred=true"))
            .header("X-Auth-Token", &self.token)
            .send()
            .unwrap();
        assert!(response.status().is_success());

        let text = response.text().unwrap();
        let deserialized: EntriesResponse = serde_json::from_str(&text).unwrap();
        
        println!("deserialized: {deserialized:?}");

        deserialized.entries
    }

    pub(crate) fn unfavorite_post(&self, id: i64) {
        todo!()
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct MinifluxPost {
    pub id: i64,
    pub content: String
}

impl MinifluxPost {
    pub(crate) fn media_links(&self) -> Vec<Url> {
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