use reqwest;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::{doc, Index, IndexWriter, ReloadPolicy, TantivyError};
use tempfile::TempDir;
use tantivy::directory::MmapDirectory;
use tantivy::IndexSettings;

fn main() -> Result<(), TantivyError> {
    let result = reqwest::blocking::get("https://www.wsws.org/en/articles/2022/10/13/vkxz-o13.html")
        .expect("Failed to fetch");
    println!("{:?}", result);
    let text = result.text().expect("Failed to get contents");
    // println!("text: {}", text);

    // let index_path = TempDir::new()?;
    let index_path = "./index";
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();
    // let index = Index::create_in_dir(&index_path, schema.clone())?;
    let directory = MmapDirectory::open(&index_path)?;
    let index = Index::create(directory, schema.clone(), IndexSettings::default())?;
    let mut index_writer: IndexWriter = index.writer(50_000_000)?;

    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();

    let mut wsws_doc = TantivyDocument::default();
    wsws_doc.add_text(title, "Services rendered: Ben Bernanke awarded Nobel Prize for economics");
    wsws_doc.add_text(
        body,
        text
    );
    index_writer.add_document(wsws_doc)?;

    index_writer.commit()?;
    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommitWithDelay)
        .try_into()?;

    println!("Searching...");
    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![title, body]);
    let query = query_parser.parse_query("Barack Obama")?;
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    for (_score, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
        println!("{}", retrieved_doc.to_json(&schema));
    }

    Ok(())
}
