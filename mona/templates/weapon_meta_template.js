{% for weapon in weapons %}
import {{ weapon.name }}_tn from "@image/weapons/{{ weapon.name }}_tn.png"
{% endfor %}

export default {
{% for weapon in weapons %}
    {{ weapon.name }}: {
        name: "{{ weapon.name }}",
        chs: "{{ weapon.chs }}",
        star: {{ weapon.star }},
        url: {{ weapon.name }}_tn,
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
