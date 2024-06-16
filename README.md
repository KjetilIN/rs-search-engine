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
- **Multithreaded Indexer:** Parses documents concurrently using `rayon` for improved performance.
- **Rust Backend:** Utilizes Rust and the tiny_http library to serve search requests.
- **Frontend Interface:** Provides a frontend created by vanilla HTML/CSS.

## Usage

Running the webserver: <br>
```terminal
cargo run serve
```
For indexing the HTML files to a file: <br>
```terminal
cargo run index file
```

For loading and viewing the files for the engine: <br>
```terminal
cargo run load
```

**NOTE** Set `domain` variable in `./frontend/script.js` to `0.0.0.0:8080`

### Docker

Running the application with docker is simple: 
```terminal
docker compose up --build 
```
(add `-d` option for running detached)


Read more about [Docker Compose here.](https://docs.docker.com/compose/reference/)

**NOTE** Set `domain` variable in `./frontend/script.js` to `localhost:8080`

## Setup 

There is two options for setting up the project.

1. Use my files as documents (easiest)
2. Setup your own search engine files 


### 1. Use my files as document

1. Unzip the pages directory locally:
```terminal 
tar -xvf ./cache/pages.tar.gz .
```
2. Re-index the documents
```terminal 
cargo run parse file 
```
3. Start the HTTP server locally 
```terminal 
cargo run serve 
```

## 2. Setup your own search engine files 

1. Create a list of urls that you want to index. Each url must lead to a html file form the [www.gutenberg.org](www.gutenberg.org) website. Store them with the url and title separated with a semicolon in `./cache/urls.txt`. For example: 
```text
https://www.gutenberg.org/cache/epub/57532/pg57532-images.html ; Passages from the Life of a Philosopher
https://www.gutenberg.org/cache/epub/69512/pg69512-images.html ; The calculus of logic
https://www.gutenberg.org/cache/epub/55280/pg55280-images.html ; An Enquiry into the Life and Legend of Michael Scot
....
```
2. Create a pages directory
```terminal
mkdir -p ./pages/
```
3. Download each html file and set the name of each file equal to `file<INDEX>.html`, where INDEX is equal to the line number of the file in `./cache/urls.txt`. Store each file in the `./pages/` directory. 
4. Re-index the documents
```terminal 
cargo run parse file 
```
5. Start the HTTP server locally 
```terminal 
cargo run serve 
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
