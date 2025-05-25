let button = document.querySelector("button")
let p = document.querySelector("p")

let text = "Sonic Spee"

button.addEventListener("click", () =>{
    text += "e"
    p.innerHTML = text+"d"
})