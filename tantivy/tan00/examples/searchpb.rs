use std::env;
use std::path::Path;
use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::{Schema, STORED, TEXT};
use tantivy::{Index, ReloadPolicy};

fn main() -> tantivy::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut search_string = "sea whale";

    if args.len() == 2 {
        search_string = &args[1];
    }

    println!("search string = {}", search_string);

    let index_path = Path::new("/tmp/tantivy/idxbs");

    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();

    let index = Index::open_in_dir(&index_path).unwrap(); //ok

    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![title, body]);
    let query = query_parser.parse_query(search_string)?;
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        println!("{}", schema.to_json(&retrieved_doc));
    }

    Ok(())
}
