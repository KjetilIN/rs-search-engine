document.addEventListener('DOMContentLoaded', () => {
    const resultsDiv = document.getElementById('results');

    // Retrieve the search result from localStorage
    const searchResult = localStorage.getItem('searchResult');
    console.log(searchResult);

    if (searchResult) {
        // Assuming searchResult is a JSON string containing an array of URLs
        const data = JSON.parse(searchResult);
        console.log("Parsed: " + data)

        // Create a list element
        const ul = document.createElement('ul');

        // Create list items for each URL and append to the list
        data.results.forEach(item => {
            const li = document.createElement('li');
            const a = document.createElement('a');
            a.href = item.url;
            a.textContent = item.url;
            const title = document.createElement('h4');
            title.textContent = item.title;
            title.className = 'title';
            li.appendChild(title);
            li.appendChild(a);
            ul.appendChild(li);
        });

        // Append the list to the results div
        resultsDiv.appendChild(ul);

        // Clear the searchResult from localStorage if you don't need it anymore
        localStorage.removeItem('searchResult');
    } else {
        resultsDiv.textContent = 'No results found.';
    }
});
