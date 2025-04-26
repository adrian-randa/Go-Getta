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
    }

    const updateRoomBannerButton = roomHeading.querySelector(".updateRoomBanner");
    const updateRoomBannerFileInput = roomHeading.querySelector(".roomUpdateBannerInput");

    updateRoomBannerButton.addEventListener("click", () => {updateRoomBannerFileInput.click()});
    updateRoomBannerFileInput.addEventListener("input", () => {
        showModal({
            title: "Change room banner?",
            body: `Do you really want to change the room banner?\n\nNotice that updating the banner may take a full website relog to take effect.`,
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


    const deleteRoomButton = roomHeading.querySelector(".deleteRoom");
    deleteRoomButton.addEventListener("click", () => {
        showModal({
            title: "Delete room?",
            body: `Do you really want to delete this room?\n\nThis cannot be undone and will also cause all the posts inside of this room to be deleted!`,
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