function upgradeFromV1(preset) {
    if (!preset.cName) {
        throw new Error("wrong preset v1 format");
    }

    let ret = {
        name: preset.name,
        character: {
            name: preset.cName,
            level: preset.cLevel,
            ascend: preset.cAscend,
            skill1: preset.cS1,
            skill2: preset.cS2,
            skill3: preset.cS3,
            constellation: preset.cC,
        },
        weapon: {
            name: preset.wName,
            level: preset.wLevel,
            ascend: preset.wAscend,
            refine: preset.wRefine,
            args: preset.wArgs,
        },
        targetFunc: {
            name: preset.tName,
            args: preset.tArgs,
        },
    };

    return ret;
}

const funcs = [
    upgradeFromV1,
];

export default function (preset) {
    let version = preset.version ?? "1";

    let index = parseInt(version) - 1;
    return funcs[index](preset);
}