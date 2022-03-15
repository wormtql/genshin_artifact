{% for c in characters %}
import {{ c.name }}_card from "@image/characters/{{ c.name }}_card"
import {{ c.name }}_avatar from "@image/characters/{{ c.name }}_avatar"
import {{ c.name }}_splash from "@image/characters/{{ c.name }}_splash"
{% endfor %}


export default {
    {% for c in characters %}
    {{ c.name }}: {
        name: "{{ c.name }}",
        chs: "{{ c.chs }}",
        element: "{{ c.element }}",
        weapon: "{{ c.weapon }}",
        star: {{ c.star }},
        card: {{ c.name }}_card,
        avatar: {{ c.name }}_avatar,
        splash: {{ c.name }}_splash,
        skillName1: "{{ c.skill1_name }}",
        skillName2: "{{ c.skill2_name }}",
        skillName3: "{{ c.skill3_name }}",
        skillMap1: [
            {% for s in c.skill_map1 %}
            { index: {{ s.index }}, chs: "{{ s.chs }}" },
            {% endfor %}
        ],
        skillMap2: [
            {% for s in c.skill_map2 %}
            { index: {{ s.index }}, chs: "{{ s.chs }}" },
            {% endfor %}
        ],
        skillMap3: [
            {% for s in c.skill_map3 %}
            { index: {{ s.index }}, chs: "{{ s.chs }}" },
            {% endfor %}
        ],
        config: [
            {% for config in c.config %}
            {{ config|e("none") }},
            {% endfor %}
        ],
        configSkill: [
            {% for config in c.config_skill %}
            {{ config|e("none") }},
            {% endfor %}
        ],
    },
    {% endfor %}
}
