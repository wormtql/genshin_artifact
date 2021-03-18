import "./styles/global.css";
import "./styles/select.css";

import "./vendors/element";
import "./vendors/fontawesome";
import "./vendors/vuerouter";
import "./vendors/vue-katex";

import Vue from 'vue';

import Router from "./router/router";
import store from "./store/store";

import App from './App.vue';

Vue.config.productionTip = false

Vue.filter("str", function (value) {
  return value.toString();
});

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