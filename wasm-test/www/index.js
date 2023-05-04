import { Universe, Cell } from "wasm-game-of-life";

//wasm_bindgenによって生成されるwasm線形メモリ空間への橋渡しをするオブジェクト
import { memory } from "wasm-game-of-life/wasm_game_of_life_bg"

const CELL_SIZE = 3; // px
const GRID_COLOR = "#CCCCCC";
const DEAD_COLOR = "#FFFFFF";
const ALIVE_COLOR = "#000000";

const universe = Universe.new();
const width = universe.width();
const height = universe.height();

const canvas = document.querySelector("canvas");
canvas.height = (CELL_SIZE + 1) * height + 1;
canvas.width = (CELL_SIZE + 1) * width + 1;


const ctx = canvas.getContext('2d');

let cellMem = new Array(Uint8Array.length);

const initCells = () => {
    const cellsPtr = universe.cells();
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    for (let i = 0; i < cells.length; i++) {
        cellMem[i] = cells[i];

    }


}

const drawGrid = () => {
    ctx.beginPath();
    ctx.strokeStyle = GRID_COLOR;

    // Vertical lines.
    for (let i = 0; i <= width; i++) {
        ctx.moveTo(i * (CELL_SIZE + 1) + 1, 0);
        ctx.lineTo(i * (CELL_SIZE + 1) + 1, (CELL_SIZE + 1) * height + 1);
    }

    // Horizontal lines.
    for (let i = 0; i <= height; i++) {
        ctx.moveTo(0, i * (CELL_SIZE + 1) + 1);
        ctx.lineTo((CELL_SIZE + 1) * width + 1, i * (CELL_SIZE + 1) + 1);
    }

    ctx.stroke();
};

const getIndex = (row, column) => {
    return row * width + column;
}

const drawCells = () => {
    //cellsの先頭を示すポインタを得る
    const cellsPtr = universe.cells();

    //cellsの状態を示す配列を得る
    //ここちょっと複雑で、やってることはwasmのメモリ空間から指定したアドレス上の生のバイナリ列を取ってきてUint8として認識できるような配列に格納……って感じ
    //memory.bufferはjs側からwasmメモリ空間にアクセスするための窓口 メモリ空間すべてを表す
    //で、そこからcellsPtrアドレスから(width * height)長のバイナリを取得して配列に格納し、uint8型の配列だよって宣言するという流れ
    const cells = new Uint8Array(memory.buffer, cellsPtr, width * height);

    ctx.beginPath();


    //生きてるセルを描画
    ctx.fillStyle = ALIVE_COLOR;
    for (let row = 0; row < height; row++) {

        for (let col = 0; col < width; col++) {
            const index = getIndex(row, col);
            if (cells[index] !== Cell.Alive) {
                continue;
            }

            //矩形塗りつぶし 引数はx座標、y座標,xサイズ,yサイズ
            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE);
        }
    }

}

const drawCellsDelta = () => {

    const deltaPtr = universe.delta();
    const delta = new Uint8Array(memory.buffer, deltaPtr, width * height);


    ctx.beginPath();
    for (let row = 0; row < height; row++) {

        for (let col = 0; col < width; col++) {
            const index = getIndex(row, col);

            if (delta[index] !== Cell.Alive) {
                continue;
            }

            if (cellMem[index] === Cell.Alive) {
                ctx.fillStyle = DEAD_COLOR;
                cellMem[index] = Cell.Dead;

            } else {

                ctx.fillStyle = ALIVE_COLOR;
                cellMem[index] = Cell.Alive;
            }

            //矩形塗りつぶし 引数はx座標、y座標,xサイズ,yサイズ
            ctx.fillRect(
                col * (CELL_SIZE + 1) + 1,
                row * (CELL_SIZE + 1) + 1,
                CELL_SIZE,
                CELL_SIZE);
        }
    }


}

const drawBG = () => {
    const boundingRect = canvas.getBoundingClientRect();

    ctx.beginPath();
    ctx.fillStyle = DEAD_COLOR;
    ctx.fillRect(
        0,
        0,
        boundingRect.width,
        boundingRect.height
    )
}

const playPauseButton = document.getElementById("play-pause");

const play = () => {
    playPauseButton.textContent = "⏸";

    renderLoop();
}

const pause = () => {
    playPauseButton.textContent = "▶";

    //これでキューイングされてる次フレームをキャンセルするっぽい
    cancelAnimationFrame(animationId);
    animationId = null;
}

playPauseButton.addEventListener("click", e => {
    if (isPaused()) {
        play();
    } else {
        pause();
    }
})

//再生中はrequestAnimationFrameが入り続け、一時停止中はnullになるような識別子
let animationId = null;

const resetButton = document.getElementById("reset");
resetButton.addEventListener("click", e => {
    universe.reset();
    initCells();
    drawGrid();

})

const renderLoop = () => {

    //debuggerを有効にするとブラウザの実行中にブレークポイントを設定できる　便利！
    //debugger;

    fps.render();

    universe.tick();

    //drawBG();
    drawGrid();
    //drawCells();
    drawCellsDelta();





    //animationIdに渡しつつ次のフレームを実行
    animationId = requestAnimationFrame(renderLoop);
};

//現在一時停止中か否かを返す
const isPaused = () => {
    return animationId === null;
}

//canvasがクリックされたときにセルを取得してそのセルの生存状況を判定させる
canvas.addEventListener("click", e => {

    //canvasが存在する矩形領域を取得する
    const boundingRect = canvas.getBoundingClientRect();

    //縮小や拡大などされているかもしれないので相対スケールを確保しておく
    const scaleX = canvas.width / boundingRect.width;
    const scaleY = canvas.height / boundingRect.height;

    //クリックされた絶対位置から矩形領域の位置を引いてローカル座標を取得する また、相対スケールも掛けておく
    const localX = (e.clientX - boundingRect.left) * scaleX;
    const localY = (e.clientY - boundingRect.top) * scaleY;

    //得られたローカル座標からセル番地を割り出す
    const row = Math.min(Math.floor(localY / (CELL_SIZE + 1)), height - 1);
    const col = Math.min(Math.floor(localX / (CELL_SIZE + 1)), width - 1);

    universe.toggle_cell(row, col);

    drawBG();
    drawGrid();
    drawCells();

})

//fpsカウンター
const fps = new class {
    constructor() {
        this.fps = document.getElementById("fps-counter");
        this.frames = [];
        this.lastFrameTimeStamp = performance.now();
    }

    render() {

        //前フレームの実行タイミングとの差分を取る
        const now = performance.now();
        const delta = now - this.lastFrameTimeStamp; //フレームあたりの消費ミリ秒
        this.lastFrameTimeStamp = now;
        const fps = 1000 / delta; //1000ミリ秒中にフレーム時間は何回分あるか？

        //直近100フレームのログを残す
        this.frames.push(fps);
        if (this.frames.length > 100) {
            this.frames.shift();
        }

        let min = Infinity;
        let max = -Infinity;
        let sum = 0;

        for (let i = 0; i < this.frames.length; i++) {
            sum += this.frames[i];
            min = Math.min(min, this.frames[i]);
            max = Math.max(max, this.frames[i]);
        }

        let mean = sum / this.frames.length;
        this.fps.textContent = `
		Frames per Second:
		         latest = ${Math.round(fps)}
		avg of last 100 = ${Math.round(fps)}
		min of last 100 = ${Math.round(min)}
		max of last 100 = ${Math.round(max)}
		`.trim();

    }
};

initCells();
play();