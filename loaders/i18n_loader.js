const fs = require("fs")
const path = require("path")
const Mustache = require("mustache")

const templatePath = path.resolve(__dirname, "i18n_loader_template.mustache")
const template = fs.readFileSync(templatePath).toString()

module.exports = function (source) {
    this.addDependency(templatePath)

    const composingObject = JSON.parse(source)

    const itemList = []

    let it = 0
    for (const item of composingObject) {
        const name = `import${it}`
        it++
        itemList.push({
            name,
            path: item.path,
            prefix: item.prefix,
        })
    }

    const rendered = Mustache.render(template, {
        items: itemList
    })

    console.log(rendered)
    return rendered
}
