import "./styles/global.css";

import Vue from 'vue';
import VueRouter from "vue-router";
// import Vuex from "vuex"
import ElementUI from "element-ui";
import "element-ui/lib/theme-chalk/index.css";
import ECharts from "vue-echarts";
import "echarts/lib/chart/pie";

import Router from "./common/router";
import { store } from "./common/store";

import App from './App.vue';

// import Clipboard from "clipboard";

// new Clipboard(".clip");

Vue.use(ElementUI);
Vue.use(VueRouter);
Vue.component("v-chart", ECharts);

Vue.config.productionTip = false

new Vue({
  render: h => h(App),
  router: Router,
  store,
}).$mount('#app')
