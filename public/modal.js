const modalWindow = document.querySelector("#modal");

modalWindow.style.display = "none";

function showModal(options = {
    title: "",
    body: "",
    inputFields: [{
        name: "",
        placeholder: "",
        minLength: 1,
        maxLength: 10,
    }],
    choices: [{
        label: "",
        class: "",
        onclick: () => {},
    }]
}) {
    modalWindow.querySelector("h1").textContent = options.title;

    modalWindow.querySelector("p").textContent = options.body;


    const inputFieldContainer = modalWindow.querySelector(".inputFields");
    inputFieldContainer.innerHTML = "";
    var inputData = {};

    options.inputFields.forEach((inputField) => {
        let input = document.createElement("input");
        input.setAttribute("type", "text");
        input.setAttribute("name", inputField.name);
        input.setAttribute("placeholder", inputField.placeholder);
        input.setAttribute("minlength", inputField.minLength);
        input.setAttribute("maxlength", inputField.maxLength);
        input.addEventListener("input", () => {
            inputData[inputField.name] = input.value;
        });
        inputFieldContainer.appendChild(input);
    });

    const buttonContainer = modalWindow.querySelector(".buttonContainer");

    buttonContainer.innerHTML = "";

    options.choices.forEach(choice => {
        let button = document.createElement("button");
        button.setAttribute("class", choice.class);
        button.innerText = choice.label;
        button.addEventListener("click", (event) => {
            modalWindow.style.display = "none";
            choice.onclick(inputData);
        })
        buttonContainer.appendChild(button);
    });

    modalWindow.style.display = "grid";
}