const postTemplate = document.querySelector("#postTemplate");
const mountPosts = (screen) => {
    return async (posts) => {
        console.log("Mounting posts", posts);
        for (let i = 0; i < posts.length; i++) {
            screen.appendChild(await makePostNode(posts[i]));
        }
    }
}

const mediaTypeLookup = {
    "Image": "img",
    "Video": "video",
    "Audio": "audio",
}

async function makePostNode(post) {
    let node = postTemplate.content.cloneNode(true);

    await applyPostDataToNode(post, node);

    return node;
}

async function applyPostDataToNode(data, node) {

    const { post, interaction } = data;

    if (node instanceof DocumentFragment) node.querySelector(".post").setAttribute("id", `post-${post.id}`);

    let userDataResponse = await fetch(`/api/get_user_data/${post.creator}`);
    let userData = await userDataResponse.json();

    let creatorDisplay = node.querySelector(".userDisplay");
    creatorDisplay.querySelector("h4").textContent = userData.public_name;
    creatorDisplay.querySelector("h5").textContent = post.creator;
    creatorDisplay.querySelector(".profilePicture").style.backgroundImage = `url("/storage/profile_picture/${post.creator}")`;

    let timestamp = new Date(post.timestamp * 1000);
    let [date, fullTime] = timestamp.toISOString().split("T");
    let [hour, minute] = fullTime.split(":");

    let timestampDisplay = node.querySelector(".timestamp");
    let [dateDisplay, timeDisplay] = timestampDisplay.querySelectorAll("h5");
    dateDisplay.textContent = date;
    timeDisplay.textContent = `${hour}:${minute}`;

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

    let ratingDisplay = node.querySelector(".rating");
    ratingDisplay.querySelector("h5").textContent = post.rating;
    let [upvoteButton, downvoteButton] = ratingDisplay.querySelectorAll("button");
    const ratingInteractionGenerator = generateRatingEventHandler(post, node);
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

    let repostButton = node.querySelector(".repost");
    repostButton.querySelector("h5").textContent = post.reposts;

    let bookmarkButton = node.querySelector(".bookmark");
    bookmarkButton.querySelector("h5").textContent = post.bookmarks;
}

function generateRatingEventHandler(post) {
    return (targetValue) => {
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
    
            //applyPostDataToNode(refreshedPost, document.querySelector(`#post-${post.id}`));
            document.querySelector(`#post-${post.id}`).replaceWith(await makePostNode(refreshedPost));
        }
    }
}