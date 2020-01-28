
To get this up and running from an initial state...

```
alias cb='cargo build'
alias cre='cargo run --example'
alias ttclean='cd /tmp; rm -fr tantivy; mkdir tantivy'
```

```
ttclean

### Build the initial index from scratch
cre indexhn

### Search the index
cre searchpb
cre searchpb dairy

### Open the index and add in more documents
cre indexa

### search the index
cre searchpb ralph
cre searchpb bill

### to build a binary from the example
cb --example searchpb
```
