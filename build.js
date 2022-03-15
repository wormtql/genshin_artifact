const { execSync } = require("child_process")
const path = require("path")

let target = ""
let cmd = "cargo build --release"
if (process.env.NETLIFY === "true") {
    console.log("target x86_64-unknown-linux-gnu")
    target = "x86_64-unknown-linux-gnu"
    cmd = `cargo build --release --target=${target}`
}

console.log("building all rust files...")
execSync(cmd, { cwd: path.resolve(__dirname, "mona") })
execSync("wasm-pack build", { cwd: path.resolve(__dirname, "mona") })
execSync("npm run build:2")
