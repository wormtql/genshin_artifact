const { execSync } = require("child_process")
const path = require("path")

console.log("building all rust files...")
execSync("cargo build --release", { cwd: path.resolve(__dirname, "mona") })
execSync("wasm-pack build", { cwd: path.resolve(__dirname, "mona") })
execSync("npm run build:2")
