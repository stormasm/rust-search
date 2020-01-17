use tantivy::schema::Field;
use tantivy::schema::*;
use tantivy::{doc, Index};

fn create_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    schema_builder.add_text_field("title", TEXT);
    schema_builder.add_u64_field("id", FAST);
    schema_builder.build()
}

fn create_index() -> tantivy::Result<Index> {
    let schema = create_schema();

    let index = Index::create_in_ram(schema);

    let mut index_writer = index.writer_with_num_threads(1, 3_000_000)?;
    let title = index.schema().get_field("title").unwrap();
    let id: Field = index.schema().get_field("id").unwrap();
    index_writer.add_document(doc!(title => "The Diary of Muadib", id => 1u64));
    index_writer.add_document(doc!(title => "A Dairy Cow", id => 10u64));
    index_writer.commit()?;
    Ok(index)
}

fn main() -> tantivy::Result<()> {
    let _index = create_index().unwrap();
    Ok(())
}
