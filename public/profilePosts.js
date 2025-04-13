function initUsersPostsPaginator(username, handler) {
    var pageCounter = 0;

    return async () => {
        console.log(`Fetching users_posts page ${pageCounter}`);

        let response = await fetch(`/api/users_posts/${username}?page=${pageCounter++}`);

        if (response.ok) {
            handler(await response.json());
        } else {
            alert(await response.text());
        }
    }
}