searchBar.addEventListener("blur", () => {
    handleSearchInput(searchBar);
});
searchBar.addEventListener("keypress", (event) => {
    if (event.key === "Enter")
    handleSearchInput(searchBar);
});

searchBarLarge.addEventListener("blur", () => {
    handleSearchInput(searchBarLarge);
});
searchBarLarge.addEventListener("keypress", (event) => {
    if (event.key === "Enter")
    handleSearchInput(searchBarLarge);
});

function handleSearchInput(inputField) {
    if (inputField.value.length !== 0) {
        window.history.pushState({}, "", `${window.location.origin}?view=search`);

        showSearchScreen(inputField.value);

        currentPaginator = initSearchResultPaginator(inputField.value, mountSearchResults(searchResultContent));
        currentPaginator();
    }
}

let searchCategory = "users";

const searchResultContainer = searchScreen.querySelector(".content");

const [searchPeopleButton, searchRoomsButton, searchPostsButton] = [
    searchScreen.querySelector(".searchPeopleButton"),
    searchScreen.querySelector(".searchRoomsButton"),
    searchScreen.querySelector(".searchPostsButton")
];
searchPeopleButton.addEventListener("click", () => {
    searchCategory = "users";

    searchPeopleButton.classList.add("active");
    searchRoomsButton.classList.remove("active");
    searchPostsButton.classList.remove("active");

    searchResultContainer.classList.remove("grid");

    handleSearchInput(searchBarLarge);
});
searchRoomsButton.addEventListener("click", () => {
    searchCategory = "rooms";

    searchPeopleButton.classList.remove("active");
    searchRoomsButton.classList.add("active");
    searchPostsButton.classList.remove("active");

    searchResultContainer.classList.add("grid");

    handleSearchInput(searchBarLarge);
});
searchPostsButton.addEventListener("click", () => {
    searchCategory = "posts";

    searchPeopleButton.classList.remove("active");
    searchRoomsButton.classList.remove("active");
    searchPostsButton.classList.add("active");

    searchResultContainer.classList.remove("grid");

    handleSearchInput(searchBarLarge);
});


function initSearchResultPaginator(searchTerm, handler) {
    var counter = 0;
    
    return async () => {
        let response = await baseErrorHandler.guard(fetch(`/api/search/${searchCategory}/?query=${searchTerm}&page=${counter++}`));

        response.json().then(handler);
    }
}

function mountSearchResults(screen) {
    const handlerTable = {
        "Posts": mountPosts(screen),
        "Rooms": mountRooms(screen),
        "Users": mountUsers(screen),
    }

    return (data) => {

        let result_type = Object.keys(data)[0]

        handlerTable[result_type](data[result_type]);
    }
}

function mountRooms(screen) {
    return (rooms) => {
        for (let i = 0; i < rooms.length; i++) {
            screen.appendChild(makeRoomNode(rooms[i]));
        }
    }
}

function mountUsers(screen) {
    return (users) => {
        for (let i = 0; i < users.length; i++) {
            screen.appendChild(makeUserNode(users[i]));
        }
    }
}

const roomSearchResultTemplate = document.querySelector("#roomSearchResult");
function makeRoomNode(data) {
    let node = roomSearchResultTemplate.content.cloneNode(true);

    let { room, joined } = data;

    node.querySelector(".roomSearchResult").setAttribute("style", `--room-color: #${room.color}`);

    node.querySelector(".banner").style.backgroundImage = `url("/storage/room_banner/${room.id}")`;

    node.querySelector("h3").textContent = room.name;
    node.querySelector("p").textContent = room.description;

    let joinButton = node.querySelector(".join");
    if (!joined) {
        joinButton.addEventListener("click", () => {
            joinRoom(room.id);
        });
    } else {
        joinButton.setAttribute("disabled", "");
        joinButton.textContent = "joined";
    }

    return node;
}


const userSearchResultTemplate = document.querySelector("#userSearchResult");
function makeUserNode(user) {
    let node = userSearchResultTemplate.content.cloneNode(true);

    node.querySelector(".profilePicture").style.backgroundImage = `url("/storage/profile_picture/${user.username}")`;

    node.querySelector("h4").textContent = user.public_name;
    node.querySelector("h5").textContent = user.username;
    node.querySelector("p").textContent = user.biography;

    node.querySelector("a").setAttribute("href", `?view=profile&id=${user.username}`);

    return node;
}