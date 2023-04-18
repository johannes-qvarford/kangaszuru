use std::path::Path;

pub(crate) fn perform(context: crate::szurubooru::SzurubooruContext, source_directory: String, _poll_name: Option<String>, tags: Vec<String>) {

    let mut tags: Vec<_> = tags.into_iter().flat_map(|tag| context.resolve_tag(tag)).collect();
    tags.sort();
    tags.dedup();

    println!("final tags: {tags:?}");

    let dir = std::fs::read_dir(Path::new(&source_directory)).unwrap();

    let mut entries = dir.into_iter().map(|r| r.unwrap()).collect::<Vec<_>>();
    entries.sort_by(|a, b| alphanumeric_sort::compare_path(a.path(), b.path()));
    println!("entries: {entries:?}");

    for entry in entries {
        if context.post_file_is_already_uploaded(&entry.path()) {
            println!("duplicate: {entry:?}");
        } else {
            context.upload_post_file(&entry.path(), tags.clone());
        }
    }
}