# Rust Search Engine
A search engine created with Rust. Parses .md and .html files. 


## Usage

```terminal
cargo run
```

## Search API

Desired output from search 

```json

{
    "results": [
        {
            "url": "https://example.com",
            "title": "Example Domain",
            "description": "This domain is for use in illustrative examples in documents."
        },
        {
            "url": "https://anotherexample.com",
            "title": "Another Example",
            "description": "This is another example of a search result description."
        }
    ]
}


```

## Resources

Term Frequency–Inverse Document Frequency (tf-idf) <br>
https://en.wikipedia.org/wiki/Tf%E2%80%93idf <br>
https://www.geeksforgeeks.org/understanding-tf-idf-term-frequency-inverse-document-frequency/ <br>