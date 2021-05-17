import { createFireFunction } from "../../../utils";
// import mergeArray from "@util/mergeArray";
// import { getAttribute } from "@util/attribute";

let skillKeys = [
    {
        key: "dmg1",
        chs: "喷火伤害",
        skill: "e",
        element: "fire",
    },
];

export default createFireFunction(skillKeys, "e");