let randomNumber = Math.floor(Math.random() * 100) + 1;

//querySelectorに渡す引数は、ドットをつけるとクラス名として扱われ、ドットがないとhtmlタグ(<p>とか<a>とか)として扱われる
//この場合はhtmlでクラスとしてユーザーによって定義されているものをピンポイントで拾ってきたいので、ドットを付けたクラス名を渡して取得する
const guesses = document.querySelector(".guesses");
const lastResult = document.querySelector(".lastResult");
const lowOrHi = document.querySelector(".lowOrHi");

const guessSubmit = document.querySelector(".guessSubmit");
const guessField = document.querySelector(".guessField");

let guessCount = 1;
let resetButton;

guessField.focus();

function checkGuess() {

    //Numberは多分tryperseみたいなもんで、数値に変換できる文字列のみ数値に変換する的な処理
    let userGuess = Number(guessField.value);
    if (guessCount === 1) {
        guesses.textContent = "これまでの予想: ";
    }

    guesses.textContent += userGuess + " ";

    if (userGuess === randomNumber) {
        lastResult.textContent = "おめでとう！　正解です！";
        lastResult.style.backgroundColor = "green";
        lowOrHi.textContent = " ";
        setGameOver();

    } else if (guessCount === 10) {
        lastResult.textContent = "!!!ゲームオーバー!!!";
        setGameOver();
    } else {
        lastResult.textContent = "間違いです！";
        lastResult.style.backgroundColor = "red";

        if (userGuess < randomNumber) {
            lowOrHi.textContent = "小さすぎだよ";
        } else if (userGuess > randomNumber) {
            lowOrHi.textContent = "大きすぎるよ";

        }
    }

    guessCount++;
    guessField.value = "";
    guessField.focus();
}

//guessSubmit(=フォームのボタン)のクリックイベントを検知するリスナーを追加する
guessSubmit.addEventListener("click", checkGuess);

function setGameOver() {
    guessField.disabled = true;
    guessSubmit.disabled = true;

    resetButton = document.createElement("button");
    resetButton.textContent = "New Game";
    document.body.appendChild(resetButton);
    resetButton.addEventListener("click", resetGame);
}

function resetGame() {
    guessCount = 1;

    //ここでdivタグによってresultParasクラスにまとめられた段落をすべて取得する
    const resetParas = document.querySelectorAll(".resultParas p");

    for (let i = 0; i < resetParas.length; i++) {
        resetParas[i].textContent = "";
    }

    //リセットボタンの親を参照して子を破棄するremoveChildメソッドでresetButtonを渡して破棄
    resetButton.parentNode.removeChild(resetButton);

    guessField.disabled = false;
    guessSubmit.disabled = false;
    guessField.value = "";
    guessField.focus();

    lastResult.style.backgroundColor = "white";

    randomNumber = Math.floor(Math.random() * 100) + 1;

}