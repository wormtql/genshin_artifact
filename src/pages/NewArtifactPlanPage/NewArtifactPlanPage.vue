<template>
    <div>
        <el-breadcrumb>
            <el-breadcrumb-item>裏</el-breadcrumb-item>
        </el-breadcrumb>
        <el-divider></el-divider>

        <el-button
            @click="onClick"
        >text</el-button>
    </div>
</template>

<script>
import { convertArtifact } from "@util/converter"

const artifactConfig = {
    "config_archaic_petra": {
        "element": "Electro",
        rate: 0,
    },
    "config_berserker": { rate: 0 },
    "config_blizzard_strayer": { critical_bonus: 0 },
    "config_bloodstained_chivalry": { rate: 0 },
    "config_brave_heart": { rate: 0 },
    "config_crimson_witch_of_flames": { level: 0 },
    "config_heart_of_depth": { rate: 0 },
    "config_husk_of_opulent_dreams": { level: 0 },
    "config_instructor": { rate: 0 },
    "config_lavawalker": { rate: 0 },
    "config_martial_artist": { rate: 0 },
    "config_noblesse_oblige": { rate: 0 },
    "config_pale_flame": { avg_level: 0, full_rate: 0 },
    "config_retracing_bolide": { rate: 0 },
    "config_shimenawas_reminiscence": { rate: 0 },
    "config_tenacity_of_the_millelith": { rate: 0 },
    "config_thundersoother": { rate: 0 },
}

export default {
    name: "NewArtifactPlanPage",
    methods: {
        onClick () {
            const characterInterface = {
                name: "Albedo",
                level: 90,
                ascend: false,
                constellation: 0,
                params: "NoConfig",
            }

            const weaponInterface = {
                name: "MistsplitterReforged",
                level: 90,
                ascend: false,
                refine: 1,
                params: {
                    "MistsplitterReforgedConfig": 1,
                },
            }

            const targetFunctionInterface = {
                name: "Expectation",
                params: {
                    "ExpectationConfig": ["Cryo", "NormalAttack"]
                }
            }

            const artifacts = this.$store.getters["artifacts/allFlat"].filter(x => {
                return x.level == 20
            }).map(x => convertArtifact(x));
            // console.log(artifacts);
            const artifacts2 = artifacts.splice(0, 75);
            console.log(artifacts2);

            // const constraint = {
            //     "set_mode": "Any",
            //     "min_level": 
            // }
            const inter = {
                // artifacts: [artifacts[i]],
                artifacts: artifacts2,
                artifact_config: artifactConfig,
                character: characterInterface,
                weapon: weaponInterface,
                target_function: targetFunctionInterface,
                constraint: {},
                buffs: [],
            }

            // let ret = this.$mona.OptimizeArtifactInterface.hello();
            // let ret = this.$mona.OptimizeArtifactInterface.from_js(inter);
            let ret = this.$mona.OptimizeArtifactInterface.optimize(inter);
                
            console.log(ret);
            console.log(typeof(ret));
        }
    }
};
</script>
