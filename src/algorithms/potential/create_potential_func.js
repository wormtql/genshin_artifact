import pfData from "@asset/potential_functions/data";

export default function (name, args) {
    let item = pfData[name];
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

    return {
        f,
        valid: item.valid,
    };
}