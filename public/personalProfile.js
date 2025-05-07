async function initPersonalProfilePage() {
    const heading = personalProfileScreen.querySelector(".heading");
    const profilePicture = heading.querySelector(".profilePicture");
    const publicNameText = heading.querySelector("h3");
    const usernameText = heading.querySelector("h4");
    const biography = personalProfileScreen.querySelector(".biography");
    const biographyText = biography.querySelector("p");

    const currentUsername = window.localStorage.getItem("currentUsername");

    let userDataResponse = await fetch(`/api/get_user_data/${currentUsername}`);
    let userData = await userDataResponse.json();

    profilePicture.style.backgroundImage = `url(/storage/profile_picture/${currentUsername})`;
    publicNameText.innerText = userData.public_name;
    usernameText.innerText = currentUsername;
    biographyText.innerText = userData.biography;


    const profilePictureForm = heading.querySelector("form");
    const profilePictureInput = profilePictureForm.querySelector("input");

    heading.querySelector("button.edit").addEventListener("click", () => {profilePictureInput.click()});

    const editPublicNameButton = heading.querySelector(".publicNameContainer").querySelector("button.edit");
    const newPublicNameInput = heading.querySelector(".publicNameContainer").querySelector("input");
    newPublicNameInput.setAttribute("placeholder", userData.public_name);

    function terminatePublicNameEditState() {
        publicNameText.style.display = "block";
        newPublicNameInput.style.display = "none";
        editPublicNameButton.style.display = "grid";
    }

    editPublicNameButton.addEventListener("click", (event) => {
        publicNameText.style.display = "none";
        newPublicNameInput.style.display = "block";
        editPublicNameButton.style.display = "none";

        newPublicNameInput.focus();
    });

    function handlePublicNameInput() {
        if (newPublicNameInput.value && newPublicNameInput.value != "") {
            showModal({
                title: "Change public name?",
                body: `Do you really want to change your public name from "${userData.public_name}" to "${newPublicNameInput.value}"?`,
                inputFields: [],
                choices: [
                    {
                        label: "Yes",
                        class: "good",
                        onclick: () => {setNewPublicName(newPublicNameInput.value)}
                    },
                    {
                        label: "No",
                    }
                ]
            });
        }
    
        terminatePublicNameEditState();
    }

    newPublicNameInput.addEventListener("blur", handlePublicNameInput);
    newPublicNameInput.addEventListener("keypress", (event) => {
        if (event.key === "Enter") handlePublicNameInput();
    })


    const newBiographyInput = biography.querySelector("textarea");
    const editBiographyButton = biography.querySelector("button.edit");

    function handleBiographyInput() {

        showModal({
            title: "Change biography?",
            body: "Do really you want to change biography?",
            inputFields: [],
            choices: [
                {
                    label: "Yes",
                    class: "good",
                    onclick: () => {updateBiography(newBiographyInput.value)}
                },
                {
                    label: "No",
                    onclick: () => {}
                }
            ]
        });

        terminateBiographyEditState();
    }

    function terminateBiographyEditState() {
        newBiographyInput.style.display = "none";
        biographyText.style.display = "block";
        editBiographyButton.style.display = "grid";
    }

    editBiographyButton.addEventListener("click", (event) => {
        newBiographyInput.style.display = "block";
        biographyText.style.display = "none";
        editBiographyButton.style.display = "none";

        newBiographyInput.value = userData.biography;
        newBiographyInput.focus();
    });

    newBiographyInput.addEventListener("blur", handleBiographyInput);

    const viewBookmarkedPostsButton = personalProfileScreen.querySelector(".viewBookmarkedPostsButton");
    viewBookmarkedPostsButton.addEventListener("click", () => {
        showBookmarkedScreen();
    });
}

function handleProfilePictureUpdate(input) {

    showModal({
        title: "Update profile picture?",
        body: "Do you really want to set the selected image as your profile picture?",
        inputFields: [],
        choices: [
            {
                label: "Yes",
                class: "good",
                onclick: () => {updateProfilePicture(input)}
            },
            {
                label: "No",
                onclick: () => {}
            }
        ]
    });

}

async function updateProfilePicture(input) {
    if (!input || !input.files) return;
    
    let formData = new FormData();

    formData.append("profile_picture", input.files[0]);

    let response = await fetch("/api/update_profile_picture", {
        method: "POST", body: formData
    });

    if (!response.ok) {
        alert(await response.text());
        return;
    }

    window.location.reload();
}

async function setNewPublicName(newPublicName) {
    let payload = {
        "new_public_name": newPublicName
    };

    let response = await fetch("/api/update_public_name", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(payload)
    });

    if (response.ok) window.location.reload();
    else alert(await response.text());
}

async function updateBiography(newBiography) {
    let payload = {
        "new_biography": newBiography
    };

    let response = await fetch("/api/update_biography", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(payload)
    });

    if (response.ok) window.location.reload();
    else alert(await response.text());
}


function initBookmarkedPaginator(handler) {
    var counter = 0;

    return async () => {
        let response = await fetch(`/api/fetch_bookmarked_posts?page=${counter++}`);

        if (!response.ok) {
            response.text().then(alert);
            return;
        }

        response.json().then(handler);
    }
}