import pfFunc from "@asset/potential_functions/func";

export default function (name, args) {
    let item = pfFunc[name];
    if (!item) {
        throw new Error(`potential function ${name} not found`);
    }

    let f = null;
    if (!item.needConfig) {
        f = item.func;
    } else {
        f = item.func({
            pArgs: args
        });
    }

    // create valid sub stat, for tree cut-off to optimize calculation time
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