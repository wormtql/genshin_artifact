const yaml = require("js-yaml")
const Mustache = require("mustache")
const fs = require("fs")
const path = require("path")

const pathFloatConfigTemplate = path.resolve(__dirname, "weapon_templates", "float_config.mustache")
const pathIntConfigTemplate = path.resolve(__dirname, "weapon_templates", "int_config.mustache")
const pathBoolConfigTemplate = path.resolve(__dirname, "weapon_templates", "bool_config.mustache")
const pathWeaponConfigTemplate = path.resolve(__dirname, "weapon_templates", "weapon_config.mustache")

const floatConfigTemplate = fs.readFileSync(pathFloatConfigTemplate).toString()
const intConfigTemplate = fs.readFileSync(pathIntConfigTemplate).toString()
const boolConfigTemplate = fs.readFileSync(pathBoolConfigTemplate).toString()
const weaponConfigTemplate = fs.readFileSync(pathWeaponConfigTemplate).toString()

const typeMapTemplate = {
    "float": floatConfigTemplate,
    "int": intConfigTemplate,
    "bool": boolConfigTemplate
}

function renderConfigItemHtml(configItem, weaponName) {
    const context = Object.assign({}, configItem, {
        weaponName
    })
    const template = typeMapTemplate[configItem.type]
    return Mustache.render(template, context)
}

function renderWeapon(parsed) {
    const weaponName = parsed.weaponName
    let items = []
    for (const config of parsed.configs) {
        const html = renderConfigItemHtml(config, weaponName)

        items.push({
            html,
            name: config.name,
            default: config.default
        })
    }

    return Mustache.render(weaponConfigTemplate, {
        items,
        weaponName: parsed.weaponName
    })
}

module.exports = function (source) {
    this.addDependency(pathFloatConfigTemplate)
    this.addDependency(pathIntConfigTemplate)
    this.addDependency(pathBoolConfigTemplate)
    this.addDependency(pathWeaponConfigTemplate)

    const parsed = yaml.load(source)
    
    return renderWeapon(parsed)
}