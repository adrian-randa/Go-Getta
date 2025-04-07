fetch("api/who_am_i").then((response) => {
    response.json().then((json) => {
        let currentUser = document.querySelector("#currentUser");

        currentUser.querySelector(".username").textContent = json.public_name;
        currentUser.querySelector(".profilePicture").setAttribute("style", `background-image: url('storage/profile_picture/${json.username}')`);

        window.localStorage.setItem("currentUser", JSON.stringify(json));
    })
})

let params = new URLSearchParams(new URL(window.location.href));
switch (params.get("view")) {
    case "view": {

    }


    default: {
        showPublicSpaceScreen();
    }
}

const mainContent = document.querySelector("#mainContent");
const postScreen = document.querySelector("#postScreen");
const postCreationScreen = document.querySelector("#postCreation");
const postThreadScreen = document.querySelector("#postThread");

const noPaginator = () => {};
let currentPaginator = noPaginator;
document.addEventListener("scrolledToBottom", () => {currentPaginator()});

function showPostCreationScreen() {
    postScreen.style.display = "none";
    postCreationScreen.style.display = "flex";
    postThreadScreen.style.display = "none";

    currentPaginator = noPaginator;
}

function showPostScreen() {
    postScreen.innerHTML = "";
    postScreen.style.display = "flex";
    postCreationScreen.style.display = "none";
    postThreadScreen.style.display = "none";
}

function showRoomCreationScreen() {
    //TODO

    currentPaginator = noPaginator;
}

function showPublicSpaceScreen() {
    showPostScreen();
    currentPaginator = initPublicSpacePaginator(debugPassthrough(mountPosts(postScreen)));
    currentPaginator();
}

function showFollowingScreen() {
    showPostScreen();
    //TODO
}

function showPostThreadScreen() {
    currentPaginator = noPaginator; //TODO: Change this to the child paginator

    postScreen.style.display = "none";
    postCreationScreen.style.display = "none";
    postThreadScreen.style.display = "flex";
}

// Scroll to bottom event
document.addEventListener("scrollend", (event) => {
    const scrollPos = document.documentElement.scrollTop;
    const maxScroll = mainContent.offsetHeight - document.documentElement.offsetHeight;

    const tolerance = 50;

    if (maxScroll - scrollPos <= tolerance) {
        document.dispatchEvent(new CustomEvent("scrolledToBottom"));
    }
})

const debugPassthrough = (handler) => {
    return function(data) {
        console.log(data);
        handler(data);
    }
}