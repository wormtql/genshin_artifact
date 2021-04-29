
const { readdirSync, writeFile } = require("fs");
const path = require("path");

const imagemin = require("imagemin");
const imageminJpegtran = require("imagemin-jpegtran");
const imageminPngquant = require("imagemin-pngquant");

function getDir(source) {
    return readdirSync(source, {
        withFileTypes: true
    }).filter(dirent => dirent.isDirectory())
    .map(dirent => dirent.name);
}

function getImages(source) {
    return readdirSync(source, {
        withFileTypes: true
    }).filter(dirent => /\.(png|jpg)$/.test(dirent.name))
    .map(dirent => dirent.name);
}

const artifactsPath = path.normalize(path.join(__dirname, "../src/assets/artifacts/data"));
const artifacts = getDir(artifactsPath);

async function func() {
    for (let i = 0; i < artifacts.length; i++) {
        let name = artifacts[i];
        // if (name !== "tenacityOfTheMillelith") {
        //     continue;
        // }
    
        let p = path.join(artifactsPath, name);
        let images = getImages(p)
            .map(item => path.join(p, item))
            // .map(item => path.relative(__dirname, item))
            .map(item => item.replace(/\\/g, "/"))
            ;
        
            let outputDir = p.replace(/\\/g, "/");
    
        const files = await imagemin(images, {
            destination: outputDir,
            plugins: [
                imageminJpegtran(),
                imageminPngquant({
                    quality: [0.2, 0.2]
                })
            ]
        })
        console.log(`optimized ${files.length} image(s)`);
    }
}
func();