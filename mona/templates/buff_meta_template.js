// generated file, do not edit
{% for b in buffs %}
import {{ b.name }}_image from "@image/{{ b.image }}"
{% endfor %}

export default {
    {% for b in buffs %}
    "{{ b.name }}": {
        name: "{{ b.name }}",
        chs: "{{ b.chs }}",
        badge: {{ b.name }}_image,
        genre: "{{ b.genre }}",
        description: "{{ b.description }}",
        config: [
            {% for config in b.config %}
            {{ config|e("none") }},
            {% endfor %}
        ],
    },
    {% endfor %}
}