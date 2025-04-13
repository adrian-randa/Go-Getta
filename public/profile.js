async function initProfilePage(username) {
    let userDataResponse = await fetch(`/api/get_user_data/${username}`);
    const userData = await userDataResponse.json();

    const heading = profileScreen.querySelector(".heading");
    const profilePicture = heading.querySelector(".profilePicture");
    const publicNameText = heading.querySelector("h3");
    const usernameText = heading.querySelector("h4");
    const biography = profileScreen.querySelector(".biography");
    const biographyText = biography.querySelector("p");

    profilePicture.style.backgroundImage = `url(/storage/profile_picture/${username})`;

    publicNameText.innerText = userData.public_name;
    usernameText.innerText = username;
    biographyText.innerText = userData.biography;
}