const displayedImage = document.querySelector('.displayed-img');
const thumbBar = document.querySelector('.thumb-bar');

const btn = document.querySelector('button');
const overlay = document.querySelector('.overlay');

/* Declaring the array of image filenames */

/* Declaring the alternative text for each image file */

/* Looping through images */
for (let i = 0; i < 5; i++) {

    const picString = "images/pic" + (i + 1) + ".jpg";

    const newImage = document.createElement('img');
    newImage.setAttribute('src', picString);
    newImage.setAttribute('alt', picString);
    thumbBar.appendChild(newImage);

    newImage.addEventListener("click", e => { displayedImage.src = e.target.src })

}
/* Wiring up the Darken/Lighten button */

btn.addEventListener("click", modeChange)

function modeChange() {
    let mode = btn.getAttribute("class");
    if (mode === "dark") {
        btn.setAttribute("class", "light");
        btn.textContent = "Lighten";
        overlay.style.backgroundColor = "rgba(0,0,0,0.5)";
    } else if (mode === "light") {
        btn.setAttribute("class", "dark");
        btn.textContent = "Darken";
        overlay.style.backgroundColor = "rgba(0,0,0,0)";
    }

}