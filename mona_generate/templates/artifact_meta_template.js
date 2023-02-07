const template = "https://upload-bbs.mihoyo.com/game_record/genshin/equip/#.png"

const getIcon = name => template.replace("#", name)

export default {
    {% for a in artifacts %}
    "{{ a.name_mona }}": {
        eng: "{{ a.name_mona }}",
        name2: "{{ a.name }}",
        nameLocale: {{a.name_locale}},
        minStar: {{ a.min_star }},
        maxStar: {{ a.max_star }},
    {% if a.effect1.is_some() %}
        effect1: {{a.effect1.unwrap()}},
    {% endif %}
    {% if a.effect2.is_some() %}
        effect2: {{a.effect2.unwrap()}},
    {% endif %}
    {% if a.effect3.is_some() %}
        effect3: {{a.effect3.unwrap()}},
    {% endif %}
    {% if a.effect4.is_some() %}
        effect4: {{a.effect4.unwrap()}},
    {% endif %}
    {% if a.effect5.is_some() %}
        effect5: {{a.effect5.unwrap()}},
    {% endif %}

        {% if a.flower.is_some() %}
        flower: {
            text: {{a.flower.unwrap()}},
            url: getIcon("{{ a.flower_icon }}")
        },
        {% endif %}
        {% if a.feather.is_some() %}
        feather: {
            text: {{a.feather.unwrap()}},
            url: getIcon("{{ a.feather_icon }}")
        },
        {% endif %}
        {% if a.sand.is_some() %}
        sand: {
            text: {{a.sand.unwrap()}},
            url: getIcon("{{ a.sand_icon }}")
        },
        {% endif %}
        {% if a.goblet.is_some() %}
        cup: {
            text: {{a.goblet.unwrap()}},
            url: getIcon("{{ a.goblet_icon }}")
        },
        {% endif %}
        {% if a.head.is_some() %}
        head: {
            text: {{a.head.unwrap()}},
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
