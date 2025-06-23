// generated file, do not edit
{% for c in characters %}
// import {{ c.name }}_card from "@image/characters/{{ c.name }}_card"
// import {{ c.name }}_avatar from "@image/characters/{{ c.name }}_avatar"
import {{ c.name }}_splash from "@image/characters/{{ c.name }}_splash"
{% endfor %}

// const template = "https://upload-bbs.mihoyo.com/game_record/genshin/character_icon/UI_AvatarIcon_#.png?x-oss-process=image/crop,w_200,h_200,y_5,g_north"
const template = "https://upload-bbs.mihoyo.com/game_record/genshin/character_icon/UI_AvatarIcon_#.png?x-oss-process=image/crop,w_200,h_200,y_5,g_north"
const newTemplate = "https://act-webstatic.mihoyo.com/hk4e/e20200928calculate/item_icon_u9b0pg/#.png?x-oss-process=image/crop,w_200,h_200,y_5,g_north"
const newerTemplate = "https://act-webstatic.mihoyo.com/hk4e/e20200928calculate/item_icon/#.png?x-oss-process=image/crop,w_200,h_200,y_5,g_north"

const avatarPathMap = {
    "Kinich": "67c7f6c8/ec67809641b4f1d2d2e87bc0e1e88b93",
    "Xilonen": "67c7f6c8/7c047dce3a1be70baa30f28111222e38",
    "Chasca": "67c7f6c8/c009627a0c0289646a10d41779001b9d",
    "Ororon": "67c7f6c8/64a90c55b4bb72cf06068f3d82dbc8da",
    "Mavuika": "67c7f6c8/c54cf9fe971d6226b78ad795ced9bee8",
    "Citlali": "67c7f6c8/abe2da348abe64c67d9c2763d2d8c4e7",
    "Lanyan": "67c7f6c8/eba829f8ca4b38367c4a2551d0e228ae",
    "YumemizukiMizuki": "67c7f6c8/1cebf43ac944ad3ff2a0581bebe309d8",
    "Varesa": "67e3357c/34596fcb08c5c4b4e91379507e75dba7",
    "Iansan": "67e3357c/f44e8b4c7a61a4abb2aa2ccfe9b810cd",
    "Escoffier": "681a947b/6c4f7f722b0226a1171ec265485de61c",
    "Ifa": "681a947b/0b3127bb721214a48b20001c64ab727a",
    "Skirk": "6851f37b/b3007bcbb99dd9b47af558ecc1bb65ff",
    "Dahlia": "6851f37b/9e5b1c5805948358c656e702e5eecae2",
}

const getName = name => template.replace("#", name)
const getHash = hash => newTemplate.replace("#", hash)
const getPath = path => newerTemplate.replace("#", path)

export default {
    {% for c in characters %}
    {{ c.name }}: {
        name: "{{ c.name }}",
        nameLocale: {{ c.name_locale }},
        element: "{{ c.element }}",
        weapon: "{{ c.weapon }}",
        star: {{ c.star }},
        // card: {{ c.name }}_card,
        // avatar: {{ c.name }}_avatar,
        {% if c.icon_hash == "" -%}
        avatar: avatarPathMap["{{ c.name }}"] ? getPath(avatarPathMap["{{ c.name }}"]) : getName("{{ c.internal_name }}"),
        {% else -%}
        avatar: getHash("{{ c.icon_hash }}"),
        {%- endif %}
        splash: {{ c.name }}_splash,
        skillName1: {{ c.skill1_name_index }},
        skillName2: {{ c.skill2_name_index }},
        skillName3: {{ c.skill3_name_index }},
        skillMap1: [
            {% for s in c.skill_map1 %}
            { index: {{ s.index }}, text: {{ s.locale_index }} },
            {% endfor %}
        ],
        skillMap2: [
            {% for s in c.skill_map2 %}
            { index: {{ s.index }}, text: {{ s.locale_index }} },
            {% endfor %}
        ],
        skillMap3: [
            {% for s in c.skill_map3 %}
            { index: {{ s.index }}, text: {{ s.locale_index }} },
            {% endfor %}
        ],
        config: [
            {% for config in c.config %}
            {{ config|e("none") }},
            {% endfor %}
        ],
        configSkill: [
            {% for config in c.config_skill %}
            {{ config|e("none") }},
            {% endfor %}
        ],
    },
    {% endfor %}
}
