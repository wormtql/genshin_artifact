// generated file, do not edit
{% for c in characters %}
// import {{ c.name }}_card from "@image/characters/{{ c.name }}_card"
// import {{ c.name }}_avatar from "@image/characters/{{ c.name }}_avatar"
import {{ c.name }}_splash from "@image/characters/{{ c.name }}_splash"
{% endfor %}

// const template = "https://upload-bbs.mihoyo.com/game_record/genshin/character_icon/UI_AvatarIcon_#.png?x-oss-process=image/crop,w_200,h_200,y_5,g_north"
const template = "https://upload-bbs.mihoyo.com/game_record/genshin/character_icon/UI_AvatarIcon_#.png?x-oss-process=image/crop,w_200,h_200,y_5,g_north"
const newTemplate = "https://act-webstatic.mihoyo.com/hk4e/e20200928calculate/item_icon_uceddf/#.png?x-oss-process=image/crop,w_200,h_200,y_5,g_north"
const getName = name => template.replace("#", name)
const getMd5 = md5 => newTemplate.replace("#", md5)

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
        avatar: getName("{{ c.internal_name }}"),
        {% else -%}
        avatar: getMd5("{{ c.icon_hash }}"),
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
