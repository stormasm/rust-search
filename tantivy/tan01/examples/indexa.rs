use std::path::Path;
use tantivy::schema::*;
use tantivy::schema::{Schema, TEXT};
use tantivy::{doc, Index};

fn create_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT);
    schema_builder.add_u64_field("id", FAST);
    schema_builder.build()
}

fn main() -> tantivy::Result<()> {
    let schema = create_schema();

    let index_path = Path::new("/tmp/tantivy/idxbs");
    let index = Index::open_in_dir(&index_path).unwrap(); //ok

    let mut index_writer = index.writer(50_000_000)?;

    let title = schema.get_field("title").unwrap();
    let id: Field = schema.get_field("id").unwrap();
    index_writer.add_document(doc!(title => "Post about Ralph", id => 50u64));
    index_writer.add_document(doc!(title => "Bill is a good guy", id => 500u64));

    index_writer.commit()?;
    Ok(())
}
