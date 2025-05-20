function initRoomPostPaginator(roomID, handler) {
    var counter = 0;

    return async () => {
        let response = await fetch(`/api/fetch_room_posts/${roomID}?page=${counter++}`);

        if (!response.ok) {
            response.text().then(alert);
            return;
        }

        response.json().then(handler);
    }
}

function createRoomHeadingNode(roomData) {

    let roomHeading = roomHeadingTemplate.content.cloneNode(true);
    let img = roomHeading.querySelector("img"); 
    img.setAttribute("src", `/storage/room_banner/${roomData.id}`);

    const nameText = roomHeading.querySelector("h1");
    nameText.textContent = roomData.name;
    
    const descriptionText = roomHeading.querySelector("h2");
    descriptionText.textContent = roomData.description;
    
    if (roomData.owner == window.localStorage.getItem("currentUsername")) {
        const editNameButton = roomHeading.querySelector(".roomNameContainer>button.edit");
        const nameInput = roomHeading.querySelector(".roomEditNameInput");
        nameInput.value = roomData.name;

        editNameButton.style.display = "grid";
        editNameButton.addEventListener("click", () => {
            nameInput.style.display = "block";
            editNameButton.style.display = "none";
            nameText.style.display = "none";

            nameInput.focus();
        });
        nameInput.addEventListener("keypress", (event) => {
            if (event.key === "Enter") nameInput.blur();
        });
        nameInput.addEventListener("blur", () => {
            nameInput.style.display = "none";
            editNameButton.style.display = "grid";
            nameText.style.display = "block";

            if (!nameInput.value) return;

            showModal({
                title: "Change room name?",
                body: `Do you really want to change the room name from "${roomData.name}" to "${nameInput.value}"?`,
                inputFields: [],
                choices: [
                    {
                        label: "Yes",
                        class: "good",
                        onclick: () => {updateRoomName(roomData.id, nameInput.value)}
                    },
                    {
                        label: "No",
                        class: "bad"
                    }
                ]
            });
        });


        const editDescriptionButton = roomHeading.querySelector(".roomDescriptionContainer>button.edit");
        const descriptionInput = roomHeading.querySelector(".roomEditDescriptionInput");
        descriptionInput.value = roomData.description;

        editDescriptionButton.style.display = "grid";
        editDescriptionButton.addEventListener("click", () => {
            descriptionInput.style.display = "block";
            editDescriptionButton.style.display = "none";
            descriptionText.style.display = "none";

            descriptionInput.focus();
        });
        descriptionInput.addEventListener("blur", () => {
            descriptionInput.style.display = "none";
            editDescriptionButton.style.display = "grid";
            descriptionText.style.display = "block";

            if (!descriptionInput.value) return;

            showModal({
                title: "Change room description?",
                body: `Do you really want to change the room description from "${roomData.name}" to "${descriptionInput.value}"?`,
                inputFields: [],
                choices: [
                    {
                        label: "Yes",
                        class: "good",
                        onclick: () => {updateRoomDescription(roomData.id, descriptionInput.value)}
                    },
                    {
                        label: "No",
                        class: "bad"
                    }
                ]
            });
        });
        
        const updateRoomBannerButton = roomHeading.querySelector(".updateRoomBanner");
        const updateRoomBannerFileInput = roomHeading.querySelector(".roomUpdateBannerInput");

        updateRoomBannerButton.addEventListener("click", () => {updateRoomBannerFileInput.click()});
        updateRoomBannerFileInput.addEventListener("input", () => {
            showModal({
                title: "Change room banner?",
                body: `Do you really want to change the room banner?\n\nNotice that updating the banner may take a full website relog to take effect.`,
                inputFields: [],
                choices: [
                    {
                        label: "Yes",
                        class: "good",
                        onclick: () => {handleRoomBannerUpdate(roomData.id, updateRoomBannerFileInput)}
                    },
                    {
                        label: "No",
                        class: "bad"
                    }
                ]
            });
        });
        updateRoomBannerButton.style.display = "grid";

        const updateColorInput = roomHeading.querySelector(".updateRoomColorInput");
        updateColorInput.value = `#${roomData.color}`;
        updateColorInput.addEventListener("change", () => {
            showModal({
                title: "Change room color?",
                body: `Do you really want to change the room color?`,
                inputFields: [],
                choices: [
                    {
                        label: "Yes",
                        class: "good",
                        onclick: () => {updateRoomColor(roomData.id, updateColorInput.value.substring(1))}
                    },
                    {
                        label: "No",
                        class: "bad"
                    }
                ]
            });
        });
        roomHeading.querySelector(".updateColorContainer").style.display = "flex";


        const manageUsersButton = roomHeading.querySelector(".manageUsers");
        manageUsersButton.addEventListener("click", () => {
            showManageRoomUsersModal(roomData.id);
        });
        manageUsersButton.style.display = "grid";


        const deleteRoomButton = roomHeading.querySelector(".deleteRoom");
        deleteRoomButton.addEventListener("click", () => {
            showModal({
                title: "Delete room?",
                body: `Do you really want to delete this room?\n\nThis cannot be undone and will also cause all the posts inside of this room to be deleted!`,
                inputFields: [],
                choices: [
                    {
                        label: "Yes",
                        class: "bad",
                        onclick: () => {deleteRoom(roomData.id)}
                    },
                    {
                        label: "No",
                    }
                ]
            });
        });
        deleteRoomButton.style.display = "grid";

        roomHeading.querySelector(".leaveRoom").style.display = "none";
    } else {
        roomHeading.querySelector(".leaveRoom").addEventListener("click", () => {
            showModal({
                title: "Leave room?",
                body: "This will remove you from the room. Your posts inside this room will remain. If this room is private, you will not be able to simply rejoin and have to ask the room owner to add you when want to do so.",
                inputFields: [],
                choices: [
                    {
                        label: "Leave",
                        class: "bad",
                        onclick: () => {
                            leaveRoom(roomData.id);
                        }
                    },
                    {
                        label: "Cancel"
                    }
                ]
            });
        });
    }

    return roomHeading;
}

async function updateRoomName(roomID, newName) {
    let payload = {
        "room_id": roomID,
        "new_name": newName,
    };

    let response = await fetch("/api/update_room_name", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(payload),
    });

    if (!response.ok) {
        response.text().then(alert);
        return;
    }

    window.location.reload();
}

async function updateRoomDescription(roomID, newDescription) {
    let payload = {
        "room_id": roomID,
        "new_description": newDescription,
    };

    let response = await fetch("/api/update_room_description", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(payload),
    });

    if (!response.ok) {
        response.text().then(alert);
        return;
    }

    window.location.reload();
}

async function updateRoomColor(roomID, newColor) {
    let payload = {
        "room_id": roomID,
        "new_color": newColor,
    };

    let response = await fetch("/api/update_room_color", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(payload),
    });

    if (!response.ok) {
        response.text().then(alert);
        return;
    }

    window.location.reload();
}

async function handleRoomBannerUpdate(roomID, input) {
    if (input && input.files && input.files[0]) {
        let formData = new FormData();
        
        formData.append("banner", input.files[0]);

        let response = await fetch(`/api/update_room_banner/${roomID}`, {
            method: "POST",
            body: formData,
        });

        if (!response.ok) {
            response.text().then(alert);
            return;
        }

        window.location.reload();
    }
}

async function deleteRoom(roomID) {
    let response = await fetch(`/api/delete_room/${roomID}`, {
        method: "DELETE",
    });

    if (!response.ok) {
        response.text().then(alert);
        return;
    }

    window.location.href = window.location.origin;
}

async function leaveRoom(roomID) {
    let response = await fetch(`/api/leave_room/${roomID}`, {
        method: "DELETE"
    });

    if (!response.ok) {
        response.text().then(alert);
        return;
    }

    window.location.href = window.location.origin;
}


const manageUsersModal = document.querySelector("#manageRoomUsersModal");
const usersContainer = manageUsersModal.querySelector(".usersContainer");
const addUserButton = manageUsersModal.querySelector(".addUser");

let joinedUsersPaginator = () => {};
usersContainer.addEventListener("scrollend", () => {
    const scrollPos = usersContainer.scrollTop;
    const maxScroll = usersContainer.scrollHeight - usersContainer.offsetHeight;

    const tolerance = 50;

    if (maxScroll - scrollPos < tolerance) {
        joinedUsersPaginator();
    }
});

manageUsersModal.querySelector(".close").addEventListener("click", () => {manageUsersModal.style.display = "none"});
manageUsersModal.querySelector("#manageRoomUsersSearch").addEventListener("keypress", (event) => {
    console.log(event.key)
    if (event.key === "Enter") event.target.blur()
});
function showManageRoomUsersModal(roomID) {
    manageUsersModal.style.display = "grid";

    const showMembersButton = manageUsersModal.querySelector(".members");
    const showBannedUsersButton = manageUsersModal.querySelector(".banned");

    const userSearchBar = manageUsersModal.querySelector("#manageRoomUsersSearch");

    const showMembers = () => {
        usersContainer.innerHTML = "";
    
        showMembersButton.style.color = "var(--green)";
        showMembersButton.style.fontWeight = "800";
        showMembersButton.style.textDecoration = "underline";

        showBannedUsersButton.style.color = "inherit";
        showBannedUsersButton.style.fontWeight = "inherit";
        showBannedUsersButton.style.textDecoration = "none";


        joinedUsersPaginator = initJoinedUsersPaginator(roomID, mountJoinedUsers(roomID, usersContainer));
        joinedUsersPaginator();

        userSearchBar.onblur = async () => {
            if (userSearchBar.value.length == 0) {
                showMembers();
            } else {
                joinedUsersPaginator = () => {};

                usersContainer.innerHTML = "";

                let response = await fetch(`/api/search_for_room_member/${roomID}?query=${userSearchBar.value}`);

                if (!response.ok) {
                    response.text().then(alert);
                    return;
                }

                response.json().then(mountJoinedUsers(roomID, usersContainer));
            }
        }
    }
    showMembersButton.addEventListener("click", showMembers);

    const showBannedUsers = () => {
        usersContainer.innerHTML = "";
        
        showBannedUsersButton.style.color = "var(--green)";
        showBannedUsersButton.style.fontWeight = "800";
        showBannedUsersButton.style.textDecoration = "underline";

        showMembersButton.style.color = "inherit";
        showMembersButton.style.fontWeight = "inherit";
        showMembersButton.style.textDecoration = "none";

        joinedUsersPaginator = initBannedUsersPaginator(roomID, mountBannedUsers(roomID, usersContainer));
        joinedUsersPaginator();

        userSearchBar.onblur = async () => {
            if (userSearchBar.value.length == 0) {
                showBannedUsers();
            } else {
                joinedUsersPaginator = () => {};

                usersContainer.innerHTML = "";

                let response = await fetch(`/api/search_for_banned_user/${roomID}?query=${userSearchBar.value}`);

                if (!response.ok) {
                    response.text().then(alert);
                    return;
                }

                response.json().then(mountBannedUsers(roomID, usersContainer));
            }
        }
    }
    showBannedUsersButton.addEventListener("click", showBannedUsers);

    showMembers();


    addUserButton.onclick = () => {showModal({
        title: "Add user",
        body: "Enter the username of the person you want to add.",
        inputFields: [{
            name: "username",
            placeholder: "username",
            minLength: 1,
            maxLength: 24,
        }],
        choices: [
            {
                label: "Add",
                class: "good",
                onclick: async (input) => {
                    let response = await fetch(`/api/add_user_to_room/${roomID}`, {
                        method: "POST",
                        body: JSON.stringify(input),
                        headers: {"Content-Type": "application/json"},
                    });

                    if (!response.ok) {
                        response.text().then(alert);
                        return;
                    }

                    window.location.reload();
                }
            },
            {
                label: "Cancel",
                onclick: () => {},
            }
        ]
    })};
}

function initJoinedUsersPaginator(roomID, handler) {
    var counter = 0;
    return async () => {
        let response = await baseErrorHandler.guard(fetch(`/api/fetch_joined_users/${roomID}?page=${counter++}`));

        response.json().then(handler);
    }
}

function initBannedUsersPaginator(roomID, handler) {
    var counter = 0;
    return async () => {
        let response = await baseErrorHandler.guard(fetch(`/api/fetch_banned_users/${roomID}?page=${counter++}`));
        
        response.json().then(handler);
    }
}

const joinedUserTemplate = manageUsersModal.querySelector(".userTemplate");
const bannedUserTemplate = manageUsersModal.querySelector(".bannedUserTemplate");
function mountJoinedUsers(roomID, screen) {
    const currentUsername = window.localStorage.getItem("currentUsername");
    return (users) => {
        Array.from(users).forEach((user) => {
            let node = joinedUserTemplate.content.cloneNode(true);
            node.querySelector(".profilePicture").style.backgroundImage = `url(/storage/profile_picture/${user.username})`;
            node.querySelector("h4").textContent = user.public_name;
            node.querySelector("h5").textContent = user.username;

            node.querySelector("a").setAttribute("href", `?view=profile&id=${user.username}`);

            const kickButton = node.querySelector(".kick");
            if (user.username == currentUsername) kickButton.style.display = "none";
            else kickButton.addEventListener("click", () => {showModal({
                title: "Kick user?",
                body: `Do you really want to kick ${user.public_name} from this room? This will remove them from the room. Notice that if the room is public, they can join the room again at any time.`,
                inputFields: [],
                choices: [
                    {
                        label: "Kick",
                        class: "bad",
                        onclick: async () => {
                            let response = await fetch(`/api/kick_user_from_room/${roomID}/${user.username}`, {
                                method: "DELETE",
                            });

                            if (!response.ok) {
                                response.text().then(alert);
                                return;
                            }

                            window.location.reload();
                        }
                    },
                    {
                        label: "Cancel",
                        onclick: () => {}
                    }
                ]
            })});

            const banButton = node.querySelector(".ban");
            if (user.username == currentUsername) banButton.style.display = "none";
            else banButton.addEventListener("click", () => {showModal({
                title: "Ban user?",
                body: `Do you really want to ban ${user.public_name} from this room? This will remove them from the room and prevent them from joining it again.`,
                inputFields: [],
                choices: [
                    {
                        label: "Ban",
                        class: "bad",
                        onclick: async () => {
                            let response = await fetch(`/api/ban_user_from_room/${roomID}/${user.username}`, {
                                method: "DELETE",
                            });

                            if (!response.ok) {
                                response.text().then(alert);
                                return;
                            }

                            window.location.reload();
                        }
                    },
                    {
                        label: "Cancel",
                        onclick: () => {}
                    }
                ]
            })});

            screen.appendChild(node);
        });
    }
}

function mountBannedUsers(roomID, screen) {
    return (users) => {
        Array.from(users).forEach((user) => {
            let node = bannedUserTemplate.content.cloneNode(true);
            node.querySelector(".profilePicture").style.backgroundImage = `url(/storage/profile_picture/${user.username})`;
            node.querySelector("h4").textContent = user.public_name;
            node.querySelector("h5").textContent = user.username;

            node.querySelector("a").setAttribute("href", `?view=profile&id=${user.username}`);

            const unbanButton = node.querySelector(".unban");
            unbanButton.addEventListener("click", () => {showModal({
                title: "Unban user?",
                body: `Do you really want to unban ${user.public_name} from this room? If this room is public, this will allow them to join the room.`,
                inputFields: [],
                choices: [
                    {
                        label: "Unban",
                        class: "bad",
                        onclick: async () => {
                            let response = await fetch(`/api/unban_user_from_room/${roomID}/${user.username}`, {
                                method: "POST",
                            });

                            if (!response.ok) {
                                response.text().then(alert);
                                return;
                            }

                            window.location.reload();
                        }
                    },
                    {
                        label: "Cancel",
                        onclick: () => {}
                    }
                ]
            })});

            screen.appendChild(node);
        });
    }
}

async function joinRoom(roomID) {
    let response = await baseErrorHandler.guard(fetch(`/api/join_room/${roomID}`, {method: "POST"}));

    window.location.reload();
}