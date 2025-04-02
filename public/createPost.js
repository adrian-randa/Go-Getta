const CHARACTER_LIMIT = 250;

async function submitPost() {
    const body = document.querySelector("#newPostBody").value;
    if (body.length > CHARACTER_LIMIT) {
        return;
    }

    let appendageID = null;

    const fileInput = document.querySelector("#newPostFiles");
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

function browseFiles() {
    document.querySelector("#newPostFiles").click();
}

function handleFileSelect(input) {
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

function removeFiles() {
    document.querySelector("#newPostFiles").value = "";
    document.querySelector("#mediaPreviewContainer").innerHTML = "";
}