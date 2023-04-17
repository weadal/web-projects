const myImage = document.querySelector("img");

myImage.onclick = () => {
    const mySrc = myImage.getAttribute("src");
    if (mySrc === "images/img-html-1.png") {
        myImage.setAttribute("src", "images/img-html-0.png");
    } else {
        myImage.setAttribute("src", "images/img-html-1.png");
    }
}

let myButton = document.querySelector("button");
let myButton1 = document.getElementById("button1");
let myHeading = document.querySelector("h1");

if (!localStorage.getItem("name")) {
    setUserName();
} else {
    const storedName = localStorage.getItem("name");
    myHeading.textContent = `ぷにぷに選手権への準備ができたようだな、${storedName}`;
}

myButton.onclick = () => {
    setUserName();
}

function setUserName() {
    const myName = prompt("あなたの名前は何ですか");

    if (!myName) {
        setUserName();
    } else {

        localStorage.setItem("name", myName);
        myHeading.textContent = `ぷにぷに選手権への準備ができたようだな、${myName}`;
    }
}

function createParagraph() {
    const para = document.createElement("p");
    para.textContent = "ボタンが押されました";
    document.body.appendChild(para);
}

const buttons = document.querySelectorAll("button");

for (const button of buttons) {
    button.addEventListener("click", createParagraph);
}

myButton1.onclick = () => {
    createParagraph();
}