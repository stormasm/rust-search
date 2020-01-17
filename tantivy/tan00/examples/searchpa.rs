use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
// use tantivy::schema::*;
use std::fs::{create_dir, remove_dir_all};
use std::path::Path;
use tantivy::schema::{Document, Schema, STORED, TEXT};
use tantivy::{doc, Index, ReloadPolicy};

fn main() -> tantivy::Result<()> {
    let check_path = Path::new("/tmp/tantivy/idxbs");
    let dir_exists = check_path.exists();
    if dir_exists {
        remove_dir_all(check_path).expect("dir does not exist");
    }

    let index_path = Path::new("/tmp/tantivy/idxbs");
    create_dir(index_path).expect("dir already exists");

    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();

    let index = Index::create_in_dir(&index_path, schema.clone())?;
    let mut index_writer = index.writer(50_000_000)?;

    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();

    let mut old_man_doc = Document::default();
    old_man_doc.add_text(title, "The Old Man and the Sea");
    old_man_doc.add_text(
        body,
        "He was an old man who fished alone in a skiff in the Gulf Stream and \
         he had gone eighty-four days now without taking a fish.",
    );

    index_writer.add_document(old_man_doc);

    // For convenience, tantivy also comes with a macro to
    // reduce the boilerplate above.
    index_writer.add_document(doc!(
    title => "Of Mice and Men",
    body => "A few miles south of Soledad, the Salinas River "
    ));

    index_writer.add_document(doc!(
    title => "Ralph Emerson",
    body => "A few miles south of Soledad, the Salinas River"
    ));

    // Multivalued field just need to be repeated.
    index_writer.add_document(doc!(
    title => "Frankenstein",
    body => "You will rejoice to hear that no disaster has accompanied"
    ));

    index_writer.commit()?;

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()?;

    let searcher = reader.searcher();
    let query_parser = QueryParser::for_index(&index, vec![title, body]);
    let query = query_parser.parse_query("sea whale")?;
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    for (_score, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        println!("{}", schema.to_json(&retrieved_doc));
    }

    Ok(())
}
