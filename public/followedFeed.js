function initFollowedFeedPaginator(handler) {
    
    var counter = 0;
    
    return async () => {
        let response = await fetch(`/api/fetch_followed_feed?page=${counter++}`);

        if (!response.ok) {
            response.text().then(alert);
            return;
        }

        response.json().then(handler);
    }
}