// generated file, do not edit
{% for p in pfs %}
import {{ p.name }}_image from "@image/{{ p.image }}"
{% endfor %}

export default {
{% for p in pfs %}
    "{{ p.name }}": {
        name: "{{ p.name }}",
        chs: "{{ p.chs }}",
        description: "{{ p.description }}",
        badge: {{ p.name }}_image,
        config: [
            {% for config in p.config %}
            {{ config|e("none") }},
            {% endfor %}
        ],
    },
{% endfor %}
}
