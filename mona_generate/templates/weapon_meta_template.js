// generated file, do not edit
// {% for weapon in weapons %}
// import {{ weapon.name }}_tn from "@image/weapons/{{ weapon.name }}_tn"
// {% endfor %}

const template = "https://upload-bbs.mihoyo.com/game_record/genshin/equip/UI_EquipIcon_#.png"
const newTemplate = "https://act-webstatic.mihoyo.com/hk4e/e20200928calculate/item_icon_uceddf/#.png"
const imageUrl = name => template.replace("#", name)
const newImageUrl = md5 => newTemplate.replace("#", md5)

export default {
{% for weapon in weapons %}
    {{ weapon.name }}: {
        name: "{{ weapon.name }}",
        internalName: "{{ weapon.internal_name }}",
        nameLocale: {{weapon.name_index}},
        star: {{ weapon.star }},
        {% if weapon.icon_hash == "" -%}
        url: imageUrl("{{ weapon.internal_name }}"),
        {% else -%}
        url: newImageUrl("{{ weapon.icon_hash }}"),
        {%- endif %}
        type: "{{ weapon.t }}",

        {% if weapon.effect.is_some() %}
        effect: {{weapon.effect.unwrap()}},
        {% endif %}
        {% if weapon.configs.len() > 0 %}
        configs: [
            {% for config in weapon.configs %}
            {{ config|e("none") }},
            {% endfor %}
        ],
        {% else %}
        configs: null,
        {% endif %}
    },
{% endfor %}
}
