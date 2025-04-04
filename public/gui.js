fetch("api/who_am_i").then((response) => {
    response.json().then((json) => {
        let currentUser = document.querySelector("#currentUser");

        currentUser.querySelector(".username").textContent = json.public_name;
        currentUser.querySelector(".profilePicture").setAttribute("style", `background-image: url('storage/profile_picture/${json.username}')`);

        window.localStorage.setItem("currentUser", JSON.stringify(json));
    })
})

const mainContent = document.querySelector("#mainContent");
const postScreen = document.querySelector("#postScreen");
const postCreationScreen = document.querySelector("#postCreation");

const noPaginator = () => {};
let currentPaginator = noPaginator;
document.addEventListener("scrolledToBottom", () => {currentPaginator()});

function showPostCreationScreen() {
    postScreen.style.display = "none";
    postCreationScreen.style.display = "flex";

    currentPaginator = noPaginator;
}

function showPostScreen() {
    postScreen.innerHTML = "";
    postScreen.style.display = "flex";
    postCreationScreen.style.display = "none";
}

function showRoomCreationScreen() {
    //TODO

    currentPaginator = noPaginator;
}

function showPublicSpaceScreen() {
    showPostScreen();
    currentPaginator = initPublicSpacePaginator(mountPosts(postScreen));
    currentPaginator();
}

function showFollowingScreen() {
    showPostScreen();
    //TODO
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

const postTemplate = document.querySelector("#postTemplate");
console.log(postTemplate)
const mountPosts = (screen) => {
    return async (posts) => {
        console.log("Mounting posts", posts);
        for (let i = 0; i < posts.length; i++) {
            screen.appendChild(await makePostNode(posts[i]));
        }
    }
}

const mediaTypeLookup = {
    "Image": "img",
    "Video": "video",
    "Audio": "audio",
}

async function makePostNode(post) {
    let node = postTemplate.content.cloneNode(true);

    let userDataResponse = await fetch(`/api/get_user_data/${post.creator}`);
    let userData = await userDataResponse.json();

    let creatorDisplay = node.querySelector(".userDisplay");
    creatorDisplay.querySelector("h4").textContent = userData.public_name;
    creatorDisplay.querySelector("h5").textContent = post.creator;
    creatorDisplay.querySelector(".profilePicture").style.backgroundImage = `url("/storage/profile_picture/${post.creator}")`

    let timestamp = new Date(post.timestamp * 1000);
    let [date, fullTime] = timestamp.toISOString().split("T");
    let [hour, minute] = fullTime.split(":");

    let timestampDisplay = node.querySelector(".timestamp");
    let [dateDisplay, timeDisplay] = timestampDisplay.querySelectorAll("h5");
    dateDisplay.textContent = date;
    timeDisplay.textContent = `${hour}:${minute}`;

    if (post.appendage_id) {
        let appendageResponse = await fetch(`/storage/appendage/${post.appendage_id}`);
        if (appendageResponse.ok) {
            let appendage = await appendageResponse.json();
    
            const mediaContainer = node.querySelector(".appendages");
            
            appendage.files.forEach((file) => {
                let mediaType = mediaTypeLookup[file.file_type];
                let mediaNode = document.createElement(mediaType);
                mediaNode.setAttribute("src", `/storage/appendage/file/${file.file_id}`);
                if (mediaType == "video") mediaNode.setAttribute("controls", "");
                mediaContainer.appendChild(mediaNode);
            })
        }
    }

    node.querySelector(".content").textContent = post.body;

    let ratingDisplay = node.querySelector(".rating");
    ratingDisplay.querySelector("h5").textContent = post.rating;

    let commentButton = node.querySelector(".comment");
    commentButton.querySelector("h5").textContent = post.comments;

    let shareButton = node.querySelector(".share");
    shareButton.querySelector("h5").textContent = post.shares;

    let repostButton = node.querySelector(".repost");
    repostButton.querySelector("h5").textContent = post.reposts;

    let bookmarkButton = node.querySelector(".bookmark");
    bookmarkButton.querySelector("h5").textContent = post.bookmarks;

    return node;
}