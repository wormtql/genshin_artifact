function f(config) {
    let code = config.tArgs.code;

    let func = new Function("attribute", code);

    return func;
}

export default {
    name: "code",
    func: f,
    needConfig: true,
}