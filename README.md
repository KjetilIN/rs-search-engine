# Rust Search Engine
A search engine created with Rust. Parses .md and .html files. 


## Usage

For running the webserver: <br>
```terminal
cargo run serve
```

For parsing the HTML files to a file: <br>
```terminal
cargo run parse file
```

For loading and viewing the files for the engine: <br>
```terminal
cargo run load
```

## Search API

Desired output from search 

```json

{
    "results": [
        {
            "url": "https://example.com",
            "title": "Example Domain",
        },
        {
            "url": "https://anotherexample.com",
            "title": "Another Example",
        }
    ]
}


```

## Resources

Term Frequency–Inverse Document Frequency (tf-idf) <br>
https://en.wikipedia.org/wiki/Tf%E2%80%93idf <br>
https://www.geeksforgeeks.org/understanding-tf-idf-term-frequency-inverse-document-frequency/ <br>