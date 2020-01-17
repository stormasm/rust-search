use tantivy::SegmentReader;
use tantivy::collector::TopDocs;
use tantivy::schema::Field;

fn create_schema() -> Schema {
   let mut schema_builder = Schema::builder();
   schema_builder.add_text_field("product_name", TEXT);
   schema_builder.add_u64_field("popularity", FAST);
   schema_builder.build()
}

fn create_index() -> tantivy::Result<Index> {
  let schema = create_schema();
  let index = Index::create_in_ram(schema);
  let mut index_writer = index.writer_with_num_threads(1, 3_000_000)?;
  let product_name = index.schema().get_field("product_name").unwrap();
  let popularity: Field = index.schema().get_field("popularity").unwrap();
  index_writer.add_document(doc!(product_name => "The Diary of Muadib", popularity => 1u64));
  index_writer.add_document(doc!(product_name => "A Dairy Cow", popularity => 10u64));
  index_writer.add_document(doc!(product_name => "The Diary of a Young Girl", popularity => 15u64));
  index_writer.commit()?;
  Ok(index)
}

fn main() -> tantivy::Result<()> {
let index = create_index().unwrap();
let product_name = index.schema().get_field("product_name").unwrap();
let popularity: Field = index.schema().get_field("popularity").unwrap();

let user_query_str = "diary";
let query_parser = QueryParser::for_index(&index, vec![product_name]);
let query = query_parser.parse_query(user_query_str).unwrap();

// This is where we build our collector with our custom score.
let top_docs_by_custom_score = TopDocs
        ::with_limit(10)
         .tweak_score(move |segment_reader: &SegmentReader| {
            // The argument is a function that returns our scoring
            // function.
            //
            // The point of this "mother" function is to gather all
            // of the segment level information we need for scoring.
            // Typically, fast_fields.
            //
            // In our case, we will get a reader for the popularity
            // fast field.
            let popularity_reader =
                segment_reader.fast_fields().u64(popularity).unwrap();

            // We can now define our actual scoring function
            move |doc: DocId, original_score: Score| {
                let popularity: u64 = popularity_reader.get(doc);
                // Well.. For the sake of the example we use a simple logarithm
                // function.
                let popularity_boost_score = ((2u64 + popularity) as f32).log2();
                popularity_boost_score * original_score
            }
          });
let reader = index.reader().unwrap();
let searcher = reader.searcher();
// ... and here are our documents. Note this is a simple vec.
// The `Score` in the pair is our tweaked score.
let resulting_docs: Vec<(Score, DocAddress)> =
     searcher.search(&query, &top_docs_by_custom_score).unwrap();
Ok(())
}
