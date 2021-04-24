export default function(vm) {
    let allFlat = vm.$store.getters["artifacts/allFlat"];

    let count = Array(21).fill(0);
    for (let i = 0; i < allFlat.length; i++) {
        let level = allFlat[i].level ?? 20;
        count[level]++;
    }

    let options = {
        tooltip: {
            trigger: "item"
        },
        legend: {
            orient: "vertical",
            // top: "0",
            left: "left",
        },
        series: [
            {
                name: "等级分布",
                type: "pie",
                radius: ["40%", "70%"],
                label: {
                    show: false
                },
                itemStyle: {
                    borderRadius: 10,
                    borderColor: '#fff',
                    borderWidth: 2
                },
                data: count.map((item, index) => ({
                    value: item,
                    name: `${index}级`,
                })).filter(item => item.value > 0)
            }
        ]
    }

    return options;
}