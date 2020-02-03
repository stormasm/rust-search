// This will work on any meta.json file located in the directory

use std::path::Path;
use tantivy::Index;
use tantivy::schema::Field;

fn main() -> tantivy::Result<()> {
    let directory = Path::new("/tmp/tantivy/idxbs");
    let dir_exists = directory.exists();
    if dir_exists {
        println!("{}", "Found the tantivy index directory")
    }

    let index = Index::open_in_dir(&directory)?;
    let schema = index.schema();

    let default_fields: Vec<Field> = schema.fields().collect::<Vec<_>>();

    // let myfields: Vec<> = schema.fields().collect();

    for (field, field_entry) in schema.fields() {
        println!("{} {}", field.field_id(), field_entry.name());
    }

    Ok(())
}
