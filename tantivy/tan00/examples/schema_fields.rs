use std::path::Path;
use tantivy::Index;

fn main() -> tantivy::Result<()> {
    let directory = Path::new("/tmp/tantivy/idxhn");
    let dir_exists = directory.exists();
    if dir_exists {
        println!("{}", "Found the tantivy index directory")
    }

    let index = Index::open_in_dir(&directory)?;
    let schema = index.schema();

    for (x, y) in schema.fields() {
        println!("{:?} {:?}", x, y);
    }

    Ok(())
}