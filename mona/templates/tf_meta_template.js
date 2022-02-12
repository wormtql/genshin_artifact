{% for t in tfs %}
import {{ t.name }}_image from "@image/{{ t.badge_path }}"
{% endfor %}

export default {
    {% for t in tfs %}
    "{{ t.name }}": {
        name: "{{ t.name }}",
        chs: "{{ t.chs }}",
        description: "{{ t.description }}",
        tags: [
            {% for tag in t.tags %}
            "{{ tag }}",
            {% endfor %}
        ],
        "for": "{{ t.four }}",
        badge: {{ t.name }}_image,
        config: [
            {% for config in t.config %}
            {{ config|e("none") }},
            {% endfor %}
        ],
    },
    {% endfor %}
}
