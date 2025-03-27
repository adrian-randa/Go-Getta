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

loginForm.addEventListener("submit", (event) => {
    event.preventDefault();
    event.stopPropagation();

    const req = new XMLHttpRequest();

    req.open("POST", "/login");

    req.setRequestHeader("Content-Type", "application/json; charset=UTF-8");

    req.addEventListener("error", (event) => {
        event.preventDefault();

        alert(req.responseText);
    })

    req.onreadystatechange = () => {
        if (req.readyState === XMLHttpRequest.DONE) {
            if (req.status === 200) {
                const res = JSON.parse(req.responseText);
    
                document.cookie = "session_id=" + res.session_id;
                window.location.reload();
            } else {
                alert(req.responseText);
            }
        }
      };

    let payload = Object.fromEntries(new FormData(event.target));
    payload.expires = !payload.expires;
    req.send(JSON.stringify(payload));
});

createAccountForm.addEventListener("submit", (event) => {
    event.preventDefault();
    event.stopPropagation();

    const req = new XMLHttpRequest();

    req.open("POST", "/create_account");

    req.setRequestHeader("Content-Type", "application/json; charset=UTF-8");

    req.onreadystatechange = () => {
        if (req.readyState === XMLHttpRequest.DONE) {
            if (req.status === 200) {
                showLoginForm();
            } else {
                alert(req.responseText);
            }
        }
      };

    req.send(JSON.stringify(Object.fromEntries(new FormData(event.target))));
});