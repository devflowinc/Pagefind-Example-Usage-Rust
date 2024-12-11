use hashbrown::HashMap;
use pagefind::{Fossicker, SearchState};

pub fn new_index(config: pagefind::PagefindInboundConfig) -> Option<SearchState> {
    match pagefind::SearchOptions::load(config) {
        Ok(opts) => Some(SearchState::new(opts)),
        Err(_) => None,
    }
}

pub async fn add_record(
    index: &mut SearchState,
    url: String,
    content: String,
    language: String,
    meta: Option<HashMap<String, String>>,
    filters: Option<HashMap<String, Vec<String>>>,
    sort: Option<HashMap<String, String>>,
) -> Result<pagefind::FossickedData, ()> {
    let data = pagefind::fossick::parser::DomParserResult {
        digest: content,
        filters: filters.unwrap_or_default(),
        sort: sort.unwrap_or_default(),
        meta: meta.unwrap_or_default(),
        anchor_content: HashMap::new(),
        has_custom_body: false,
        force_inclusion: true,
        has_html_element: true,
        has_old_bundle_reference: false,
        language: index.options.force_language.clone().unwrap_or(language),
    };
    let file = Fossicker::new_with_data(url, data);
    index.fossick_one(file).await
}

pub async fn write_files(index: &mut SearchState, output_path: Option<String>) {
    index.build_indexes().await;

    index.write_files(output_path.map(Into::into)).await;
}

pub async fn get_files(index: &mut SearchState) -> Vec<pagefind::SyntheticFile> {
    index.build_indexes().await;
    index.get_files().await
}

#[tokio::main]
pub async fn main() {
    // read from file data.json
    let data = std::fs::read_to_string("data.json").unwrap();
    let json_value: serde_json::Value = serde_json::from_str(&data).unwrap();

    let mut search_index = new_index(pagefind::PagefindInboundConfig {
        source: "source".into(),
        site: "site".into(),
        bundle_dir: None,
        output_subdir: None,
        output_path: None,
        root_selector: "root_selector".into(),
        exclude_selectors: vec![],
        glob: "**/*.{html}".into(),
        force_language: None,
        serve: false,
        verbose: false,
        logfile: None,
        keep_index_url: false,
        service: false,
    })
    .unwrap();

    if let serde_json::Value::Array(items) = &json_value {
        for item in items {
            // If you want to treat each item as an object
            if let serde_json::Value::Object(obj) = item {
                // Iterate over the key-value pairs in the object
                let chunk_html = &obj["chunk_html"];
                let link = &obj["link"];

                // get metadata from the object
                let metadata = &obj["metadata"];

                match add_record(
                    &mut search_index,
                    link.to_string(),
                    chunk_html.to_string(),
                    "en".to_string(),
                    metadata.as_object().map(|m| {
                        m.iter()
                            .map(|(k, v)| (k.to_string(), v.to_string()))
                            .collect()
                    }),
                    None,
                    None,
                )
                .await
                {
                    Ok(_) => {
                        println!("Successfully added record");
                    }
                    Err(_) => {
                        println!("Failed to add record");
                    }
                }
            }
        }
    }

    let files = get_files(&mut search_index).await;
    for file in files.iter() {
        // Need to write the file to the output path
        println!("file {:?}", file.filename);
        println!("content {:?}", file.contents);
    }
}
