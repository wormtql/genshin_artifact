/// #if !USE_CDN
import Vue from "vue"
// echarts relies on composition-api
// import "@vue/composition-api"
import { use } from "echarts/core"
import { CanvasRenderer } from "echarts/renderers"
import {
    LineChart,
    PieChart,
    BarChart
} from "echarts/charts"
import {
    TooltipComponent,
    VisualMapComponent,
    ToolboxComponent,
    LegendComponent,
    GridComponent,
    TitleComponent,
} from "echarts/components"
console.log("!use cdn")
/// #else
console.log("use cdn")
/// #endif

import VueECharts from "vue-echarts"

/// #if !USE_CDN
use([
    CanvasRenderer,
    LineChart,
    PieChart,
    BarChart,
    TooltipComponent,
    VisualMapComponent,
    ToolboxComponent,
    LegendComponent,
    GridComponent,
    TitleComponent
])
/// #endif

Vue.component("v-chart", VueECharts)