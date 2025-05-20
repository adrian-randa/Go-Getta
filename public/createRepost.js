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

        const response = await fileUploadErrorHandler.guard(fetch("/api/file_upload", {method: "POST", body: fileUploadFormData}));

        let responseObj = await response.json();
        appendageID = responseObj.appendage_id;
    }

    let roomID = document.querySelector("#newRepostRoom").value || null;

    let payload = {
        "body": body,
        "appendage_id": appendageID,
        "room": roomID,
        "parent": null,
        "child": referencedPostID,
    }

    let response = await createPostErrorHandler.guard(fetch("/api/create_post", {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
        body: JSON.stringify(payload)
    }));

    let { post_id } = await response.json();

    window.location.href = `${window.location.origin}?view=post&id=${post_id}`;
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