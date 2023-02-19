let wrapper = document.querySelector('#tiles')

let columns = Math.floor(document.body.clientWidth / 50),
    rows = Math.floor(document.body.clientHeight / 50)

let toggled = false

const handleOnClick = (index) => {
    toggled = !toggled

    anime({
        targets: ".tile",
        opacity: toggled ? 0: 1,
        delay: anime.stagger(50, {
            grid: [columns, rows],
            from: index
        })
    })
}

const createTile = (index) => {
    const tile = document.createElement('div')

    tile.classList.add("tile")

    tile.style.opacity = toggled ? 0 : 1

    tile.onclick = (e) => handleOnClick(index)

    return tile
}

const createTiles = (quantity) => {
    Array.from(Array(quantity)).map((tile, index) => {
        wrapper.appendChild(createTile(index))
    })
}

const createGrid = () => {
    wrapper.innerHTML = "";
    columns = Math.floor(document.body.clientWidth / 50),
        rows = Math.floor(document.body.clientHeight / 50);

    wrapper.style.setProperty("--columns", columns);
    wrapper.style.setProperty("--rows", rows);

    createTiles(columns * rows);
};

createGrid();
window.onresize = () => createGrid();