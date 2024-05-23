let searchButton = document.querySelector("#searchbutton");
let input = document.querySelector("#searchbar");

// Domain of the server 
const domain = "http://127.0.0.1:8080"; 

// Handle Search Requests 
function handleSearchRequests(e){
    // Prevent default
    e.preventDefault()
    let text = input.value;
    console.log("User Searched: '" + text + "'");

    const url = `${domain}/api/search`;

    // Do post request to server 
    fetch(url, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/text'
        },
        body: text
    })
    .then(response => response.json())
    .then(data => {
        console.log('Success:', data);
        // Store the result in localStorage
        localStorage.setItem('searchResult', JSON.stringify(data));

        // Redirect to search.html
        window.location.href = "/search.html";
    })
    .catch((error) => {
        console.error('Error:', error);
    }).finally(() =>{
        // Clear text input after search
        input.value = "";
    });

}

function handleKeyPress(e){
    if (e.key === 'Enter') {
        handleSearchRequests(e);
    }
}


// Set event listeners 
searchButton.addEventListener("click", handleSearchRequests);
input.addEventListener("keypress", handleKeyPress)