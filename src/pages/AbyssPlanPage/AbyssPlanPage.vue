<template>
    <div>
        <button @click="f">123</button>
    </div>
</template>

<script>
import { computeShieldRate, computeDamageRate } from "@/abyss";
import Enemy from "@asset/enemies/enemy";
import { charactersData } from "@asset/characters";

export default {
    name: "AbyssPlanPage",
    methods: {
        f() {
            let enemiesConfig = [
                { name: "hilichurl" },
                { name: "anemoSamachurl" },
                { name: "rockShieldwallMitachurl" },
                { name: "iceShieldMitachurl" },
            ]

            let enemies = enemiesConfig.map(item => {
                return new Enemy(item.name, 100);
            })

            let characters = [
                charactersData["hutao"],
                charactersData["xingqiu"],
                charactersData["fengyuanwanye"],
                charactersData["zhongli"],
            ];

            // console.log(enemies);
            console.log(computeShieldRate(enemies, characters));
            console.log("damage", computeDamageRate(enemies, characters));
        }
    }
}
</script>