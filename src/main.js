import "./styles/global.scss";
import "./styles/select.css";

import "./vendors/element";
import "./vendors/fontawesome";
import "./vendors/vuerouter";
// import "./vendors/vue-katex";
import "./vendors/echarts";
import "./vendors/mona";

// install global api
import "./global_api";

import Vue from 'vue';

import Router from "./router/router";
import store from "./store/store";

import App from './App.vue';

Vue.config.productionTip = false

Vue.filter("str", function (value) {
    return value.toString();
});

async function mount() {
    let monaApp = new Vue(
        {
            render: h => h(App),
            methods: {
                message(msg) {
                    this.$message(msg);
            },
        },
        router: Router,
        store,
    }).$mount('#app');

    window.monaApp = monaApp;
}

mount();

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
