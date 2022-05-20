// generated file, do not edit
// {% for weapon in weapons %}
// import {{ weapon.name }}_tn from "@image/weapons/{{ weapon.name }}_tn"
// {% endfor %}

const template = "https://upload-bbs.mihoyo.com/game_record/genshin/equip/UI_EquipIcon_#.png"

const imageUrl = name => template.replace("#", name)

export default {
{% for weapon in weapons %}
    {{ weapon.name }}: {
        name: "{{ weapon.name }}",
        internalName: "{{ weapon.internal_name }}",
        chs: "{{ weapon.chs }}",
        star: {{ weapon.star }},
        // url: {{ weapon.name }}_tn,
        url: imageUrl("{{ weapon.internal_name }}"),
        type: "{{ weapon.t }}",

        {% if weapon.effect.len() > 0 %}
        effect: "{{ weapon.effect }}",
        {% else %}
        effect: null,
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
