const axios = require("axios");
const { createWriteStream } = require("fs");
const path = require("path");

const { parse } = require("node-html-parser");

let artName = process.argv[2];
let url = "https://genshin-impact.fandom.com/wiki/" + artName;

const positions = ["flower", "feather", "sand", "cup", "head"];

axios.get(url).then(response => {
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
        }, {
            headers: {
                "user-agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/92.0.4515.107 Safari/537.36",
            }
        }).then(response => {
            const writer = createWriteStream(path.join(__dirname, "temp", positions[i] + ".png"));
            response.data.pipe(writer);
        }).catch(err => {
            console.log(err.message);
        });
    }
}).catch(err => {
    console.log(err);
});