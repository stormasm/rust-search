use std::path::Path;
use tantivy::schema::{Document, Schema, STORED, TEXT};
use tantivy::Index;

fn main() -> tantivy::Result<()> {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("body", TEXT);
    let schema = schema_builder.build();

    let index_path = Path::new("/tmp/tantivy/idxbs");
    let index = Index::open_in_dir(&index_path).unwrap(); //ok

    let mut index_writer = index.writer(50_000_000)?;
    let title = schema.get_field("title").unwrap();
    let body = schema.get_field("body").unwrap();

    let mut my_doc = Document::default();
    my_doc.add_text(title, "Santa Fe New Mexico");
    my_doc.add_text(body, "Sangre de Cristo Mountains");

    index_writer.add_document(my_doc);

    index_writer.commit()?;
    Ok(())
}
