function initFollowedFeedPaginator(handler) {
    
    var counter = 0;
    
    return async () => {
        let response = await baseErrorHandler.guard(fetch(`/api/fetch_followed_feed?page=${counter++}`));

        response.json().then(handler);
    }
}