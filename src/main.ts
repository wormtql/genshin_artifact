import "./styles/global.scss";
import "./styles/select.css";

// import "./vendors/mona";
import 'element-plus/theme-chalk/display.css'

import router from "./router/router"

import { createApp, h } from 'vue'

// import store from "./store/store";

import App from './App.vue';

// Vue.config.productionTip = false

// Vue.filter("str", function (value) {
//     return value.toString();
// });

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

    // vue router
    monaApp.use(router)

    monaApp.mount("#app")
    // let monaApp = new Vue(
    //     {
    //         render: h => h(App),
    //         methods: {
    //             message(msg) {
    //                 this.$message(msg);
    //         },
    //     },
    //     router: Router,
    //     store,
    // }).$mount('#app');

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
