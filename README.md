<div align="center">
    <h1>Rust Search Engine</h1>
    <i>Created by Kjetil Indrehus</i>
</div>

<div align="center">
    <br />
    <img alt="version" src="https://img.shields.io/badge/version-0.1.0-blue" />
    <img alt="Rust" src="https://img.shields.io/badge/rust-1.74-orange?logo=rust" />

</div>
<br />

This project is a custom search engine built with Rust, designed specifically for the Kubernetes website. It parses HTML files from the Kubernetes website and leverages TF-IDF (Term Frequency-Inverse Document Frequency) for scoring the relevance of documents, providing accurate and efficient search results. The search engine is composed of two main components: the backend server and the frontend interface. The backend server, implemented in Rust, processes HTML files, tokenizes the content, and calculates TF-IDF scores to determine the relevance of documents based on search terms. The frontend interface allows users to input search queries and view the ranked search results.



[Screencast from 24. mai 2024 kl. 17.35 +0200.webm](https://github.com/KjetilIN/rs-search-engine/assets/66110094/9d9f5832-7031-456d-80d4-d9c90bc6d4e1)



## Key Features

- **HTML Parsing:** Efficiently parses HTML files from the Kubernetes website.
- **Tokenization:** Breaks down the content into individual tokens (words) for analysis.
- **TF-IDF Scoring:** Uses TF-IDF to score and rank documents based on their relevance to the search query.
- **Rust Backend:** Utilizes Rust and the tiny_http library to serve search requests.
- **Frontend Interface:** Provides a frontend created by vanilla HTML/CSS.


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

Searching is executed by sending a POST request to the backend, with the search query as plain text

### Endpoint
`POST /api/search`

### Response
The following is a sample response from the output:

```json

{
    "results": [
        {
            "url": "https://example.com",
            "title": "Example Domain",
            "tf_idf_score": 0.00234 
        },
        {
            "url": "https://anotherexample.com",
            "title": "Another Example",
            "tf_idf_score": 0.00234 
        }
    ]
}
```

## Resources

Term Frequency–Inverse Document Frequency (tf-idf) <br>
https://en.wikipedia.org/wiki/Tf%E2%80%93idf <br>
https://www.geeksforgeeks.org/understanding-tf-idf-term-frequency-inverse-document-frequency/ <br>

Kubernetes Website Repository: <br>
https://github.com/kubernetes/website
