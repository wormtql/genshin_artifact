const axios = require("axios");
const { createWriteStream } = require("fs");
const path = require("path");

const { parse } = require("node-html-parser");

let artName = process.argv[2];
let url = "https://genshin-impact.fandom.com/wiki/" + artName;

const positions = ["flower", "feather", "sand", "cup", "head"];

axios.get(url).then(response => {
    console.log(response.data.slice(0, 200));

    const root = parse(response.data);
    const images = root.querySelectorAll("a.image");

    let imageUrls = images.map(item => item.getAttribute("href"));
    console.log(imageUrls);

    for (let i = 0; i < imageUrls.length; i++) {
        let imageUrl = imageUrls[i];
        axios({
            method: "GET",
            url: imageUrl,
            responseType: "stream",
        }).then(response => {
            const writer = createWriteStream(path.join(__dirname, "temp", positions[i] + ".png"));
            response.data.pipe(writer);
        });
    }
});