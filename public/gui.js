let whoAmI = new Promise((resolve, reject) => {
    fetch("api/who_am_i").then((response) => {
        response.json().then((json) => {
            let currentUser = document.querySelector("#currentUser");
    
    
            currentUser.querySelector(".username").textContent = json.public_name;
            currentUser.querySelector(".profilePicture").setAttribute("style", `background-image: url('storage/profile_picture/${json.username}')`);
    
            window.localStorage.setItem("currentUsername", json.username);
            window.localStorage.setItem("currentPublicName", json.public_name);

            resolve(json);
        });
    });
});

const roomButtonTamplate = document.querySelector("#roomButtonTemplate");
const roomButtonContainer = document.querySelector("#myRooms");
const newPostRoomSelector = document.querySelector("#newPostRoom");
let joinedRooms = new Promise((resolve, reject) => {
    fetch("api/get_joined_rooms").then((response) => {
        if (!response.ok) {
            response.text().then(alert);
            return;
        }
    
        response.json().then((rooms) => {
            rooms = Array.from(rooms);
            rooms.forEach((room) => {
                let node = roomButtonTamplate.content.cloneNode(true);

                node.children[0].setAttribute("style", `--room-color: #${room.color}`);
                node.children[0].setAttribute("href", `?view=room&id=${room.id}`);

                node.querySelector("span").textContent = room.name;

                roomButtonContainer.appendChild(node);

                let option = document.createElement("option")
                option.setAttribute("value", room.id);
                option.innerText = room.name;
                newPostRoomSelector.appendChild(option);
            });
            resolve(rooms);
        });
    });
});


async function logout() {
    let response = await baseErrorHandler.guard(fetch("/logout", {method: "DELETE"}));

    document.cookie = "session_id= ;";
    window.location.reload();
}

const mainContent = document.querySelector("#mainContent");
const postScreen = document.querySelector("#postScreen");
const postCreationScreen = document.querySelector("#postCreation");
const repostCreationScreen = document.querySelector("#repostCreation");
const postThreadScreen = document.querySelector("#postThread");
const postThreadParentsSection = postThreadScreen.querySelector(".parentPostsContainer");
const postThreadFocusedSlot = postThreadScreen.querySelector(".focusedPost");
const postThreadCommentSection = postThreadScreen.querySelector(".childPosts");
const personalProfileScreen = document.querySelector("#personalProfile");
const profileScreen = document.querySelector("#profile");
const roomCreationScreen = document.querySelector("#roomCreation");
const searchScreen = document.querySelector("#search");

const showPublicSpaceButton = document.querySelector("#showPublicSpaceButton");
const showFollowingFeedButton = document.querySelector("#showFollowingFeedButton");

const noPaginator = () => {};
let currentPaginator = noPaginator;
document.addEventListener("scrolledToBottom", () => {currentPaginator()});


const newPostRemainingCharactersDisplay = document.querySelector("#postCreation")
        .querySelector(".bodyInput")
        .querySelector("h5");

const updateNewPostRemainingCharactersDisplay = (event) => {
    let remaining = POST_CHARACTER_LIMIT - (event || {target: {value: {length: 0}}}).target.value.length;

    newPostRemainingCharactersDisplay.textContent = remaining;

    newPostRemainingCharactersDisplay.style.color = remaining < 40 ? "var(--red)" : "var(--gray)";
}
document.querySelector("#newPostBody").addEventListener("input", updateNewPostRemainingCharactersDisplay);

function showPostCreationScreen() {
    postScreen.style.display = "none";
    postCreationScreen.style.display = "flex";
    repostCreationScreen.style.display = "none";
    postThreadScreen.style.display = "none";
    personalProfileScreen.style.display = "none";
    profileScreen.style.display = "none";
    roomCreationScreen.style.display = "none";
    searchScreen.style.display = "none";

    updateNewPostRemainingCharactersDisplay();

    currentPaginator = noPaginator;
}

const referencedPostContainer = repostCreationScreen.querySelector(".referencedPost");
async function showRepostCreationScreen(childPostID) {
    let response = await baseErrorHandler.guard(fetch(`/api/get_post/${childPostID}`));
    
    let referencedPostData = await response.json();
    
    referencedPostContainer.innerHTML = "";
    referencedPostContainer.appendChild(await makePostNode(referencedPostData));

    document.querySelector("#newRepostSubmitButton").onclick = () => {submitRepost(childPostID)};

    const repostRoomSelector = document.querySelector("#newRepostRoom");
    let rooms = await joinedRooms;
    rooms.forEach((r) => {
        let option = document.createElement("option");
        option.setAttribute("value", r.id);
        option.innerText = r.name;
        repostRoomSelector.appendChild(option);
    });

    if (referencedPostData.post.room) {
        let room = rooms.find((r) => r.id == referencedPostData.post.room);
        if (!room) return;

        if (room.is_private) {
            repostRoomSelector.innerHTML = "";
            let option = document.createElement("option");
            option.setAttribute("value", room.id);
            option.innerText = room.name;
            option.setAttribute("selected", "");
            repostRoomSelector.appendChild(option);
            repostRoomSelector.value = referencedPostData.post.room;
        }
    }


    postScreen.style.display = "none";
    postCreationScreen.style.display = "none";
    repostCreationScreen.style.display = "flex";
    postThreadScreen.style.display = "none";
    personalProfileScreen.style.display = "none";
    profileScreen.style.display = "none";
    roomCreationScreen.style.display = "none";
    searchScreen.style.display = "none";

    currentPaginator = noPaginator;
}

const newRepostRemainingCharactersDisplay = document.querySelector("#repostCreation")
        .querySelector(".bodyInput")
        .querySelector("h5");

const updateNewRepostRemainingCharactersDisplay = (event) => {
    let remaining = POST_CHARACTER_LIMIT - (event || {target: {value: {length: 0}}}).target.value.length;

    newRepostRemainingCharactersDisplay.textContent = remaining;

    newRepostRemainingCharactersDisplay.style.color = remaining < 40 ? "var(--red)" : "var(--gray)";
}
document.querySelector("#newRepostBody").addEventListener("input", updateNewRepostRemainingCharactersDisplay);

function showPostScreen() {
    postScreen.innerHTML = "";

    postScreen.style.display = "flex";
    postCreationScreen.style.display = "none";
    repostCreationScreen.style.display = "none";
    postThreadScreen.style.display = "none";
    personalProfileScreen.style.display = "none";
    profileScreen.style.display = "none";
    roomCreationScreen.style.display = "none";
    searchScreen.style.display = "none";
}

function showRoomCreationScreen() {
    postScreen.style.display = "none";
    postCreationScreen.style.display = "none";
    repostCreationScreen.style.display = "none";
    postThreadScreen.style.display = "none";
    personalProfileScreen.style.display = "none";
    profileScreen.style.display = "none";
    roomCreationScreen.style.display = "flex";
    searchScreen.style.display = "none";

    currentPaginator = noPaginator;
}

function showPublicSpaceScreen() {
    window.history.pushState({}, "", window.location.origin);

    showPublicSpaceButton.setAttribute("selected", "");
    showFollowingFeedButton.removeAttribute("selected")


    showPostScreen();
    currentPaginator = initPublicSpacePaginator(mountPosts(postScreen));
    currentPaginator();
}

function showFollowingScreen() {

    showFollowingFeedButton.setAttribute("selected", "");
    showPublicSpaceButton.removeAttribute("selected");

    showPostScreen();
    currentPaginator = initFollowedFeedPaginator(mountPosts(postScreen));
    currentPaginator();
}

function showBookmarkedScreen() {
    showPostScreen();

    currentPaginator = initBookmarkedPaginator(mountPosts(postScreen));
    currentPaginator();
}

function showPersonalProfileScreen() {
    postScreen.style.display = "none";
    postCreationScreen.style.display = "none";
    repostCreationScreen.style.display = "none";
    postThreadScreen.style.display = "none";
    personalProfileScreen.style.display = "flex";
    profileScreen.style.display = "none";
    roomCreationScreen.style.display = "none";
    searchScreen.style.display = "none";

    const currentUsername = window.localStorage.getItem("currentUsername");
    const postsContainer = personalProfileScreen.querySelector(".posts");

    currentPaginator = initUsersPostsPaginator(currentUsername, mountPosts(postsContainer));
    currentPaginator();
}

function showProfileScreen(username) {
    postScreen.style.display = "none";
    postCreationScreen.style.display = "none";
    repostCreationScreen.style.display = "none";
    postThreadScreen.style.display = "none";
    personalProfileScreen.style.display = "none";
    profileScreen.style.display = "flex";
    roomCreationScreen.style.display = "none";
    searchScreen.style.display = "none";

    const postsContainer = profileScreen.querySelector(".posts");

    currentPaginator = initUsersPostsPaginator(username, mountPosts(postsContainer));
    currentPaginator();
}

async function showPostThreadScreen(postID) {
    postThreadParentsSection.innerHTML = "";
    postThreadCommentSection.innerHTML = "";
    postThreadFocusedSlot.innerHTML = "";

    let parentThreadResponse = await baseErrorHandler.guard(fetch(`/api/get_thread/${postID}`));
    let parentThread = await parentThreadResponse.json();

    for (let i = 0; i < parentThread.length - 1; i++) {
        postThreadParentsSection.appendChild(await makePostNode(parentThread[i]));
    }

    postThreadFocusedSlot.appendChild(await makePostNode(parentThread[parentThread.length - 1]));

    currentPaginator = initCommentPaginator(postID, mountPosts(postThreadCommentSection));
    currentPaginator();
    

    postScreen.style.display = "none";
    postCreationScreen.style.display = "none";
    repostCreationScreen.style.display = "none";
    postThreadScreen.style.display = "flex";
    personalProfileScreen.style.display = "none";
    profileScreen.style.display = "none";
    roomCreationScreen.style.display = "none";
    searchScreen.style.display = "none";
}


const searchBar = document.querySelector("#searchBar");
const searchBarLarge = document.querySelector("#searchBarLarge");
const searchResultContent = searchScreen.querySelector(".content");

function showSearchScreen(initialTerm) {
    postScreen.style.display = "none";
    postCreationScreen.style.display = "none";
    repostCreationScreen.style.display = "none";
    postThreadScreen.style.display = "none";
    personalProfileScreen.style.display = "none";
    profileScreen.style.display = "none";
    roomCreationScreen.style.display = "none";
    searchScreen.style.display = "flex";

    searchBarLarge.value = initialTerm;

    searchResultContent.innerHTML = "";
}

const roomHeadingTemplate = document.querySelector("#roomHeadingTemplate");
async function showRoomScreen(roomData) {
    showPostScreen();
    await whoAmI;

    let roomHeading = createRoomHeadingNode(roomData);

    postScreen.appendChild(roomHeading);
    postScreen.setAttribute("style", `--room-color: #${roomData.color}`);



    currentPaginator = initRoomPostPaginator(roomData.id, mountPosts(postScreen, window.localStorage.getItem("currentUsername") == roomData.owner));
    currentPaginator();
}


const notificationsScreen = document.querySelector(".notifications");
const notificationTemplate = document.querySelector("#notificationTemplate");
let notificationsPaginator = initNotificationsPaginator(storeNotifications(mountNotifications(notificationsScreen)));
notificationsPaginator();

class NotificationStore {
    
    static notifications = [];

    static append(notifications) {
        this.notifications = this.notifications.concat(notifications);
    }

    static async dispatchDelete() {
        let ids = this.notifications.map((n) => {return n.id});

        let response = await baseErrorHandler.guard(fetch(`/api/delete_notifications`, {
            method: "DELETE",
            body: JSON.stringify({
                "ids": ids
            }),
            headers: { "Content-Type": "application/json" },
        }));

        notificationsScreen.querySelectorAll(".notification").forEach((node) => {
            node.remove();
        });
    }
}

function mountNotifications(screen) {
    return (notifications) => {
        for (let i = 0; i < notifications.length; i++) {
            let node = notificationTemplate.content.cloneNode(true);

            let timestamp = new Date(notifications[i].timestamp * 1000);
            let [date, fullTime] = timestamp.toISOString().split("T");
            let [hour, minute] = fullTime.split(":");

            node.querySelector("h6").textContent = `${date} ${hour}:${minute}`;

            let content = node.querySelector("a");
            content.setAttribute("href", notifications[i].href);
            content.textContent = notifications[i].message;

            let element = node.querySelector(".notification");

            node.querySelector(".delete").addEventListener("click", async () => {
                let response = await baseErrorHandler.guard(fetch(`/api/delete_notifications`, {
                    method: "DELETE",
                    body: JSON.stringify({
                        "ids": [notifications[i].id]
                    }),
                    headers: { "Content-Type": "application/json" },
                }));

                element.style.display = "none";
            })

            screen.appendChild(node);
        }
    }
}

function initNotificationsPaginator(handler) {
    
    var counter = 0;

    return async () => {
        let response = await baseErrorHandler.guard(fetch(`/api/fetch_notifications?page=${counter++}`));

        response.json().then(handler);
    }
}


function storeNotifications(handler) {
    return (notifs) => {
        NotificationStore.append(notifs);

        handler(notifs);
    }
}

notificationsScreen.addEventListener("scrollend", (event) => {
    const scrollPos = notificationsScreen.scrollTop;
    const maxScroll = mainContent.offsetHeight - notificationsScreen.offsetHeight;

    const tolerance = 50;

    if (maxScroll - scrollPos <= tolerance) {
        notificationsPaginator();
    }
})



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
        initPersonalProfilePage();
        showPersonalProfileScreen();
        break;
    }

    case "profile": {
        const username = params.get("id");
        initProfilePage(username);
        showProfileScreen(username);
        break;
    }

    case "room": {
        const id = params.get("id");
        joinedRooms.then((rooms) => {
            showRoomScreen(rooms.find((r) => r.id == id));
        });
        break;
    }

    case "search": {
        showSearchScreen("");
        break;
    }

    default: {
        showPublicSpaceScreen();
    }
}