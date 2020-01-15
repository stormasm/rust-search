
This file is all about the **schema** directory

**schema_builder** is mission critical to the whole tantivy ecosystem
and to better understand it you can search across either the src
directory or the examples directory.

##### This example is located in schema.rs

```
use tantivy::schema::*;

let mut schema_builder = Schema::builder();
let id_field = schema_builder.add_text_field("id", STRING);
let title_field = schema_builder.add_text_field("title", TEXT);
let body_field = schema_builder.add_text_field("body", TEXT);
let schema = schema_builder.build();
```

[On integer types in Rust](https://medium.com/@marcinbaraniecki/on-integer-types-in-rust-b3dc1b0a23d3) shows why for Hackernews ID's we want to use

```
pub fn add_u64_field
```

Unsigned integer types are those, that cannot hold negative values. Simply put — they cannot go below zero (think of unsigned as those, that can’t have a minus sign in front), and zero is exactly what is returned by their min_value() method..

In the examples see **integer_range_search** to see how a **u64_field** gets
indexed.  This is relevant when indexing the hackernews ID in tantivy.

### Parsing Json Documents

If you have a json document that you are trying to parse...
For example from Hackernews there is a **by** field in the json.
If you don't have the **by** field in the schema...

You will get a
[DocParsingError](https://docs.rs/tantivy/0.11.1/tantivy/schema/enum.DocParsingError.html)
**NoSuchFieldInSchema**
from the method
[parse_document](https://docs.rs/tantivy/0.11.1/tantivy/schema/struct.Schema.html#method.parse_document)

Bottom line here is for now the JSON that you are parsing has to align
up with the Schema that you define...

If you want to have a shorter schema with just the {id, score, title}
then the corresponding JSON documents you parse can only have those
fields as well...

For this reason: we need to have a program that reads the schema file
first and based on this schema only pull the fields from the json that
correspond with those fields in the schema prior to calling **parse_document** on
the JSON that got read...

Solution:
* Read a channel of json lines
* Build a new JSON based on the schema fields
* Write this reduced JSON to a new channel
