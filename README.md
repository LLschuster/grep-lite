# grep lite

## Description

Simple application in rust that showcase how to create a simple cli, how to handle strings, doing file IO
using regex.

## Usage

first build the command
```bash
cargo build
```

Then run it giving it a file and a pattern
```bash
cargo run -- -p salsa -f testfile.txt
```

example response

```
3. Ich werde dich ins Auge Spucken
Found match here >>>> 4. La vida es una, vivela como salsa
5. yu yyuy uyuyuyuyu
```


