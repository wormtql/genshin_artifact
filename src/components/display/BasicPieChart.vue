<template>
    <v-chart :option="option" style="height: 300px" :autoresize="true"></v-chart>
</template>

<script>
export default {
    name: "BasicPieChart",
    props: ["title", "data"],
    // mounted() {
    //     console.log(this.option)
    // },
    computed: {
        option() {
            return {
                title: {
                    text: this.title,
                    left: "center"
                },
                tooltip: {
                    trigger: "item",
                    formatter: "{b}：{d}%",
                },
                // legend: {
                //     orient: 'vertical',
                //     left: 'left'
                // },
                series: [
                    {
                        // name: 'Access From',
                        type: 'pie',
                        radius: '50%',
                        center: ["50%", "60%"],
                        data: this.data,
                        emphasis: {
                            itemStyle: {
                                shadowBlur: 10,
                                shadowOffsetX: 0,
                                shadowColor: 'rgba(0, 0, 0, 0.5)'
                            }
                        }
                    }
                ]
            }
        }
    }
}
</script>

<style scoped>

</style>