class UserDataStore {

    static cache = new Map();

    static async get(username) {
        let cached = this.cache.get(username);
        if (cached) return cached;

        let response = await fetch(`/api/get_user_data/${username}`);
        let userData = await response.json();

        this.cache.set(username, userData);

        return userData;
    }
}