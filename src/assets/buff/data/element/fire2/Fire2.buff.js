import badge from "@asset/badges/pyro.png";

export default {
    name: "fire2",
    chs: "元素共鸣-热诚之火",
    badge,
    genre: "resonance",
    getStandardConfig() {
        return {
            type: "atk-percentage",
            value: 0.25,
        }
    }
}