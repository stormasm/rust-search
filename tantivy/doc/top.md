
#### number of threads = number of segments

There is a one to one correspondence between the number of threads
and the number of segments.

The exact relationship is defined in
core/index.rs
pub fn writer

Core coding lingua franca

* schema_builder.add_text_field("title", TEXT | STORED);
* index_writer.add_document
* index.reader()?;
* reader.searcher();
* let query = query_parser.parse_query("good food")?;
* let top_docs = searcher.search(&query


#### Term

When we talk about index or search, we always mean terms. In Lucene, the only way to turn things to be searchable is break them into terms. Put it simply, the core data structure of Lucene is just a big hash map, the keys are terms, values are list of document ids. Querying becomes an O(1) operation.

##### References

[term](http://makble.com/what-is-lucene-term)
