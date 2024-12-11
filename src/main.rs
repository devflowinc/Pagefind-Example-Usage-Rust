use pagefind::service::api::PagefindIndex;

#[tokio::main]
pub async fn main() {
    // read from file data.json
    let mut index = PagefindIndex::new(pagefind::PagefindInboundConfig {
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
    .expect("config is valid");

    let data = std::fs::read_to_string("data.json").unwrap();
    let json_value: serde_json::Value = serde_json::from_str(&data).unwrap();

    if let serde_json::Value::Array(items) = &json_value {
        for item in items {
            // If you want to treat each item as an object
            if let serde_json::Value::Object(obj) = item {
                // Iterate over the key-value pairs in the object
                let chunk_html = &obj["chunk_html"];
                let link = &obj["link"];

                // get metadata from the object
                let metadata = &obj["metadata"];

                match index
                    .add_record(
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

    let files = index.get_files().await;
    for file in files.iter() {
        // Need to write the file to the output path
        println!("file {:?}", file.path);
        println!("content {:?}", file.content);
    }

    index.write_files(Some("static/pagefind".to_string())).await;
}
