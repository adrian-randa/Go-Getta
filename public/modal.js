const modalWindow = document.querySelector("#modal");

modalWindow.style.display = "none";

function showModal(options = {
    title: "",
    body: "",
    choices: [{
        label: "",
        class: "",
        onclick: () => {},
    }]
}) {
    modalWindow.querySelector("h1").textContent = options.title;

    modalWindow.querySelector("p").textContent = options.body;

    const buttonContainer = modalWindow.querySelector(".buttonContainer");

    buttonContainer.innerHTML = "";

    options.choices.forEach(choice => {
        let button = document.createElement("button");
        button.setAttribute("class", choice.class);
        button.innerText = choice.label;
        button.addEventListener("click", (event) => {
            modalWindow.style.display = "none";
            choice.onclick();
        })
        buttonContainer.appendChild(button);
    });

    modalWindow.style.display = "grid";
}