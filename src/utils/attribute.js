import * as genshin from "genshin_panel";

function getArtifact(myArtifact) {
    let temp = new genshin.ArtifactBuilder()
        .setName(myArtifact.setName)
        .position(myArtifact.position)
        .mainTag(myArtifact.mainTag.name, myArtifact.mainTag.value)
    ;

    for (let tag of myArtifact.normalTags) {
        temp.tag(tag.name, tag.value);
    }

    return temp.build();
}

export function applyBuffs(attribute, buffs) {
    for (let buff of buffs) {
        switch (buff.type) {
            case "atk-static":
                attribute.attackStatic += buff.value;
                break;
            case "atk-percentage":
                attribute.attackPercentage += buff.value * attribute.attackBasic;
                break;
            case "bonus":
                attribute.bonus += buff.value;
                break;
            case "critical":
                attribute.crit(buff.value);
                break;
            case "em":
                attribute.elementalMastery += buff.value;
                break;
            case "criticalDamage":
                attribute.criticalDamage += buff.value;
                break;
        }
    }
}

export function getAttribute(artifacts, c, w, stdBuffs, artifactsConfig = {}) {
    let character = new genshin.Character(c.name, c.level, c.ascend, c.constellation, c.args);
    let weapon = new genshin.Weapon(w.name, w.level, w.ascend, w.refine, w.args);

    let builder = new genshin.AttributeBuilder();
    builder
        .character(character)
        .weapon(weapon)
    ;
    for (let art of artifacts) {
        builder.artifact(getArtifact(art));
    }
    builder.artifactsConfig(artifactsConfig);

    let attribute = builder.build();
    applyBuffs(attribute, stdBuffs);

    return attribute;
}