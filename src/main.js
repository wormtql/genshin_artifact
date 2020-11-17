import "./styles/global.css";

import Vue from 'vue';
import VueRouter from "vue-router";
// import Vuex from "vuex"
import ElementUI from "element-ui";
import "element-ui/lib/theme-chalk/index.css";
import ECharts from "vue-echarts";
import "echarts/lib/chart/pie";
import { library } from "@fortawesome/fontawesome-svg-core";
import { FontAwesomeIcon } from "@fortawesome/vue-fontawesome";
import {
  faGithub
} from "@fortawesome/free-brands-svg-icons";
import {
  faQuestionCircle,
  faPercent,
  faComment,
  faDatabase,
} from "@fortawesome/free-solid-svg-icons";

import Router from "./common/router";
import { store } from "./common/store";

import App from './App.vue';

library.add(faGithub, faQuestionCircle, faPercent, faComment, faDatabase);
// import Clipboard from "clipboard";

// new Clipboard(".clip");

Vue.use(ElementUI);
Vue.use(VueRouter);
Vue.component("v-chart", ECharts);
Vue.component("font-awesome-icon", FontAwesomeIcon);

Vue.config.productionTip = false

new Vue({
  render: h => h(App),
  router: Router,
  store,
}).$mount('#app')
