<template>
    <div>
        <div class="character-panel">
            <div
                v-for="c in elementCharData['thunder']"
                :key="c.name"
                class="character-item"
                :class="{active: c.name === character}"
                @click="handleClickCharacter(c.name)"
            >
                <span v-if="c.test" class="test">测试</span>
                <img :src="c.cardURL" class="image">
                <span class="text">{{ c.chs }}</span>
            </div>
        </div>

        <div class="character-panel">
            <div
                v-for="c in elementCharData['fire']"
                :key="c.name"
                class="character-item"
                :class="{active: c.name === character}"
                @click="handleClickCharacter(c.name)"
            >
                <span v-if="c.test" class="test fs-12">非正式服</span>
                <img :src="c.cardURL" class="image">
                <span class="text">{{ c.chs }}</span>
            </div>
        </div>

        <div class="character-panel">
            <div
                v-for="c in elementCharData['rock']"
                :key="c.name"
                class="character-item"
                :class="{active: c.name === character}"
                @click="handleClickCharacter(c.name)"
            >
                <img :src="c.cardURL" class="image">
                <span class="text">{{ c.chs }}</span>
            </div>
        </div>

        <div class="character-panel">
            <div
                v-for="c in elementCharData['wind']"
                :key="c.name"
                class="character-item"
                :class="{active: c.name === character}"
                @click="handleClickCharacter(c.name)"
            >
                <img :src="c.cardURL" class="image">
                <span class="text">{{ c.chs }}</span>
            </div>
        </div>

        <div class="character-panel">
            <div
                v-for="c in elementCharData['ice']"
                :key="c.name"
                class="character-item"
                :class="{active: c.name === character}"
                @click="handleClickCharacter(c.name)"
            >
                <img :src="c.cardURL" class="image">
                <span class="text">{{ c.chs }}</span>
            </div>
        </div>

        <div class="character-panel">
            <div
                v-for="c in elementCharData['water']"
                :key="c.name"
                class="character-item"
                :class="{active: c.name === character}"
                @click="handleClickCharacter(c.name)"
            >
                <img :src="c.cardURL" class="image">
                <span class="text">{{ c.chs }}</span>
            </div>
        </div>
    </div>
</template>

<script>
import { charactersData } from "@asset/characters";

let elementCharData = {
    thunder: [],
    water: [],
    rock: [],
    fire: [],
    wind: [],
    ice: [],
};

for (let charData of Object.values(charactersData)) {
    elementCharData[charData.element].push(charData);
}

export default {
    name: "SelectCharacter",
    inject: ["notifyChange"],
    created: function () {
        this.elementCharData = elementCharData;
    },
    data() {
        return {
            character: "anbo",
        }
    },
    props: {
        value: {
            type: String,
            required: true,
        }
    },
    methods: {
        handleClickCharacter(name) {
            if (this.character !== name) {
                this.character = name;
                this.notifyChange("character", name);
            }
        },

        getCharacterName() {
            return this.character;
        },

        setCharacterName(name) {
            this.character = name;
        }
    }
};
</script>

<style scoped>
.character-item {
    width: 96px;
    cursor: pointer;
    margin-right: 12px;
    padding: 8px;
    transition: 300ms;
    border-radius: 3px;
    position: relative;
}

.character-item:hover {
    background: rgba(0, 0, 0, 0.1);
}

.character-item .test {
    position: absolute;
    left: 4px;
    top: 4px;
    color: white;
    border-radius: 3px;
    background: rgb(243, 183, 18);
    padding: 4px;
}

.text {
    display: block;
    text-align: center;
    font-size: 14px;
    color: #999999;
    padding-top: 8px;
}

.image {
    border-radius: 5px;
    width: 96px;
    /* height: 250px; */
    display: block;
}

.character-panel {
    display: flex;
    -ms-overflow-style: none;
    scrollbar-width: 0;
    overflow: auto;
    margin-bottom: 32px;
    /* flex-wrap: wrap; */
}

.character-panel::-webkit-scrollbar {
    display: none;
}

.active {
    background: #12345622;
}
</style>