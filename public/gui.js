fetch("api/who_am_i").then((response) => {
    response.json().then((json) => {
        let currentUser = document.querySelector("#currentUser");


        currentUser.querySelector(".username").textContent = json.public_name;
        currentUser.querySelector(".profilePicture").setAttribute("style", `background-image: url('storage/profile_picture/${json.username}')`);

        window.localStorage.setItem("currentUsername", json.username);
        window.localStorage.setItem("currentPublicName", json.public_name);
    })
})

async function logout() {
    let response = await fetch("/logout", {method: "DELETE"});

    if (!response.ok) {
        alert(await response.text());
    } else {
        document.cookie = "session_id= ;";
        window.location.reload();
    }
}

const mainContent = document.querySelector("#mainContent");
const postScreen = document.querySelector("#postScreen");
const postCreationScreen = document.querySelector("#postCreation");
const postThreadScreen = document.querySelector("#postThread");
const postThreadParentsSection = postThreadScreen.querySelector(".parentPosts");
const postThreadCommentSection = postThreadScreen.querySelector(".childPosts");
const personalProfileScreen = document.querySelector("#personalProfile");

const noPaginator = () => {};
let currentPaginator = noPaginator;
document.addEventListener("scrolledToBottom", () => {currentPaginator()});

function showPostCreationScreen() {
    postScreen.style.display = "none";
    postCreationScreen.style.display = "flex";
    postThreadScreen.style.display = "none";
    personalProfileScreen.style.display = "none";

    currentPaginator = noPaginator;
}

function showPostScreen() {
    postScreen.innerHTML = "";
    postScreen.style.display = "flex";
    postCreationScreen.style.display = "none";
    postThreadScreen.style.display = "none";
    personalProfileScreen.style.display = "none";
}

function showRoomCreationScreen() {
    //TODO

    currentPaginator = noPaginator;
}

function showPublicSpaceScreen() {
    window.history.pushState({}, "", window.location.origin);

    showPostScreen();
    currentPaginator = initPublicSpacePaginator(mountPosts(postScreen));
    currentPaginator();
}

function showFollowingScreen() {
    showPostScreen();
    //TODO
}

function showPersonalProfileScreen() {
    postScreen.style.display = "none";
    postCreationScreen.style.display = "none";
    postThreadScreen.style.display = "none";
    personalProfileScreen.style.display = "flex";

    const currentUsername = window.localStorage.getItem("currentUsername");
    const postsContainer = personalProfileScreen.querySelector(".posts");

    currentPaginator = initUsersPostsPaginator(currentUsername, mountPosts(postsContainer));
    currentPaginator();
}

async function showPostThreadScreen(postID) {
    postThreadParentsSection.innerHTML = "";
    postThreadCommentSection.innerHTML = "";

    let parentThreadResponse = await fetch(`/api/get_thread/${postID}`);
    let parentThread = await parentThreadResponse.json();

    for (let i = 0; i < parentThread.length; i++) {
        postThreadParentsSection.appendChild(await makePostNode(parentThread[i]));
    }

    currentPaginator = initCommentPaginator(postID, mountPosts(postThreadCommentSection));
    currentPaginator();
    

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

const params = new URL(window.location.href).searchParams;
switch (params.get("view")) {
    case "post": {
        showPostThreadScreen(params.get("id"));
        break;
    }

    case "me": {
        showPersonalProfileScreen();
        break;
    }

    default: {
        showPublicSpaceScreen();
    }
}