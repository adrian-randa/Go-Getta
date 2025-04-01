function initPublicSpacePaginator(handler) {
    var pageCounter = 0;

    return () => {
        let req = new XMLHttpRequest();

        req.open("GET", `/api/fetch_public_space?page=${pageCounter++}`);

        req.setRequestHeader("Content-Type", "application/json; charset=UTF-8");

        req.addEventListener("error", (event) => {
            event.preventDefault();

            alert(req.responseText);
        })

        req.onreadystatechange = () => {
            if (req.readyState === XMLHttpRequest.DONE) {
                if (req.status === 200) {
                    handler(JSON.parse(req.responseText));
                } else {
                    alert(req.responseText);
                }
            }
        };

        req.send();
    }
}