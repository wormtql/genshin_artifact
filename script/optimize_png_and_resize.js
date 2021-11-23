const path = require("path")
const fs = require("fs")

const sharp = require("sharp")

const imagemin = require("imagemin")
const imageminJpegtran = require("imagemin-jpegtran")
const imageminPngquant = require("imagemin-pngquant")

const artifactSetName = process.argv[2]
const size = parseInt(process.argv[3])

const artifactsPath = path.normalize(path.join(__dirname, "../src/assets/artifacts/data", artifactSetName))
if (!fs.existsSync(artifactsPath)) {
    throw new Error(`artifact set name "${artifactSetName} does not exist"`)
}

function getImages(p) {
    return fs.readdirSync(p,{
        withFileTypes: true
    }).filter(dirent => {
        const f1 = /\.(png|jpg)$/.test(dirent.name)
        const f2 = f1 && dirent.name.indexOf("bak") === -1
        return f2
    }).map(dirent => dirent.name)
}

function backupImages(images) {
    let newNames = []
    for (let image of images) {
        const temp = image.split(".", 2)
        const newName = `${temp[0]}.bak.${temp[1]}`
        newNames.push(newName)

        fs.renameSync(image, newName)
    }
    
    return newNames
}

function deleteImages(images) {
    for (let image of images) {
        fs.rmSync(image)
    }
}

async function resize(file, size, output) {
    const image = sharp(file)
    const image2 = await image.metadata()
        .then(metadata => {
            const height = metadata.height;
            const width = metadata.width;
            if (height > size || width > size) {
                throw new Error("cannot padding to smaller size")
            }

            const excessX = size - width;
            const excessY = size - height;

            const top = Math.round(excessY / 2);
            const bottom = excessY - top;
            const left = Math.round(excessX / 2);
            const right = excessX - left;

            return image
                .extend({
                    top, bottom, left, right,
                    background: { r: 0, g: 0, b: 0, alpha: 0 }
                })
        })
    await image2.toFile(output)
        // .catch(err => {
        //     throw new Error(err)
        // })
}

async function run() {
    const images = getImages(artifactsPath)
        .map(name => path.join(artifactsPath, name))
        .map(name => name.replace(/\\/g, "/"))
    const outputDir = artifactsPath.replace(/\\/g, "/")
    console.log(outputDir)

    const backedImages = backupImages(images)

    for (let i = 0; i < images.length; i++) {
        const imageName = images[i];
        const backImageName = backedImages[i];
        await resize(backImageName, size, imageName);
    }
    // console.log(images);
    // await new Promise(resolve => setTimeout(resolve, 5000));
    

    const files = await imagemin(images, {
        destination: outputDir,
        plugins: [
            imageminJpegtran(),
            imageminPngquant({
                quality: [0.2, 0.2]
            })
        ]
    }).catch(e => {
        console.log(e)
    })

    console.log(`optimized ${files.length} image(s)`)

    deleteImages(backedImages)
}

run();