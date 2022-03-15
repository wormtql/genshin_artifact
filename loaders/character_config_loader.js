const yaml = require("js-yaml")
const Mustache = require("mustache")
const fs = require("fs")
const path = require("path")

const pathFloatConfigTemplate = path.resolve(__dirname, "character_templates", "float_config.mustache")
const pathIntConfigTemplate = path.resolve(__dirname, "character_templates", "int_config.mustache")
const pathBoolConfigTemplate = path.resolve(__dirname, "character_templates", "bool_config.mustache")
const pathCharacterConfigTemplate = path.resolve(__dirname, "character_templates", "character_config.mustache")
const pathCharacterSkillConfigTemplate = path.resolve(__dirname, "character_templates", "character_skill_config.mustache")

const floatConfigTemplate = fs.readFileSync(pathFloatConfigTemplate).toString()
const intConfigTemplate = fs.readFileSync(pathIntConfigTemplate).toString()
const boolConfigTemplate = fs.readFileSync(pathBoolConfigTemplate).toString()
const characterConfigTemplate = fs.readFileSync(pathCharacterConfigTemplate).toString()
const characterSkillConfigTemplate = fs.readFileSync(pathCharacterSkillConfigTemplate).toString()

const typeMapTemplate = {
    "float": floatConfigTemplate,
    "int": intConfigTemplate,
    "bool": boolConfigTemplate
}

function renderConfigItemHtml(configItem, characterName) {
    const context = Object.assign({}, configItem, {
        characterName
    })
    const template = typeMapTemplate[configItem.type]
    return Mustache.render(template, context)
}

function renderCharacter(parsed, options) {
    const characterName = parsed.characterName
    let items = []
    for (const config of parsed.configs) {
        const html = renderConfigItemHtml(config, characterName)

        items.push({
            html,
            name: config.name,
            default: config.default
        })
    }

    let template
    if (options.type === "character") {
        template = characterConfigTemplate
    } else {
        template = characterSkillConfigTemplate
    }

    return Mustache.render(template, {
        items,
        characterName: parsed.characterName
    })
}

module.exports = function (source) {
    this.addDependency(pathFloatConfigTemplate)
    this.addDependency(pathIntConfigTemplate)
    this.addDependency(pathBoolConfigTemplate)
    this.addDependency(pathCharacterConfigTemplate)
    this.addDependency(pathCharacterSkillConfigTemplate)

    const options = this.getOptions()

    const parsed = yaml.load(source)
    
    return renderCharacter(parsed, options)
}