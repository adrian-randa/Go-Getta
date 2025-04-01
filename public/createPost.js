const CHARACTER_LIMIT = 250;

function submitPost() {
    const body = document.querySelector("#newPostBody").value;
    if (body.length > CHARACTER_LIMIT) {
        return;
    }

    let payload = {
        "body": body,
        "appendage_id": null,
        "room": null,
        "parent": null,
    }

    const req = new XMLHttpRequest();

    req.open("POST", "/api/create_post");

    req.setRequestHeader("Content-Type", "application/json; charset=UTF-8");

    req.addEventListener("error", (event) => {
        event.preventDefault();

        alert(req.responseText);
    })

    req.onreadystatechange = () => {
        if (req.readyState === XMLHttpRequest.DONE) {
            if (req.status === 200) {
                //TODO: Change this to show the newly created post
                showPublicSpaceScreen();
            } else {
                alert(req.responseText);
            }
        }
      };

    req.send(JSON.stringify(payload));
}