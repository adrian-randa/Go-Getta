fetch("api/who_am_i").then((response) => {
    response.json().then((json) => {
        let currentUser = document.querySelector("#currentUser");

        currentUser.querySelector(".username").textContent = json.public_name;
        currentUser.querySelector(".profilePicture").setAttribute("style", `background-image: url('storage/profile_picture/${json.username}')`);

        window.localStorage.setItem("currentUser", JSON.stringify(json));
    })
})

const postScreen = document.querySelector("#postScreen");
const postCreationScreen = document.querySelector("#postCreation");

function showPostCreationScreen() {
    postScreen.style.display = "none";
    postCreationScreen.style.display = "flex";
}

function showPostScreen() {
    postScreen.style.display = "flex";
    postCreationScreen.style.display = "none";
}

function showRoomCreationScreen() {
    //TODO
}

function showPublicSpaceScreen() {
    showPostScreen();
    //TODO
}

function showFollowingScreen() {
    showPostScreen();
    //TODO
}