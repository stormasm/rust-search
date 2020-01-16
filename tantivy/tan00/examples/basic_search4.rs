use std::fs::create_dir;
use std::path::Path;
use tantivy::schema::{Document, Schema, STORED, TEXT};
use tantivy::Index;

fn main() -> tantivy::Result<()> {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();

    let index_path = Path::new("/tmp66/tantivy/idxbs");
    let dir_exists = index_path.exists();

    if !dir_exists {
        create_dir(index_path).expect("dir already exists");
    }

    let index = Index::open_in_dir(&index_path).unwrap(); //ok

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

    index_writer.commit()?;
    Ok(())
}
