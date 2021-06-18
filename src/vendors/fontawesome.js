import Vue from "vue";
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
  faCrown,
  faCalculator,
  faBell,
  faThumbsUp,
  faChartPie,
} from "@fortawesome/free-solid-svg-icons";

library.add(
  faGithub,
  faQuestionCircle,
  faPercent,
  faComment,
  faDatabase,
  faCrown,
  faCalculator,
  faBell,
  faThumbsUp,
  faChartPie
  );
Vue.component("font-awesome-icon", FontAwesomeIcon);