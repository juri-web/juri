window.onload = () => {
    let juriEl = document.querySelector("#juri");
    setInterval(() => {
        if (juriEl.innerHTML == "Hello") {
            juriEl.innerHTML = "你好";
        } else {
            juriEl.innerHTML = "Hello";
        }
    }, 2000);
};
