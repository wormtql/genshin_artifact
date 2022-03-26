const path = require("path")
const fs = require("fs")
const { execSync } = require("child_process")


function get_meta(name) {
    // let cmd = path.resolve(__dirname, `../mona/target/release/gen_${name}_meta`)
    // if (!fs.existsSync(cmd)) {
    //     cmd = path.resolve(__dirname, `../mona/target/debug/gen_${name}_meta`)
    // }
    // if (!fs.existsSync(cmd)) {
    //     cmd = `cargo run --release --bin gen_${name}_meta`
    cmd = path.resolve(__dirname, `../mona/target/release/gen_${name}_meta`)
    // }
    console.log(cmd)

    let stdout = execSync(cmd, { cwd: path.resolve(__dirname, "../mona") })
    let ret = stdout.toString();
    // console.log(ret)

    return ret
}

// get_weapon_meta()

module.exports = function (source) {
    const options = this.getOptions()

    return get_meta(options.type)
}
