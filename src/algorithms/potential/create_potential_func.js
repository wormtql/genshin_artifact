import pfFunc from "@asset/potential_functions/func";

export default function (name, args) {
    let item = pfFunc[name];
    if (!item) {
        throw "fatal error";
    }

    let f = null;
    if (!item.needConfig) {
        f = item.func;
    } else {
        f = item.func({
            pArgs: args
        });
    }

    let valid = null;
    if (typeof item.valid === "function") {
        valid = item.valid({ pArgs: args });
    } else if (Array.isArray(item.valid)) {
        valid = item.valid;
    }

    return {
        f,
        valid,
    };
}