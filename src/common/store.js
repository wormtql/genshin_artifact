import Vuex from "vuex";
import Vue from "vue";
import { getCharacterAttribute, getWeaponAttribute } from "genshin_panel";

Vue.use(Vuex);

export const store = new Vuex.Store({
    state: {
        flower: [],
        feather: [],
        sand: [],
        cup: [],
        head: [],

        id: 0,

        // 是否自定义武器
        customWeapon: false,
        // 是否自定义角色
        customCharacter: false,
        selectedCharacter: "keqing-70-0",
        selectedWeapon: "heijian-70-0",
        selectedWeaponAttribute: getWeaponAttribute("heijian-70-0"),
        selectedCharacterAttribute: getCharacterAttribute("keqing-70-0"),
        selectedCustomWeapon: "",
        selectedCustomCharacter: "",

        // 当前选择的目标函数名称
        selectedTargetFunction: "keqing_normal",

        customedWeapons: {},
        customedCharacters: {},

        customedTargets: {},
    },
    mutations: {
        addCustomedTarget(state, target) {
            Vue.set(state.customedTargets, target.name, target);
        },

        deleteCustomedTarget(state, name) {
            Vue.delete(state.customedTargets, name);
        },

        addCustomedWeapon(state, weapon) {
            Vue.set(state.customedWeapons, weapon.name, weapon.item);
        },

        deleteCustomedWeapon(state, name) {
            Vue.delete(state.customedWeapons, name);
        },

        addCustomedCharacter(state, ch) {
            // state.customedCharacters[ch.name] = ch.item;
            Vue.set(state.customedCharacters, ch.name, ch.item);
            // window.console.log(state.customedCharacters);
        },

        deleteCustomedCharacter(state, name) {
            // delete state.customedCharacters[name]
            Vue.delete(state.customedCharacters, name);
        },

        setSelectedTargetFunction(state, target) {
            state.selectedTargetFunction = target;
        },

        setSelectedWeapon(state, weapon) {
            state.selectedCustomWeapon = "";
            state.customWeapon = false;
            state.selectedWeapon = weapon;
            state.selectedWeaponAttribute = getWeaponAttribute(weapon);
        },

        setSelectedCharacter(state, character) {
            state.selectedCustomCharacter = "";
            state.customCharacter = false;
            state.selectedCharacter = character;
            state.selectedCharacterAttribute = getCharacterAttribute(character);
        },

        setSelectedCustomWeapon(state, weapon) {
            state.selectedWeapon = "";
            state.customWeapon = true;
            state.selectedCustomWeapon = weapon;
            state.selectedWeaponAttribute = state.customedWeapons[weapon];
        },

        setSelectedCustomCharacter(state, character) {
            state.selectedCharacter = "";
            state.customCharacter = true;
            state.selectedCustomCharacter = character;
            state.selectedCharacterAttribute = state.customedCharacters[character];
        },

        setSelectedWeaponAttribute(state, attr) {
            state.customWeapon = true;
            state.selectedWeaponAttribute = attr;
        },

        setSelectedCharacterAttribute(state, attr) {
            state.customCharacter = true;
            state.selectedCharacterAttribute = attr;
        },

        addFlower(state, item) {
            // id no need to be reactive
            item["id"] = state.id++;
            state.flower.push(item);
        },

        deleteFlower(state, item) {
            let index = state.flower.indexOf(item);
            state.flower.splice(index, 1);
        },
        
        setFlower(state, item) {
            state.flower = item;
        },

        addFeather(state, item) {
            item["id"] = state.id++;
            state.feather.push(item);
        },

        deleteFeather(state, item) {
            let index = state.feather.indexOf(item);
            state.feather.splice(index, 1);
        },

        setFeather(state, item) {
            state.feather = item;
        },

        addSand(state, item) {
            item["id"] = state.id++;
            state.sand.push(item);
        },
        
        deleteSand(state, item) {
            let index = state.sand.indexOf(item);
            state.sand.splice(index, 1);
        },

        setSand(state, item) {
            state.sand = item;
        },

        addCup(state, item) {
            item["id"] = state.id++;
            state.cup.push(item);
        },

        deleteCup(state, item) {
            let index = state.cup.indexOf(item);
            state.cup.splice(index, 1);
        },

        setCup(state, item) {
            state.cup = item;
        },

        addHead(state, item) {
            item["id"] = state.id++;
            state.head.push(item);
        },

        deleteHead(state, item) {
            let index = state.head.indexOf(item);
            state.head.splice(index, 1);
        },

        setHead(state, item) {
            state.head = item;
        }
    }
})