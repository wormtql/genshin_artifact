// generated file, do not edit
// {% for a in artifacts %}
// {% if a.flower.len() > 0 %}
// import {{ a.name }}_flower from "@image/artifacts/{{ a.name }}_flower"
// {% endif %}
// {% if a.feather.len() > 0 %}
// import {{ a.name }}_feather from "@image/artifacts/{{ a.name }}_feather"
// {% endif %}
// {% if a.sand.len() > 0 %}
// import {{ a.name }}_sand from "@image/artifacts/{{ a.name }}_sand"
// {% endif %}
// {% if a.goblet.len() > 0 %}
// import {{ a.name }}_goblet from "@image/artifacts/{{ a.name }}_goblet"
// {% endif %}
// {% if a.head.len() > 0 %}
// import {{ a.name }}_head from "@image/artifacts/{{ a.name }}_head"
// {% endif %}
// {% endfor %}

const template = "https://upload-bbs.mihoyo.com/game_record/genshin/equip/#.png"

const getIcon = name => template.replace("#", name)

export default {
    {% for a in artifacts %}
    "{{ a.name_mona }}": {
        eng: "{{ a.name_mona }}",
        chs: "{{ a.chs }}",
        name2: "{{ a.name }}",
        minStar: {{ a.min_star }},
        maxStar: {{ a.max_star }},
        effect2: "{{ a.effect2 }}",
        effect4: "{{ a.effect4 }}",
        {% if a.flower.len() > 0 %}
        flower: {
            chs: "{{ a.flower }}",
            // url: {{ a.name }}_flower,
            url: getIcon("{{ a.flower_icon }}")
        },
        {% endif %}
        {% if a.feather.len() > 0 %}
        feather: {
            chs: "{{ a.feather }}",
            // url: {{ a.name }}_feather,
            url: getIcon("{{ a.feather_icon }}")
        },
        {% endif %}
        {% if a.sand.len() > 0 %}
        sand: {
            chs: "{{ a.sand }}",
            // url: {{ a.name }}_sand,
            url: getIcon("{{ a.sand_icon }}")
        },
        {% endif %}
        {% if a.goblet.len() > 0 %}
        cup: {
            chs: "{{ a.goblet }}",
            // url: {{ a.name }}_goblet,
            url: getIcon("{{ a.goblet_icon }}")
        },
        {% endif %}
        {% if a.head.len() > 0 %}
        head: {
            chs: "{{ a.head }}",
            // url: {{ a.name }}_head,
            url: getIcon("{{ a.head_icon }}")
        },
        {% endif %}
        config4: [
            {% for config in a.config4 %}
            {{ config|e("none") }},
            {% endfor %}
        ],
    },
    {% endfor %}
}
