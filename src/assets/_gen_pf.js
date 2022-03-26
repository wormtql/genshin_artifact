// generated file, do not edit

import ArtifactEff_image from "@image/misc/sword"


export default {

    "ArtifactEff": {
        name: "ArtifactEff",
        chs: "有效词条",
        description: "以单次强化最大档位为1分",
        badge: ArtifactEff_image,
        config: [
            
            {"default":false,"name":"atk_use","title":"攻击力有效","type":"bool"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"atk_weight","title":"攻击力权重","type":"float"},
            
            {"default":true,"name":"atk_p_use","title":"攻击力%有效","type":"bool"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"atk_p_weight","title":"攻击力%权重","type":"float"},
            
            {"default":false,"name":"hp_use","title":"生命值有效","type":"bool"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"hp_weight","title":"生命值权重","type":"float"},
            
            {"default":false,"name":"hp_p_use","title":"生命值%有效","type":"bool"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"hp_p_weight","title":"生命值%权重","type":"float"},
            
            {"default":false,"name":"def_use","title":"防御力有效","type":"bool"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"def_weight","title":"防御力权重","type":"float"},
            
            {"default":false,"name":"def_p_use","title":"防御力%有效","type":"bool"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"def_p_weight","title":"防御力%权重","type":"float"},
            
            {"default":true,"name":"critical_use","title":"暴击率有效","type":"bool"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"critical_weight","title":"暴击率权重","type":"float"},
            
            {"default":true,"name":"critical_damage_use","title":"暴击伤害有效","type":"bool"},
            
            {"default":1.0,"max":1.0,"min":0.0,"name":"critical_damage_weight","title":"暴击伤害权重","type":"float"},
            
            {"default":false,"name":"elemental_mastery_use","title":"元素精通有效","type":"bool"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"elemental_mastery_weight","title":"元素精通权重","type":"float"},
            
            {"default":false,"name":"recharge_use","title":"元素充能效率有效","type":"bool"},
            
            {"default":0.0,"max":1.0,"min":0.0,"name":"recharge_weight","title":"元素充能效率权重","type":"float"},
            
        ],
    },

}