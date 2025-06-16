let whoAmI = new Promise((resolve, reject) => {
    fetch("api/who_am_i").then((response) => {
        response.json().then((json) => {
            let currentUser = document.querySelector("#currentUser");
            let mobileCurrentUser = document.querySelector("#mobilePersonalProfileButton");
    
            currentUser.querySelector(".username").textContent = json.public_name;
            currentUser.querySelector(".profilePicture").setAttribute("style", `background-image: url('storage/profile_picture/${json.username}')`);

            mobileCurrentUser.querySelector(".profilePicture").setAttribute("style", `background-image: url('storage/profile_picture/${json.username}')`);
    
            window.localStorage.setItem("currentUsername", json.username);
            window.localStorage.setItem("currentPublicName", json.public_name);

            resolve(json);
        });
    });
});

const roomButtonTamplate = document.querySelector("#roomButtonTemplate");
const roomButtonContainer = document.querySelector("#myRooms");

const mobileRoomSelectorScreen = Screen.fromElementId("mobileMyRooms", null, null);

const newPostRoomSelector = document.querySelector("#newPostRoom");
const mobileRoomsButton = document.querySelector("#mobileRoomsButton");
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

                roomButtonContainer.appendChild(node.cloneNode(true));
                mobileRoomSelectorScreen.domNode.appendChild(node);

                let option = document.createElement("option")
                option.setAttribute("value", room.id);
                option.innerText = room.name;
                newPostRoomSelector.appendChild(option);
            });
            resolve(rooms);
        });
    });
});

const mobileAddScreen = Screen.fromElementId("mobileAddMenu", null, null);

async function logout() {
    let response = await baseErrorHandler.guard(fetch("/logout", {method: "DELETE"}));

    document.cookie = "session_id= ;";
    window.location.reload();
}

const mainContent = document.querySelector("#mainContent");
const postScreen = Screen.fromElementId("postScreen", showPostScreen, null);
const postCreationScreen = Screen.fromElementId("postCreation", showPostCreationScreen, null);
const repostCreationScreen = Screen.fromElementId("repostCreation", showRepostCreationScreen, null);
const postThreadScreen = Screen.fromElementId("postThread", showPostThreadScreen, null);
const postThreadParentsSection = postThreadScreen.querySelector(".parentPostsContainer");
const postThreadFocusedSlot = postThreadScreen.querySelector(".focusedPost");
const postThreadCommentSection = postThreadScreen.querySelector(".childPosts");
const personalProfileScreen = Screen.fromElementId("personalProfile", showPersonalProfileScreen, null);
const profileScreen = Screen.fromElementId("profile", showProfileScreen, null);
const roomCreationScreen = Screen.fromElementId("roomCreation", showRoomCreationScreen, null);
const searchScreen = Screen.fromElementId("search", null, null);

const mobileBottomBar = document.querySelector(".bottomBar");
const mobileFeedSelector = document.querySelector("#mobileFeedSelector");
function hideMobileFeedSelector() {
    mobileFeedSelector.style.display = "none";
}

const showPublicSpaceButton = document.querySelector("#showPublicSpaceButton");
const mobileShowFeedButton = document.querySelector("#mobileFeedButton");
const mobileShowPublicSpaceButton = document.querySelector("#mobilePublicSpaceSelectorButton");
const mobileShowFollowingButton = document.querySelector("#mobileFollowingSelectorButton");
const mobileShowNotificationsButton = document.querySelector("#mobileNotificationsButton");
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

    let params = new URL(window.location.href).searchParams;

    if (params.get("view") == "room") {
        let roomID = params.get("id");
        
        if (roomID) document.querySelector("#newPostRoom").value = roomID;
    }

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
    postScreen.domNode.innerHTML = "";
}

function showRoomCreationScreen() {
    currentPaginator = noPaginator;
}

function showPublicSpaceScreen() {
    window.history.pushState({}, "", window.location.origin);

    showPublicSpaceButton.setAttribute("selected", "");
    showFollowingFeedButton.removeAttribute("selected")

    mobileShowFeedButton.setAttribute("selected", "");
    mobileShowPublicSpaceButton.setAttribute("selected", "");
    mobileShowFollowingButton.removeAttribute("selected");
    mobileShowNotificationsButton.removeAttribute("selected");

    document.querySelector("#mobileRoomsButton").removeAttribute("selected");

    mainContentScreenSwitch.showScreen("postScreen");
    currentPaginator = initPublicSpacePaginator(mountPosts(postScreen));
    currentPaginator();
}

function showFollowingScreen() {

    showFollowingFeedButton.setAttribute("selected", "");
    showPublicSpaceButton.removeAttribute("selected");

    mobileShowFeedButton.setAttribute("selected", "");
    mobileShowFollowingButton.setAttribute("selected", "");
    mobileShowPublicSpaceButton.removeAttribute("selected");
    mobileShowNotificationsButton.removeAttribute("selected");

    document.querySelector("#mobileRoomsButton").removeAttribute("selected");

    mainContentScreenSwitch.showScreen("postScreen");
    currentPaginator = initFollowedFeedPaginator(mountPosts(postScreen));
    currentPaginator();
}

function showBookmarkedScreen() {
    mobileFeedSelector.style.display = "none";
    mobileShowFeedButton.setAttribute("selected", "");
    mobileShowNotificationsButton.removeAttribute("selected");

    mainContentScreenSwitch.showScreen("postScreen");

    currentPaginator = initBookmarkedPaginator(mountPosts(postScreen));
    currentPaginator();
}

function showPersonalProfileScreen() {
    mobileShowFeedButton.removeAttribute("selected");
    mobileRoomsButton.removeAttribute("selected");
    mobileShowNotificationsButton.removeAttribute("selected");

    const currentUsername = window.localStorage.getItem("currentUsername");
    const postsContainer = personalProfileScreen.querySelector(".posts");

    currentPaginator = initUsersPostsPaginator(currentUsername, mountPosts(postsContainer));
    currentPaginator();
}

function showProfileScreen(username) {
    mobileShowFeedButton.removeAttribute("selected");
    mobileRoomsButton.removeAttribute("selected");
    mobileShowNotificationsButton.removeAttribute("selected");

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
}


const searchBar = document.querySelector("#searchBar");
const searchBarLarge = document.querySelector("#searchBarLarge");
const searchResultContent = searchScreen.querySelector(".content");

function showSearchScreen(initialTerm) {
    mainContentScreenSwitch.showScreen("search");

    mobileShowFeedButton.removeAttribute("selected");

    searchBarLarge.value = initialTerm;

    searchResultContent.innerHTML = "";
}

const roomHeadingTemplate = document.querySelector("#roomHeadingTemplate");
async function showRoomScreen(roomData) {
    
    mainContentScreenSwitch.showScreen("postScreen");

    await whoAmI;

    let roomHeading = createRoomHeadingNode(roomData);

    postScreen.appendChild(roomHeading);
    postScreen.setAttribute("style", `--room-color: #${roomData.color}`);


    currentPaginator = initRoomPostPaginator(roomData.id, mountPosts(postScreen, window.localStorage.getItem("currentUsername") == roomData.owner));
    currentPaginator();
}


const mobileNotificationsScreen = Screen.fromElementId("mobileNotificationsScreen", showMobileNotificationsScreen, null);
const notificationsScreen = document.querySelector(".notifications");
const notificationTemplate = document.querySelector("#notificationTemplate");

function showMobileNotificationsScreen() {
    mobileNotificationsScreen.innerHTML = "";

    currentPaginator = initNotificationsPaginator(storeNotifications(mountNotifications(mobileNotificationsScreen)));
    currentPaginator();

    mobileShowNotificationsButton.setAttribute("selected", "");
    mobileShowFeedButton.removeAttribute("selected");
    mobileRoomsButton.removeAttribute("selected");
}

let notificationsPaginator = initNotificationsPaginator(storeNotifications(mountNotifications(notificationsScreen)));
notificationsPaginator();

class NotificationStore {
    
    static notifications = [];

    static empty = true;

    static append(notifications) {
        if (notifications.length == 0) return;

        this.notifications = this.notifications.concat(notifications);

        if (this.empty) {
            this.empty = false;

            document.querySelector("#mobileNotificationsButton").setAttribute("blink", "");
        }
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

            let timestamp = new Date(notifications[i].timestamp * 1000 - new Date().getTimezoneOffset() * 60000);
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

                element.remove();

                if (mobileNotificationsScreen.children.length == 0) {
                    document.querySelector("#mobileNotificationsButton").removeAttribute("blink");
                }
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

const mainContentScreenSwitch = new ScreenSwitch()
    .withDefaultScreen(postScreen)
    .withScreen(profileScreen)
    .withScreen(mobileAddScreen)
    .withScreen(postThreadScreen)
    .withScreen(postCreationScreen)
    .withScreen(roomCreationScreen)
    .withScreen(repostCreationScreen)
    .withScreen(personalProfileScreen)
    .withScreen(mobileRoomSelectorScreen)
    .withScreen(mobileNotificationsScreen)
    .withScreen(searchScreen);


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
        mainContentScreenSwitch.showScreen("postThread", params.get("id"));
        break;
    }

    case "me": {
        initPersonalProfilePage();
        mainContentScreenSwitch.showScreen("personalProfile");
        break;
    }

    case "profile": {
        const username = params.get("id");
        initProfilePage(username);
        mainContentScreenSwitch.showScreen("profile", username);
        break;
    }

    case "room": {
        const id = params.get("id");
        joinedRooms.then((rooms) => {
            let r = rooms.find((r) => r.id == id);
            if (r) {
                showRoomScreen(r);
                document.querySelector("#mobileRoomsButton").setAttribute("selected", "");
            }
            else {
                showModal({
                    title: "Not a member",
                    body: "You are not a member of this room yet. Do you want to join it?",
                    inputFields: [],
                    choices: [
                        {
                            label: "Yes",
                            class: "good",
                            onclick: () => {
                                joinRoom(id);
                            }
                        },
                        {
                            label: "Cancel",
                            onclick: () => {
                                window.location.href = window.location.origin;
                            }
                        }
                    ]
                })
            }
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