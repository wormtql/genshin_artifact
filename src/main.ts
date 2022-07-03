import "./styles/global.scss";
import "./styles/select.css";
import 'element-plus/theme-chalk/display.css'

import router from "./router/router"

import { createApp, h } from 'vue'

import App from './App.vue'
import {initI18n} from "@/i18n/i18n"

declare global {
    interface Window {
        monaApp: any
    }
}

async function mount() {
    const monaApp = createApp({
        render: () => {
            return h(App)
        },
        methods: {
            message(msg: string) {
                this.$message(msg)
            }
        }
    })

    // i18n
    await initI18n()

    // vue router
    monaApp.use(router)

    monaApp.mount("#app")

    window.monaApp = monaApp;
}

mount().catch(e => {
    console.error(e)
});

console.log(`／ ￣￣ ＼
|  ー  ー \\   /￣￣￣￣￣￣￣￣￣￣￣\\
|   ◉  ◉ |  /                       \\
\\     ▱  / ∠    神里绫华天下第一    /
 ＼      イ   \\                      /
／       ＼    \\____________________/
/  |       \\ \\
|  |        | |
|    |               | |`);

// import { listen, emit } from "@tauri-apps/api/event"
//
// (async function () {
//     const unlisten = await listen("test", event => {
//         console.log(event)
//     })
//
//     emit("yas-scan", {
//         min_star: 5
//     })
// })()
