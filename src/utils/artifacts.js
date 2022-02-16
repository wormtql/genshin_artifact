import positions from "@const/positions"
import { artifactEff } from "@const/artifact"
import objectHash from "object-hash"
import store from "@/store/store"

// count how many artifacts
export function count(artifacts) {
    let c = 0;

    positions.forEach(pos => {
        c += artifacts[pos].length;
    });

    return c;
}

// count min and max upgrade count
export function howManyUpgradeCount(value, tagName, star) {
    const eff = artifactEff[star][tagName]
    const min = Math.round(value / eff[3])
    const max = Math.round(value / eff[0])
    // if (tagName === "attackPercentage") {
    //     console.log(value);
    // }

    // value = Math.round(value * 1000);
    // let eff0 = Math.round(eff[0] * 1000);
    // let eff3 = Math.round(eff[3] * 1000);
    //
    // let max = Math.floor(value / eff0);
    // let min = Math.ceil(value / eff3);
    return [min, max];
}

export function hash(artifact) {
    let h = objectHash(artifact, {
        excludeKeys: (k) => {
            return k === "id" || k === "omit" || k === "detailName";
        }
    });
    return h;
}

export function newDefaultArtifactConfigForWasm() {
    return {
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
}

export function toggleArtifact(id) {
    store.commit("artifacts/toggleArtifactById", {
        id
    })
}
