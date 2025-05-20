const POST_CHARACTER_LIMIT = 300;

async function submitPost() {
    const bodyInput = document.querySelector("#newPostBody");
    const body = bodyInput.value;

    if (body.length > POST_CHARACTER_LIMIT) {
        return;
    }

    let appendageID = null;

    const fileInput = document.querySelector("#newPostFiles");
    if (fileInput.files && fileInput.files.length > 0) {

        let fileUploadFormData = new FormData();

        Array.from(fileInput.files).forEach((file, i) => {
            fileUploadFormData.append(`media_${i}`, file);
        });

        const response = await fileUploadErrorHandler.guard(fetch("/api/file_upload", {method: "POST", body: fileUploadFormData}));

        let responseObj = await response.json();
        appendageID = responseObj.appendage_id;
    }

    let roomID = document.querySelector("#newPostRoom").value || null;

    let payload = {
        "body": body,
        "appendage_id": appendageID,
        "room": roomID,
        "parent": null,
        "child": null,
    }

    let response = await createPostErrorHandler.guard(fetch("/api/create_post", {
        method: "POST",
        headers: {
            "Content-Type": "application/json"
        },
        body: JSON.stringify(payload)
    }));

    let { post_id } = await response.json();

    window.location.href = `${window.location.origin}?view=post&id=${post_id}`;
}

function browseNewPostFiles() {
    document.querySelector("#newPostFiles").click();
}

function handleNewPostFileSelect(input) {
    const previewContainer = document.querySelector("#mediaPreviewContainer");
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

function removeNewPostFiles() {
    document.querySelector("#newPostFiles").value = "";
    document.querySelector("#mediaPreviewContainer").innerHTML = "";
}