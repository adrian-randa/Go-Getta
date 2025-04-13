//const POST_CHARACTER_LIMIT = 300; // Set in createPost.js

async function submitRepost(referencedPostID) {
    const body = document.querySelector("#newRepostBody").value;
    if (body.length > POST_CHARACTER_LIMIT) {
        return;
    }

    let appendageID = null;

    const fileInput = document.querySelector("#newRepostFiles");
    if (fileInput.files && fileInput.files.length > 0) {

        let fileUploadFormData = new FormData();

        Array.from(fileInput.files).forEach((file, i) => {
            fileUploadFormData.append(`media_${i}`, file);
        });

        const response = await fetch("/api/file_upload", {method: "POST", body: fileUploadFormData});

        if (!response.ok) {
            alert(await response.text());
        }

        let responseObj = await response.json();
        appendageID = responseObj.appendage_id;
    }

    let payload = {
        "body": body,
        "appendage_id": appendageID,
        "room": null,
        "parent": null,
        "child": referencedPostID,
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
                let response = JSON.parse(req.responseText);
                window.location.href = `${window.location.origin}?view=post&id=${response.post_id}`;
            } else {
                alert(req.responseText);
            }
        }
      };

    req.send(JSON.stringify(payload));
}

function browseNewRepostFiles() {
    document.querySelector("#newRepostFiles").click();
}

function handleNewRepostFileSelect(input) {
    const previewContainer = document.querySelector("#repostMediaPreviewContainer");
    previewContainer.innerHTML = "";

    if (input.files) {
        Array.from(input.files).forEach((file, i) => {
            let reader = new FileReader();

            reader.onload = (event) => {
                let container = document.createElement("div");

                let img = document.createElement("img");
                img.setAttribute("src", event.target.result);
                img.setAttribute("alt", file.name);
                container.appendChild(img);

                previewContainer.appendChild(container);
            }

            reader.readAsDataURL(file);
        });
    }
}

function removeNewRepostFiles() {
    document.querySelector("#newRepostFiles").value = "";
    document.querySelector("#repostMediaPreviewContainer").innerHTML = "";
}