use crate::{miniflux::MinifluxContext, szurubooru::SzurubooruContext};

pub(crate) fn perform(miniflux_context: MinifluxContext, szurubooru_context: SzurubooruContext) {
    let posts = miniflux_context.download_starred_posts();

    let media_links: Vec<_> = posts.iter().flat_map(|post| post.media_links()).collect();
    println!("Media links are: {media_links:?}");

    let new_media_links: Vec<_> = media_links.into_iter().filter(|link| !szurubooru_context.post_is_already_uploaded(link)).collect();
    println!("New Media links are: {new_media_links:?}");

    for link in new_media_links {
        szurubooru_context.upload_post(link);
    }

    for id in posts.iter().map(|post| post.id) {
        miniflux_context.unfavorite_post(id);
    }
}