// setup canvas

const canvas = document.querySelector('canvas');
const ctx = canvas.getContext('2d');

const width = canvas.width = window.innerWidth;
const height = canvas.height = window.innerHeight;

// function to generate random number

function random(min, max) {
    const num = Math.floor(Math.random() * (max - min + 1)) + min;
    return num;
}

// function to generate random color

function randomRGB() {
    return `rgb(${random(0, 255)},${random(0, 255)},${random(0, 255)})`;
}

class Shape {
    constructor(x, y, velX, velY, color, size) {
        this.x = x;
        this.y = y;
        this.velX = velX;
        this.velY = velY;
    }
}

class Ball extends Shape {

    constructor(x, y, velX, velY, color, size) {
        super(x, y, velX, velY);
        this.color = color;
        this.size = size;
        this.exists = true;
    }

    draw() {

        //パスによる描画を宣言
        ctx.beginPath();

        //先んじて塗りつぶしのスタイルを設定　ここでは自身のcolor
        ctx.fillStyle = this.color;

        //パスを設定　arc()で円弧を描画する　引数は(中心点x座標,中心点y座標、円弧の半径,始点角度(ラジアン),終点角度(ラジアン))
        ctx.arc(this.x, this.y, this.size, 0, 2 * Math.PI);

        //パス内部を塗りつぶし(先程設定したfillStyleがここで利用される)
        ctx.fill();
    }

    update() {

        if ((this.x + this.size) >= width) {
            this.velX = -(this.velX);
        }

        if ((this.x - this.size) <= 0) {
            this.velX = -(this.velX);
        }

        if ((this.y + this.size) >= height) {
            this.velY = -(this.velY);
        }

        if ((this.y - this.size) <= 0) {
            this.velY = -(this.velY);
        }

        this.x += this.velX;
        this.y += this.velY;
    }

    collisionDetect() {
        for (const ball of balls) {
            if (!(this === ball) && ball.exists) {
                const dx = this.x - ball.x;
                const dy = this.y - ball.y;
                const distance = Math.sqrt(dx * dx + dy * dy);

                if (distance < this.size + ball.size) {
                    ball.color = this.color = randomRGB();
                }
            }

        }
    }

}

class EvilCircle extends Shape {
    constructor(x, y) {
        super(x, y, 20, 20);
        this.color = "white";
        this.size = 10;

        //コンストラクタでキー入力を受け付けるイベントリスナーを作る
        window.addEventListener("keydown", (e) => {
            switch (e.key) {
                case "a":
                    this.x -= this.velX;
                    break;
                case "d":
                    this.x += this.velX;
                    break;
                case "w":
                    this.y -= this.velY;
                    break;
                case "s":
                    this.y += this.velY;
                    break;
            }
        })

    }

    draw() {

        //パスによる描画を宣言
        ctx.beginPath();

        //先んじてストロークのスタイルを設定　ここでは自身のcolor
        ctx.strokeStyle = this.color;
        ctx.lineWidth = 3;

        //パスを設定　arc()で円弧を描画する　引数は(中心点x座標,中心点y座標、円弧の半径,始点角度(ラジアン),終点角度(ラジアン))
        ctx.arc(this.x, this.y, this.size, 0, 2 * Math.PI);

        //パスにそってストローク描画(先程設定したstrokeStyleがここで利用される)
        ctx.stroke();
    }


    checkBounds() {

        if ((this.x + this.size) >= width) {
            this.x -= this.size;
        }

        if ((this.x - this.size) <= 0) {
            this.x += this.size;
        }

        if ((this.y + this.size) >= height) {
            this.y -= this.size;
        }

        if ((this.y - this.size) <= 0) {
            this.y += this.size;
        }

    }

    collisionDetect() {
        for (const ball of balls) {
            if (ball.exists) {
                const dx = this.x - ball.x;
                const dy = this.y - ball.y;
                const distance = Math.sqrt(dx * dx + dy * dy);

                if (distance < this.size + ball.size) {
                    ball.exists = false;
                    ballCount--;
                }
            }

        }
    }

}


const balls = [];
let ballCount = 0;
const para = document.querySelector("p");
const ballCountText = para.textContent;

while (balls.length < 25) {
    const size = random(10, 20);
    const ball = new Ball(
        random(0 + size, width - size),
        random(0 + size, height - size),
        random(-7, 7),
        random(-7, 7),
        randomRGB(),
        size
    );

    balls.push(ball);
    ballCount++;
}

const evilCircle = new EvilCircle(100, 100);

function loop() {
    ctx.fillStyle = "rgba(0,0,0,1)";
    ctx.fillRect(0, 0, width, height);

    for (const ball of balls) {
        if (ball.exists) {

            ball.draw();
            ball.update();
            ball.collisionDetect();
        }

        evilCircle.draw();
        evilCircle.checkBounds();
        evilCircle.collisionDetect();

    }

    para.textContent = ballCountText + ballCount;

    requestAnimationFrame(loop);
}



loop();