<template>
    <div class="select-weapon">
        <div
            v-for="w in allowedWeapon"
            :key="w.name"
            class="item"
            :class="{ active: w.name === weapon }"
            @click="handleSelectWeapon(w.name)"
        >
            <img
                :src="w.url"
                class="image"
            >
            <span
                class="text"
                :style="{color: `${colors[w.star - 1]}`}"
            >{{ w.chs }}</span>
        </div>
    </div>
</template>

<script>
import { weaponsData } from "@asset/weapons";

let weaponTypeMap = {
    "sword": [],
    "sword2": [],
    "book": [],
    "stick": [],
    "bow": [],
    "none": [],
};
Object.values(weaponsData).forEach(item => {
    weaponTypeMap[item.type].push(item);
})

// sort by star
for (let weaponType in weaponTypeMap) {
    weaponTypeMap[weaponType].sort((a, b) => {
        return b.star - a.star;
    });
}

export default {
    name: "SelectWeapon",
    inject: ["notifyChange"],
    created: function () {
        let transparent = 1;
        this.colors = [
            `rgba(125,135,147,${transparent})`,
            `rgba(95,200,117,${transparent})`,
            `rgba(86,151,200,${transparent})`,
            `rgba(200,134,194,${transparent})`,
            `rgba(205,155,92,${transparent})`,
        ]
    },
    data() {
        return {
            allow: "bow",
            weapon: "liegong",
        }
    },
    methods: {
        handleSelectWeapon(name) {
            if (name !== this.weapon) {
                this.weapon = name;
                this.notifyChange("weapon", name);
            }
        },

        setAllow(name) {
            if (this.allow !== name) {
                this.allow = name;

                let newName = weaponTypeMap[name][0].name;
                this.handleSelectWeapon(newName);
            }
        },

        getWeaponName() {
            return this.weapon;
        },

        setWeaponName(allow, name) {
            this.allow = allow;
            this.weapon = name;
        }
    },
    computed: {
        allowedWeapon() {
            return weaponTypeMap[this.allow];
        }
    }
}
</script>

<style scoped>
.image {
    width: 80%;
    /* height: 64px; */
    border-radius: 50%;
    /* display: block; */
    /* background: rgba(240, 198, 62, 0.781); */
    /* border: 2px solid rgba(240, 198, 62, 0.781); */
    border: 1px solid #00000011;
    transition: 300ms;
}

.text {
    display: block;
    width: 100%;
    box-sizing: border-box;
    text-align: center;
    font-size: 12px;
    padding-top: 8px;
    color: #999999;
    /* padding: 4px 0; */
    /* margin: 0 32px; */
    /* margin-top: 8px; */
    /* color: white; */
}

.item {
    width: 12.5%;
    /* margin: 8px 20px 0 0; */
    /* padding: 8px; */
    cursor: pointer;
    transition: 300ms;
    border-radius: 3px;
    text-align: center;
    padding: 8px 0;
    /* width: 11%; */
    /* min-width: 128px; */
}

.item:hover {
    /* background: rgba(240, 198, 62, 0.3); */
    background: rgba(0, 0, 0, 0.05);
}

.select-weapon {
    display: flex;
    flex-flow: row wrap;
    /* justify-content: space-between; */
}

.active {
    background: #12345622;
}

/* .select-weapon::after {
    content: "";
    width: 128px;
    flex: auto;
} */
</style>