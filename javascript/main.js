//this function takes what the user types into the input field and 
//displays it as an alert

function postAlert() {
    titleInput = document.getElementById("name");
    alert(titleInput.value);
    titleInput.value = null;
}

let createButton = document.getElementById("create-button");
createButton.addEventListener("click", postAlert);