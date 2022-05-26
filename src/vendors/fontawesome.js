import Vue from "vue";
import {library} from "@fortawesome/fontawesome-svg-core";
import {FontAwesomeIcon} from "@fortawesome/vue-fontawesome";
import {faGithub} from "@fortawesome/free-brands-svg-icons";
import {
  faBell,
  faCalculator,
  faChartPie,
  faComment,
  faCrown,
  faDatabase,
  faPercent,
  faQuestionCircle,
  faTerminal,
  faThumbsUp,
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
    faChartPie,
    faTerminal
);
Vue.component("font-awesome-icon", FontAwesomeIcon);