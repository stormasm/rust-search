// This will work on any meta.json file located in the directory

use std::path::Path;
use tantivy::query::QueryParser;
use tantivy::schema::Field;
use tantivy::Index;

fn main() -> tantivy::Result<()> {
    let directory = Path::new("/tmp/tantivy/idxbs");
    let dir_exists = directory.exists();
    if dir_exists {
        println!("{}", "Found the tantivy index directory")
    }

    let index = Index::open_in_dir(&directory)?;
    let schema = index.schema();

    let default_fields: Vec<Field> = schema.fields().map(|(f, _)| f).collect::<Vec<_>>();

    let _query_parser =
        QueryParser::new(schema.clone(), default_fields, index.tokenizers().clone());

    for val in schema.fields() {
        println!("{:?}", val);
    }

    for (field, field_entry) in schema.fields() {
        println!("{} {}", field.field_id(), field_entry.name());
    }

    Ok(())
}
