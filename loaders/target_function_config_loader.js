const yaml = require("js-yaml")
const Mustache = require("mustache")
const fs = require("fs")
const path = require("path")

const pathFloatConfigTemplate = path.resolve(__dirname, "target_function_templates", "float_config.mustache")
const pathIntConfigTemplate = path.resolve(__dirname, "target_function_templates", "int_config.mustache")
const pathBoolConfigTemplate = path.resolve(__dirname, "target_function_templates", "bool_config.mustache")
const pathTargetFunctionConfigTemplate = path.resolve(__dirname, "target_function_templates", "target_function_config.mustache")

const floatConfigTemplate = fs.readFileSync(pathFloatConfigTemplate).toString()
const intConfigTemplate = fs.readFileSync(pathIntConfigTemplate).toString()
const boolConfigTemplate = fs.readFileSync(pathBoolConfigTemplate).toString()
const targetFunctionConfigTemplate = fs.readFileSync(pathTargetFunctionConfigTemplate).toString()

const typeMapTemplate = {
    "float": floatConfigTemplate,
    "int": intConfigTemplate,
    "bool": boolConfigTemplate
}

function renderConfigItemHtml(configItem, targetFunctionName) {
    const context = Object.assign({}, configItem, {
        targetFunctionName
    })
    const template = typeMapTemplate[configItem.type]
    return Mustache.render(template, context)
}

function renderTargetFunction(parsed) {
    const targetFunctionName = parsed.targetFunctionName
    let items = []
    for (const config of parsed.configs) {
        const html = renderConfigItemHtml(config, targetFunctionName)

        items.push({
            html,
            name: config.name,
            default: config.default
        })
    }

    return Mustache.render(targetFunctionConfigTemplate, {
        items,
        targetFunctionName: parsed.targetFunctionName
    })
}

module.exports = function (source) {
    this.addDependency(pathFloatConfigTemplate)
    this.addDependency(pathIntConfigTemplate)
    this.addDependency(pathBoolConfigTemplate)
    this.addDependency(pathTargetFunctionConfigTemplate)

    const parsed = yaml.load(source)
    
    return renderTargetFunction(parsed)
}