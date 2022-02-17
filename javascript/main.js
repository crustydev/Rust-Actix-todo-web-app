//this function takes what the user types into the input field and 
//displays it as an alert


function postAlert() {

    function postAlert() {
        // Gets reference to input field
        let titleInput = document.getElementById("name");
        // creates an alert that displays the input field's value
        alert(titleInput.value);
        // set input field value back to null
        titleInput = null;
    }

    // get reference to create button
    let createButton = document.getElementById("create-button");
    // when button is clicked, call function postAlert
    createButton.addEventListener("click", postAlert);
}