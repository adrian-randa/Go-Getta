let manageFollowPaginator = () => {};

const manageFollowModal = document.querySelector("#manageFollowsModal");

async function initPersonalProfilePage() {
    const heading = personalProfileScreen.querySelector(".heading");
    const profilePicture = heading.querySelector(".profilePicture");
    const publicNameText = heading.querySelector("h3");
    const usernameText = heading.querySelector("h4");
    const biography = personalProfileScreen.querySelector(".biography");
    const biographyText = biography.querySelector("p");

    const currentUsername = window.localStorage.getItem("currentUsername");

    let userDataResponse = await baseErrorHandler.guard(fetch(`/api/get_user_data/${currentUsername}`));
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


    const viewFollowersButton = personalProfileScreen.querySelector(".viewFollowers");
    const viewFollowedButton = personalProfileScreen.querySelector(".viewFollowed");
    const manageFollowUsersContainer = manageFollowModal.querySelector(".usersContainer");

    viewFollowersButton.textContent = `View ${userData.followers} followers`;
    viewFollowersButton.onclick = () => {
        manageFollowModal.querySelector("h1").textContent = "Followers";

        manageFollowUsersContainer.innerHTML = "";

        manageFollowPaginator = initFollowerPaginator(mountFollowers(manageFollowUsersContainer));
        manageFollowPaginator();

        manageFollowModal.style.display = "grid";
    }

    viewFollowedButton.textContent = `View ${userData.followed} followed`;
    viewFollowedButton.onclick = () => {
        manageFollowModal.querySelector("h1").textContent = "Followed";

        manageFollowUsersContainer.innerHTML = "";

        manageFollowPaginator = initFollowedPaginator(mountFollowed(manageFollowUsersContainer));
        manageFollowPaginator();

        manageFollowModal.style.display = "grid";
    }
}

document.querySelector("#manageFollowsModal").querySelector(".usersContainer").addEventListener("scrollend", (event) => {
    let usersContainer = event.target;
    
    const scrollPos = usersContainer.scrollTop;
    const maxScroll = usersContainer.scrollHeight - usersContainer.offsetHeight;

    const tolerance = 50;

    if (maxScroll - scrollPos < tolerance) {
        manageFollowPaginator();
    }
});

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

    let response = await fileUploadErrorHandler.guard(fetch("/api/update_profile_picture", {
        method: "POST", body: formData
    }));

    window.location.reload();
}

async function setNewPublicName(newPublicName) {
    let payload = {
        "new_public_name": newPublicName
    };

    let response = await baseErrorHandler.guard(fetch("/api/update_public_name", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(payload)
    }));

    if (response.ok) window.location.reload();
    else alert(await response.text());
}

async function updateBiography(newBiography) {
    let payload = {
        "new_biography": newBiography
    };

    let response = await baseErrorHandler.guard(fetch("/api/update_biography", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(payload)
    }));

    window.location.reload();
}


function initBookmarkedPaginator(handler) {
    var counter = 0;

    return async () => {
        let response = await baseErrorHandler.guard(fetch(`/api/fetch_bookmarked_posts?page=${counter++}`));

        response.json().then(handler);
    }
}

function initFollowerPaginator(handler) {
    
    var counter = 0;

    return async () => {
        let response = await baseErrorHandler.guard(fetch(`/api/fetch_followers?page=${counter++}`));

        response.json().then(handler);
    }
}

function initFollowedPaginator(handler) {
    
    var counter = 0;

    return async () => {
        let response = await baseErrorHandler.guard(fetch(`/api/fetch_followed?page=${counter++}`));

        response.json().then(handler);
    }
}

function mountFollowers(screen) {
    console.log(manageFollowModal);
    const followerTemplate = manageFollowModal.querySelector(".followerTemplate");

    return (users) => {
        for (let i = 0; i < users.length; i++) {
            let node = followerTemplate.content.cloneNode(true);

            node.querySelector("a").setAttribute("href", `?view=profile&id=${users[i].username}`);

            node.querySelector(".profilePicture").style.backgroundImage = `url("/storage/profile_picture/${users[i].username}")`;
            node.querySelector("h4").textContent = users[i].public_name;
            node.querySelector("h5").textContent = users[i].username;

            screen.appendChild(node);
        }
    }
}

function mountFollowed(screen) {
    const followedTemplate = manageFollowModal.querySelector(".followedTemplate");

    return (users) => {
        for (let i = 0; i < users.length; i++) {
            let node = followedTemplate.content.cloneNode(true);

            node.querySelector("a").setAttribute("href", `?view=profile&id=${users[i].username}`);

            node.querySelector(".profilePicture").style.backgroundImage = `url("/storage/profile_picture/${users[i].username}")`;
            node.querySelector("h4").textContent = users[i].public_name;
            node.querySelector("h5").textContent = users[i].username;

            node.querySelector(".unfollow").addEventListener("click", async () => {
                let response = await fetch(`/api/unfollow/${users[i].username}`, { method: "DELETE" });

                if (!response.ok) {
                    response.text().then(alert);
                    return;
                }

                window.location.reload();
            })

            screen.appendChild(node);
        }
    }
}

document.querySelector("#manageFollowsModal").querySelector(".close").addEventListener("click", () => {
    document.querySelector("#manageFollowsModal").style.display = "none";
});

document.querySelector("#manageFollowsModal").querySelector(".backdrop").addEventListener("click", () => {
    document.querySelector("#manageFollowsModal").style.display = "none";
});