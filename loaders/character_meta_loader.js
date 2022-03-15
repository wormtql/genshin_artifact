const path = require("path")
const fs = require("fs")
const { execSync } = require("child_process")


function get_meta() {
    let cmd = path.resolve(__dirname, "../mona/target/release/gen_character_meta.exe")
    if (!fs.existsSync(cmd)) {
        cmd = path.resolve(__dirname, "../mona/target/debug/gen_character_meta.exe")
    }
    if (!fs.existsSync(cmd)) {
        cmd = "cargo run --bin gen_character_meta"
    }
    console.log(cmd)

    let stdout = execSync(cmd)
    let ret = stdout.toString();
    // console.log(ret)

    return ret
}

module.exports = function (source) {
    return get_meta()
}
