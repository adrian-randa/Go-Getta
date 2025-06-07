const postTemplate = document.querySelector("#postTemplate");
const mountPosts = (screen, showDeleteButton = false) => {
    return async (posts) => {
        for (let i = 0; i < posts.length; i++) {
            screen.appendChild(await makePostNode(posts[i], showDeleteButton));
        }
    }
}

const mediaTypeLookup = {
    "Image": "img",
    "Video": "video",
    "Audio": "audio",
}

async function makePostNode(post, showDeleteButton = false, showNsfw = false) {
    let node = postTemplate.content.cloneNode(true);

    await applyPostDataToNode(post, node, showDeleteButton, showNsfw);

    return node;
}

async function applyPostDataToNode(data, node, showDeleteButton = false, showNsfw = false) {

    const { post, interaction, child } = data;

    if (node instanceof DocumentFragment) node.querySelector(".post").setAttribute("id", `post-${post.id}`);

    let userData = await UserDataStore.get(post.creator);

    let creatorDisplay = node.querySelector(".userDisplay");
    creatorDisplay.setAttribute("href", `?view=profile&id=${post.creator}`);
    creatorDisplay.querySelector("h4").textContent = userData.public_name;
    creatorDisplay.querySelector("h5").textContent = post.creator;
    creatorDisplay.querySelector(".profilePicture").style.backgroundImage = `url("/storage/profile_picture/${post.creator}")`;

    let threadInfo = node.querySelector(".threadInfoContainer");
    if (!post.parent) threadInfo.style.display = "none";
    else {
        threadInfo.setAttribute("href", `?view=post&id=${post.parent}`);
    }

    let timestamp = new Date(post.timestamp * 1000 - new Date().getTimezoneOffset() * 60000);
    let [date, fullTime] = timestamp.toISOString().split("T");
    let [hour, minute] = fullTime.split(":");

    let timestampDisplay = node.querySelector(".timestamp");
    let [dateDisplay, timeDisplay] = timestampDisplay.querySelectorAll("h5");
    dateDisplay.textContent = date;
    timeDisplay.textContent = `${hour}:${minute}`;

    if (!post.is_nsfw || showNsfw) {
        if (post.appendage_id) {
            let appendageResponse = await fetch(`/storage/appendage/${post.appendage_id}`);
            if (appendageResponse.ok) {
                let appendage = await appendageResponse.json();
        
                const mediaContainer = node.querySelector(".appendages");
                
                appendage.files.forEach((file) => {
                    let mediaType = mediaTypeLookup[file.file_type];
                    let mediaNode = document.createElement(mediaType);
                    mediaNode.setAttribute("src", `/storage/appendage/file/${file.file_id}`);
                    if (mediaType == "video") mediaNode.setAttribute("controls", "");
                    mediaContainer.appendChild(mediaNode);
                })
            }
        }

        let content = node.querySelector(".content");
        content.textContent = post.body;
    
        const urlRegex = /([(http(s)?):\/\/(www\.)?a-zA-Z0-9@:%._\+~#=]{2,256}\.[a-z]{2,6}\b([-a-zA-Z0-9@:%_\+.~#?&//=]*))/ig
        const userReferenceRegex = /@([\wöäüß]+)/u
    
        content.innerHTML = content.innerHTML.replace(urlRegex, (match) => {
            let href = match;
            if (!match.startsWith("http")) href = `http://${match}`
            return `<a rel="external" href="${href}">${match}</a>`;
        });
    
        content.innerHTML = content.innerHTML.replace(userReferenceRegex, (match) => {
            let username = match;
            return `<a href="?view=profile&id=${username.substring(1)}">${username}</a>`;
        });
        
        let body = node.querySelector(".body");
        body.setAttribute("href", `?view=post&id=${post.id}`);

        const referencedPost = node.querySelector(".referencedPost");
        if (post.child) {
            if (child) {
                referencedPost.appendChild(await makeChildPostNode(child));
                referencedPost.setAttribute("href", `?view=post&id=${child.id}`);
            }
            else referencedPost.innerText = "The referenced post has been deleted.";
        } else referencedPost.style.display = "none";
    } else {
        let body = node.querySelector(".body");

        node.querySelector(".referencedPost").remove();

        body.setAttribute("style", "display: flex; flex-direction: column; align-items: center; justify-content: center; gap: 1rem;")

        body.innerHTML = `
            <p>This post is marked as <span>NSFW</span>, which means that it contains content targetet at mature audiences.</p>

            <button>Show anyways</button>
        `

        body.querySelector("span").style.color = "var(--red)";

        let button = body.querySelector("button");

        button.style.backgroundColor = "var(--red-25)";
        
        button.addEventListener("click", async () => {
            document.querySelector(`#post-${post.id}`).replaceWith(await makePostNode(data, showDeleteButton, true))
        });
    }

    let ratingDisplay = node.querySelector(".rating");
    let ratingDisplayNumber = ratingDisplay.querySelector("h5");

    ratingDisplayNumber.textContent = post.rating;
    
    if (post.rating < 0) ratingDisplayNumber.style.color = "var(--red-50)";
    
    let [upvoteButton, downvoteButton] = ratingDisplay.querySelectorAll("button");
    
    const ratingInteractionGenerator = (targetValue) => {
        return async (event) => {
            let payload = `{
                "post_id": "${post.id}",
                "new_rating": "${targetValue}"
            }`;

            let response = await fetch("/api/set_rating_state", {
                method: "POST",
                body: payload,
                headers: {
                    "Content-Type": "application/json"
                }
            });

            if (!response.ok) {
                alert(await response.text());
                return;
            }
    
            let refreshedPost = await response.json();
    
            document.querySelector(`#post-${post.id}`).replaceWith(await makePostNode(refreshedPost, showDeleteButton));
        }
    };
    
    if (interaction.rating == "Upvote") {
        const path = upvoteButton.querySelector("path");
        path.setAttribute("fill", "var(--green)");
        path.setAttribute("stroke-opacity", "1");

        upvoteButton.addEventListener("click", ratingInteractionGenerator("None"));
    } else {
        upvoteButton.addEventListener("click", ratingInteractionGenerator("Upvote"));
    }
    if (interaction.rating == "Downvote") {
        const path = downvoteButton.querySelector("path");
        path.setAttribute("fill", "var(--red)");
        path.setAttribute("stroke-opacity", "1");
        downvoteButton.addEventListener("click", ratingInteractionGenerator("None"));
    } else {
        downvoteButton.addEventListener("click", ratingInteractionGenerator("Downvote"));
    }

    let commentButton = node.querySelector(".comment");
    commentButton.querySelector("h5").textContent = post.comments;
    commentButton.addEventListener("click", (event) => {
        window.location.href = `${window.location.origin}?view=post&id=${post.id}`;
    });

    let shareButton = node.querySelector(".share");
    shareButton.querySelector("h5").textContent = post.shares;
    shareButton.addEventListener("click", () => {
        navigator.share({
            title: `Post by ${post.creator}`,
            url: `${window.location.origin}?view=post&id=${post.id}`
        }).then(async () => {
            let response = await fetch(`/api/register_post_share/${post.id}`, {method: "POST"});

            if (!response.ok) {
                response.text().then(alert);
                return;
            }

            let refreshedPost = await response.json();

            document.querySelector(`#post-${post.id}`).replaceWith(await makePostNode(refreshedPost, showDeleteButton));
        });
    });

    let repostButton = node.querySelector(".repost");
    repostButton.querySelector("h5").textContent = post.reposts;
    repostButton.addEventListener("click", () => {
        console.log(post.id);
        showRepostCreationScreen(post.id);
    });

    let bookmarkButton = node.querySelector(".bookmark");
    bookmarkButton.querySelector("h5").textContent = post.bookmarks;
    if (interaction.bookmarked) {
        bookmarkButton.querySelector("h5").style.color = "var(--green)";
        bookmarkButton.querySelector("path").style.stroke = "var(--green)";

        bookmarkButton.addEventListener("click", async () => {
            let response = await fetch(`/api/unbookmark_post/${post.id}`, {method: "POST"});

            if (!response.ok) {
                response.text().then(alert);
                return;
            }

            let refreshedPost = await response.json();
            document.querySelector(`#post-${post.id}`).replaceWith(await makePostNode(refreshedPost, showDeleteButton));
        });
    } else {
        bookmarkButton.addEventListener("click", async () => {
            let response = await fetch(`/api/bookmark_post/${post.id}`, {method: "POST"});

            if (!response.ok) {
                response.text().then(alert);
                return;
            }

            let refreshedPost = await response.json();
            document.querySelector(`#post-${post.id}`).replaceWith(await makePostNode(refreshedPost, showDeleteButton));
        });
    }

    let deleteButton = node.querySelector(".delete");
    if (post.creator !== window.localStorage.getItem("currentUsername") && !showDeleteButton) deleteButton.style.display = "none";
    else {
        let deleteHandler = async () => {
            let response = await fetch(`/api/delete_post/${post.id}`, {
                method: "DELETE"
            });
            if (response.ok) window.location.href = window.location;
            else alert(await response.text());
        };

        deleteButton.addEventListener("click", () => {showModal({
            title: "Delete Post?",
            body: "Deleting a post is irreversible. Do you wish to proceed?",
            inputFields: [],
            choices: [
                {
                    label: "Delete",
                    class: "bad",
                    onclick: deleteHandler
                },
                {
                    label: "Cancel"
                }
            ]
        })});
    }
}

async function makeChildPostNode(post) {
    let node = postTemplate.content.cloneNode(true);

    let userData = await UserDataStore.get(post.creator);

    let creatorDisplay = node.querySelector(".userDisplay");
    creatorDisplay.setAttribute("href", `?view=profile&id=${post.creator}`);
    creatorDisplay.querySelector("h4").textContent = userData.public_name;
    creatorDisplay.querySelector("h5").textContent = post.creator;
    creatorDisplay.querySelector(".profilePicture").style.backgroundImage = `url("/storage/profile_picture/${post.creator}")`;

    let threadInfo = node.querySelector(".threadInfoContainer");
    if (!post.parent) threadInfo.style.display = "none";
    else {
        threadInfo.setAttribute("href", `?view=post&id=${post.parent}`);
    }

    let timestamp = new Date(post.timestamp * 1000);
    let [date, fullTime] = timestamp.toISOString().split("T");
    let [hour, minute] = fullTime.split(":");

    let timestampDisplay = node.querySelector(".timestamp");
    let [dateDisplay, timeDisplay] = timestampDisplay.querySelectorAll("h5");
    dateDisplay.textContent = date;
    timeDisplay.textContent = `${hour}:${minute}`;

    if (!post.is_nsfw) {
        if (post.appendage_id) {
            let appendageResponse = await fetch(`/storage/appendage/${post.appendage_id}`);
            if (appendageResponse.ok) {
                let appendage = await appendageResponse.json();
        
                const mediaContainer = node.querySelector(".appendages");
                
                appendage.files.forEach((file) => {
                    let mediaType = mediaTypeLookup[file.file_type];
                    let mediaNode = document.createElement(mediaType);
                    mediaNode.setAttribute("src", `/storage/appendage/file/${file.file_id}`);
                    if (mediaType == "video") mediaNode.setAttribute("controls", "");
                    mediaContainer.appendChild(mediaNode);
                })
            }
        }
    
        node.querySelector(".content").textContent = post.body;
    } else {
        let content = node.querySelector(".content");
        
        content.innerHTML = "This post is marked as <span>NSFW</span>";
        content.querySelector("span").style.color = "var(--red)";
    }

    node.querySelector(".interactionBar").remove();
    node.querySelector(".referencedPostContainer").remove();

    return node;
}

function generateRatingEventHandler(post) {
    return (targetValue) => {
        return async (event) => {
            let payload = {
                "post_id": post.id,
                "new_rating": targetValue,
            };

            let response = await baseErrorHandler.guard(fetch("/api/set_rating_state", { 
                method: "POST",
                headers: {
                    "Content-Type": "application/json"
                },
                body: JSON.stringify(payload)
            }));
    
            let refreshedPost = await response.json();
    
            document.querySelector(`#post-${post.id}`).replaceWith(await makePostNode(refreshedPost));
        }
    }
}