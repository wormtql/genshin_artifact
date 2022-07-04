// generated file, do not edit
{% for b in buffs %}
{% if b.image_type == "misc" %}
import {{ b.name }}_image from "@image/{{ b.image }}"
{% endif %}
{% endfor %}

const template = "https://upload-bbs.mihoyo.com/game_record/genshin/character_icon/UI_AvatarIcon_#.png"
const getImage = name => template.replace("#", name)

export default {
    {% for b in buffs %}
    "{{ b.name }}": {
        name: "{{ b.name }}",
        // chs: "{{ b.chs }}",
        {% if b.image_type == "character" %}
        badge: getImage("{{ b.character_internal_name }}"),
        {% else %}
        badge: {{ b.name }}_image,
        {% endif %}
        genre: "{{ b.genre }}",
        // description: "{{ b.description }}",
        config: [
            {% for config in b.config %}
            {{ config|e("none") }},
            {% endfor %}
        ],
    },
    {% endfor %}
}