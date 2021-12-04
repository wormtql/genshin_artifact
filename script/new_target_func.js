const fs = require("fs")
const path = require("path")
const Mustache = require("mustache")

const fullName = process.argv[2]
const name = path.basename(fullName)
const dirName = path.dirname(fullName)

const folder = path.normalize(path.join(__dirname, "../src/assets/target_functions/data", dirName))

if (!fs.existsSync(folder)) {
    throw new Error(`path "${folder}" does not exist`)
}

templateFolder = path.normalize(path.join(__dirname, "./templates/target_function"))

// generate xxx.tf.func.js
const funcTemplate = fs.readFileSync(path.join(templateFolder, "target.func.mustache")).toString()
const funcRendered = Mustache.render(funcTemplate, {
    name,
})
const funcCreated = path.join(folder, `${name}.tf.func.js`)
fs.writeFileSync(funcCreated, funcRendered)


// generate xxx.tf.js
const infoTemplate = fs.readFileSync(path.join(templateFolder, "target.mustache")).toString()
const infoRendered = Mustache.render(infoTemplate, {
    name,
})
const infoCreated = path.join(folder, `${name}.tf.js`)
fs.writeFileSync(infoCreated, infoRendered)


// generate config vue component
// todo

