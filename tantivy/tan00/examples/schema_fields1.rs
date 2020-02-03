// This will work on any meta.json file located in the directory

use std::path::Path;
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

    //let x1 = schema

    let default_fields = schema
        .fields()
        .enumerate()
        .map(|(_, fe)| fe)
        .collect::<Vec<_>>();

    println!("{:?}", default_fields);

    println!("{}", "----------------------");

    for val in schema.fields() {
        println!("{:?}", val);
    }

    // let myfields: Vec<> = schema.fields().collect();

    for (field, field_entry) in schema.fields() {
        println!("{} {}", field.field_id(), field_entry.name());
    }

    Ok(())
}
