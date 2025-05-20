function initUsersPostsPaginator(username, handler) {
    var pageCounter = 0;

    return async () => {
        let response = await baseErrorHandler.guard(fetch(`/api/users_posts/${username}?page=${pageCounter++}`));

        response.json().then(handler);
    }
}