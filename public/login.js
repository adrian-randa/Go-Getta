let [landingForm, loginForm, createAccountForm] = [
    document.querySelector("form#landingForm"),
    document.querySelector("form#loginForm"),
    document.querySelector("form#createAccountForm"),
];

function showCreateAccountForm() {
    landingForm.style.display = "none";
    loginForm.style.display = "none";
    createAccountForm.style.display = "flex";
}

function showLoginForm() {
    landingForm.style.display = "none";
    loginForm.style.display = "flex";
    createAccountForm.style.display = "none";
}

let [landingButtonCreateAccount, landingButtonLogin] = landingForm.querySelectorAll("button");
landingButtonCreateAccount.addEventListener("click", (event) => {
    event.preventDefault();
    showCreateAccountForm();
});
landingButtonLogin.addEventListener("click", (event) => {
    event.preventDefault();
    showLoginForm();
});
landingForm.addEventListener("submit", (event) => {event.preventDefault();});


let [, loginCreateAccountInsteadButton] = loginForm.querySelectorAll("button");
loginCreateAccountInsteadButton.addEventListener("click", (event) => {
    event.preventDefault();
    showCreateAccountForm();
});


let [, createAccountLoginInsteadButton] = createAccountForm.querySelectorAll("button");
createAccountLoginInsteadButton.addEventListener("click", (event) => {
    event.preventDefault();
    showLoginForm();
});

let createAccountPasswordInput = document.querySelector("input#accountCreationPassword");
let createAccountPasswordRepeatInput = document.querySelector("input#accountCreationRepeatPassword");

createAccountPasswordRepeatInput.addEventListener("input", (event) => {
    event.target.setCustomValidity(
        event.target.value == createAccountPasswordInput.value ? "" : "Does not match first password!"
    )
    event.target.reportValidity();
})

loginForm.addEventListener("submit", async (event) => {
    event.preventDefault();
    event.stopPropagation();

    let payload = Object.fromEntries(new FormData(event.target));
    payload.expires = !payload.expires;

    let response = await loginErrorHandler.guard(fetch("/login", {
        headers: {
            "Content-Type": "application/json",
        },
        method: "POST",
        body: JSON.stringify(payload),
    }));

    const res = await response.json();

    document.cookie = "session_id=" + res.session_id;
    window.location.reload();
});

createAccountForm.addEventListener("submit", async (event) => {
    event.preventDefault();
    event.stopPropagation();

    let payload = Object.fromEntries(new FormData(event.target));

    let response = await createAccountErrorHandler.guard(fetch("/create_account", {
        headers: {
            "Content-Type": "application/json",
        },
        method: "POST",
        body: JSON.stringify(payload),
    }));

    const res = await response.json();

    document.cookie = "session_id=" + res.session_id;
    window.location.reload();
});