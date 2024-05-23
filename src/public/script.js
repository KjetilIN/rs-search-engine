let searchButton = document.querySelector("#searchbutton");
let input = document.querySelector("#searchbar");


// Handle Search Requests 
function handleSearchRequests(e){
    // Prevent default
    e.preventDefault()
    let text = input.value;
    console.log("User Searched: '" + text + "'");
}

function handleKeyPress(e){
    if (e.key === 'Enter') {
        handleSearchRequests(e);
    }
}


// Set event listeners 
searchButton.addEventListener("click", handleSearchRequests);
input.addEventListener("keypress", handleKeyPress)