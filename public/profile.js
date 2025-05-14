async function initProfilePage(username) {
    let userData = await UserDataStore.get(username);

    const heading = profileScreen.querySelector(".heading");
    const profilePicture = heading.querySelector(".profilePicture");
    const publicNameText = heading.querySelector("h3");
    const usernameText = heading.querySelector("h4");
    const biography = profileScreen.querySelector(".biography");
    const biographyText = biography.querySelector("p");
    const followButton = heading.querySelector(".follow");

    profilePicture.style.backgroundImage = `url(/storage/profile_picture/${username})`;

    publicNameText.innerText = userData.public_name;
    usernameText.innerText = username;
    biographyText.innerText = userData.biography;

    if (username === window.localStorage.getItem("currentUsername")) {
        followButton.style.display = "none";
    } else {
        if (userData.is_followed) {
            followButton.textContent = "Unfollow";
            followButton.style.backgroundColor = "var(--gray-125)";
            followButton.onclick = async () => {
                let response = await fetch(`/api/unfollow/${username}`, { method: "DELETE" });
                if (!response.ok) {
                    response.text().then(alert);
                    return;
                }
    
                window.location.reload();
            }
        } else {
            followButton.onclick = async () => {
                let response = await fetch(`/api/follow/${username}`, { method: "POST" });
                if (!response.ok) {
                    response.text().then(alert);
                    return;
                }
    
                window.location.reload();
            }
        }
    }
}