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

    clone() {
        return Object.assign(Object.create(Object.getPrototypeOf(this)), this);
    }

}

const baseErrorHandler = new ErrorHandler()
    .map("InternalServerError", "Internal Server Error", "The server ran into an error while processing your request. Please try again.")
    .map("InvalidSessionError", "Session Expired", "You have been inactive for too long. Please refresh the page to log back in.")
    .map("RoomDoesNotExistError", "Room Doesn't Exist", "This room does not exist.")
    .map("RoomBoundaryViolationError", "Room Boundary Violation", "You violated the bounds of a private room")
    ;

const loginErrorHandler = baseErrorHandler.clone()
    .map("InvalidPasswordError", "Invalid Password", "The password you entered was incorrect. Try again.")
    ;

const createAccountErrorHandler = baseErrorHandler.clone()
    .map("InvalidKeyError", "Invalid Key", "The account creation key you tried to use was invalid. If you believe this key should be valid, please contact the administrator.")
    .map("UserAlreadyExistsError", "User Already Exists", "This username is already taken. Please use another.")
    ;

const createPostErrorHandler = baseErrorHandler.clone()
    .map("EmptyContentError", "Empty Post", "Your post has no content.")
    .map("ContentTooLargeError", "Post Too Long", "Your post contains too many characters. Please be concise.")
    .map("PostDoesNotExistError", "Referenced Post Unavailable", "The post you were trying to reference does not exist. This can happen when the referenced post is deleted while you're commenting or reposting it.")
    .map("RoomDoesNotExistError", "Room Doesn't Exist", "The room you tried to post in doesn't exist.")
    .map("RoomBoundaryViolationError", "Room Boundary Violation", "You tried to reference a post outside of the private rooms' boundary.")
    ;

const fileUploadErrorHandler = baseErrorHandler.clone()
    .map("InvalidFileError", "Unsupported File Type", "The type of file you tried to upload is not supported.")
    .map("EmptyContentError", "Empty File", "The file(s) you uploaded are empty.")
    .map("RoomDoesNotExistError", "Room Doesn't Exist", "This room does not exist.")
    .map("InsufficientPermissionsError", "No Permission", "You do not have the permissions for this operation.")
    ;