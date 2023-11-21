const template = "https://upload-bbs.mihoyo.com/game_record/genshin/equip/#.png"
const newTemplate = "https://act-webstatic.mihoyo.com/hk4e/e20200928calculate/item_icon_uceddf/#.png"
const getIcon = name => template.replace("#", name)
const getHash = md5 => newTemplate.replace("#", md5)

export default {
    {% for a in artifacts %}
    "{{ a.name_mona }}": {
        eng: "{{ a.name_mona }}",
        name2: "{{ a.name }}",
        nameLocale: {{a.name_locale}},
        minStar: {{ a.min_star }},
        maxStar: {{ a.max_star }},
    {% if a.effect1.is_some() -%}
        effect1: {{a.effect1.unwrap()}},
    {%- endif %}
    {% if a.effect2.is_some() -%}
        effect2: {{a.effect2.unwrap()}},
    {%- endif %}
    {% if a.effect3.is_some() -%}
        effect3: {{a.effect3.unwrap()}},
    {%- endif %}
    {% if a.effect4.is_some() -%}
        effect4: {{a.effect4.unwrap()}},
    {%- endif %}
    {% if a.effect5.is_some() -%}
        effect5: {{a.effect5.unwrap()}},
    {%- endif %}

        {% if a.flower.is_some() -%}
        flower: {
            text: {{a.flower.unwrap()}},
            {% if a.flower_hash == "" -%}
            url: getIcon("{{ a.flower_icon }}")
            {% else -%}
            url: getHash("{{ a.flower_hash }}")
            {%- endif -%}
        },
        {%- endif %}
        {% if a.feather.is_some() -%}
        feather: {
            text: {{a.feather.unwrap()}},
            {% if a.feather_hash == "" -%}
            url: getIcon("{{ a.feather_icon }}")
            {% else -%}
            url: getHash("{{ a.feather_hash }}")
            {%- endif -%}
        },
        {%- endif %}
        {% if a.sand.is_some() -%}
        sand: {
            text: {{a.sand.unwrap()}},
            {% if a.sand_hash == "" -%}
            url: getIcon("{{ a.sand_icon }}")
            {% else -%}
            url: getHash("{{ a.sand_hash }}")
            {%- endif -%}
        },
        {%- endif %}
        {% if a.goblet.is_some() -%}
        cup: {
            text: {{a.goblet.unwrap()}},
            {% if a.goblet_hash == "" -%}
            url: getIcon("{{ a.goblet_icon }}")
            {% else -%}
            url: getHash("{{ a.goblet_hash }}")
            {%- endif -%}
        },
        {%- endif %}
        {% if a.head.is_some() -%}
        head: {
            text: {{a.head.unwrap()}},
            {% if a.head_hash == "" -%}
            url: getIcon("{{ a.head_icon }}")
            {% else -%}
            url: getHash("{{ a.head_hash }}")
            {%- endif -%}
        },
        {%- endif %}
        config4: [
            {% for config in a.config4 %}
            {{ config|e("none") }},
            {% endfor %}
        ],
    },
    {% endfor %}
}
