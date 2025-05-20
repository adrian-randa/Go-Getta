function initPublicSpacePaginator(handler) {
    var pageCounter = 0;

    return async () => {
        //console.log(`Fetching public_space page ${pageCounter}`);

        let response = await baseErrorHandler.guard(fetch(`/api/fetch_public_space?page=${pageCounter++}`, {
            headers: {
                "Content-Type": "application/json"
            }
        }));

        response.json().then(handler);
    }
}