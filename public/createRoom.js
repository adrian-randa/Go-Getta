const newRoomBannerInput = roomCreationScreen.querySelector("#newRoomBanner");
const newRoomBannerPreview = roomCreationScreen.querySelector("#newRoomBannerPreview");
const newRoomForm = roomCreationScreen.querySelector("#newRoomSettings");
const newRoomNameInput = newRoomForm.querySelector("#newRoomName");
const newRoomDescriptionInput = newRoomForm.querySelector("#newRoomDescription");
const newRoomPrivateToggle = newRoomForm.querySelector(".privateToggleButton");
const newRoomColorInput = newRoomForm.querySelector("#newRoomColor");

function browseNewRoomBannerImage() {
    newRoomBannerInput.click();
}

function handleNewRoomBannerInput(input) {
    if (input && input.files && input.files[0]) {
        let reader = new FileReader();
    
        reader.onload = (event) => {
            newRoomBannerPreview.setAttribute("src", event.target.result);
        }
    
        reader.readAsDataURL(input.files[0]);
    }
}

let newRoomIsPrivate = false;
const newRoomPrivateButtonHalves = {
    public: newRoomPrivateToggle.querySelector(".public"),
    private: newRoomPrivateToggle.querySelector(".private")
};

function toggleNewRoomIsPrivateState() {
    newRoomIsPrivate = !newRoomIsPrivate;

    let { public, private } = newRoomPrivateButtonHalves;

    let [active, passive] = newRoomIsPrivate ? [private, public] : [public, private];

    active.style.color = "var(--green)";
    active.style.backgroundColor = "var(--green-25)";

    passive.style.color = "var(--gray)";
    passive.style.backgroundColor = "var(--gray-25)";
}

async function openNewRoom() {
    let payload = {
        "name": newRoomNameInput.value,
        "description": newRoomDescriptionInput.value,
        "color": newRoomColorInput.value.substring(1),
        "is_private": newRoomIsPrivate,
    };

    let response = await baseErrorHandler.guard(fetch("/api/create_room", {
        method: "POST",
        headers: {"Content-Type": "application/json"},
        body: JSON.stringify(payload)
    }));

    let { room_id } = await response.json();

    await setRoomBanner(room_id, newRoomBannerInput);

    window.location.href = `${window.location.origin}?view=room&id=${room_id}`;
}

async function setRoomBanner(roomID, fileInput) {
    if (fileInput && fileInput.files && fileInput.files[0]) {
        let formData = new FormData();
    
        formData.append("banner", fileInput.files[0]);
        
        let response = await fileUploadErrorHandler.guard(fetch(`/api/update_room_banner/${roomID}`, {
            method: "POST",
            body: formData
        }));
    }
}