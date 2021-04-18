import badge from "@asset/badges/geo.png";

export default {
    name: "rock2",
    chs: "元素共鸣-坚定之岩",
    badge,
    genre: "resonance",
    getStandardConfig() {
        return {
            type: "bonus",
            value: 0.15,
        }
    }
}