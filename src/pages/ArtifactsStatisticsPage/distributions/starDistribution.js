export default function(vm) {
    let allFlat = vm.$store.getters["artifacts/allFlat"];

    let count = [0, 0, 0, 0, 0];
    for (let i = 0; i < allFlat.length; i++) {
        let star = allFlat[i].star ?? 5;
        count[star - 1]++;
    }

    let options = {
        tooltip: {
            trigger: "item"
        },
        legend: {
            // top: "0",
            left: "left",
            orient: "vertical",
        },
        series: [
            {
                name: "品质分布",
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
                data: [
                    { value: count[0], name: "1星" },
                    { value: count[1], name: "2星" },
                    { value: count[2], name: "3星" },
                    { value: count[3], name: "4星" },
                    { value: count[4], name: "5星" },
                ],
            }
        ]
    }

    return options;
}