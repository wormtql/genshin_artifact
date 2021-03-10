import "./styles/global.css";
import "./styles/select.css";

import "./vendors/element";
import "./vendors/fontawesome";
import "./vendors/vuerouter";
import "./vendors/vue-katex";

import Vue from 'vue';

import Router from "./common/router";
import store from "./common/store";

import App from './App.vue';

Vue.config.productionTip = false

async function mount() {
  new Vue({
    render: h => h(App),
    router: Router,
    store,
  }).$mount('#app');
}

mount();

console.log(`／ ￣￣ ＼
|  ー  ー \\   /￣￣￣￣￣￣￣￣￣￣＼
|   ◉  ◉ |  /                      \\
\\     ▱  / ∠     刻师傅天下第一    /
 ＼      イ   \\                     /
／       ＼    \\___________________/
/  |       \\ \\
|  |        | |
|    |               | |`);