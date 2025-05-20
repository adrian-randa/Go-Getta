const COMMENT_CHARACTER_LIMIT = 250;

async function submitComment() {
    const body = document.querySelector("#newCommentBody").value;
    if (body.length > POST_CHARACTER_LIMIT) {
        return;
    }

    const parentID = new URL(window.location.href).searchParams.get("id");
    if (!parentID) return;

    let appendageID = null;

    const fileInput = document.querySelector("#newCommentFiles");
    if (fileInput.files && fileInput.files.length > 0) {

        let fileUploadFormData = new FormData();

        Array.from(fileInput.files).forEach((file, i) => {
            fileUploadFormData.append(`media_${i}`, file);
        });

        const response = await fileUploadErrorHandler.guard(fetch("/api/file_upload", {method: "POST", body: fileUploadFormData}));

        let responseObj = await response.json();
        appendageID = responseObj.appendage_id;
    }

    let payload = {
        "body": body,
        "appendage_id": appendageID,
        "room": null,
        "parent": parentID,
        "child": null
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

function browseNewCommentFiles() {
    document.querySelector("#newCommentFiles").click();
}

function handleNewCommentFileSelect(input) {
    const previewContainer = document.querySelector("#commentMediaPreviewContainer");
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

function removeNewCommentFiles() {
    document.querySelector("#newCommentFiles").value = "";
    document.querySelector("#commentMediaPreviewContainer").innerHTML = "";
}

function initCommentPaginator(parentID, handler) {
    var pageCounter = 0;

    return async () => {
        let response = await baseErrorHandler.guard(fetch(`/api/fetch_comments/${parentID}?page=${pageCounter++}`, {
            headers: {
                "Content-Type": "application/json"
            }
        }));

        response.json().then(handler);
    }
}