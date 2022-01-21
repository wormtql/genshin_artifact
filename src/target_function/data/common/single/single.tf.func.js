function f(config) {
    let name = config.tArgs.fieldName;

    if (name === "attack") {
        return function (attribute) {
            return attribute.attack();
        }
    }
    
    if (name === "life") {
        return function (attribute) {
            return attribute.life();
        }
    }

    if (name === "defend") {
        return function (attribute) {
            return attribute.defend();
        }
    }

    return function(attribute) {
        return attribute[name];
    };
}

export default {
    name: "single",
    func: f,
    needConfig: true,
}