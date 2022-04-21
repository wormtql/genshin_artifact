import Vue from "vue"
import VueECharts from "vue-echarts"
import { use } from "echarts/core"
import { CanvasRenderer } from "echarts/renderers"
import {
    LineChart,
    PieChart,
} from "echarts/charts"
import {
    TooltipComponent,
    VisualMapComponent,
    ToolboxComponent,
    LegendComponent,
    GridComponent,
    TitleComponent,
} from "echarts/components"

use([
    CanvasRenderer,
    LineChart,
    PieChart,
    TooltipComponent,
    VisualMapComponent,
    ToolboxComponent,
    LegendComponent,
    GridComponent,
    TitleComponent
])

Vue.component("v-chart", VueECharts)