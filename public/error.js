const errorTemplate = document.querySelector("#errorTemplate");

const errorContainer = document.querySelector(".errorContainer");

class ErrorHandler {

    static mountErrorPopup(data) {
        let node = errorTemplate.content.cloneNode(true);

        let errorObject = node.querySelector(".error");
        node.querySelector("h4").textContent = data.title;
        node.querySelector("p").textContent = data.body;

        errorContainer.appendChild(node);

        setTimeout(() => {
            errorObject.remove();
        }, 5000);
    }

    constructor() {
        this.lut = new Map();

        return this;
    }

    map(error_id, title, body) {
        this.lut.set(error_id, {title: title, body: body});

        return this;
    }

    handle(response_string) {
        let id = response_string.split(" ")[2];

        let data = this.lut.get(id);

        if (!data) console.error("Unknown error: " + id);

        ErrorHandler.mountErrorPopup(data || {title: "Unknown Error", body: "An unknown error occured..."});
    }

    guard(fetch_promise) {
        return new Promise(async (resolve, reject) => {
            let response = await fetch_promise;
    
            if (!response.ok) {
                this.handle(await response.text());
                reject();
            }

            resolve(response);
        })
    }

}


const loginErrorHandler = new ErrorHandler()
    .map("InvalidPasswordError", "Invalid Password", "The password you entered was incorrect. Try again.")
    ;

const createAccountErrorHandler = new ErrorHandler()
    .map("InvalidKeyError", "Invalid Key", "The account creation key you tried to use was invalid. If you believe this key should be valid, please contact the administrator.")
    .map("UserAlreadyExistsError", "User Already Exists", "This username is already taken. Please use another.")
    ;