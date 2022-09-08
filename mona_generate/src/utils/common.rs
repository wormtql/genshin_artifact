use regex::Regex;

pub fn merge_affix_string(strings: &[&str]) -> String {
    if strings.len() == 0 {
        return String::new();
    }
    let mut split_items: Vec<Vec<&str>> = strings.iter()
        .map(|x| split_keep(*x, "<color=#([0-9]|[a-f]|[A-F])+>[^<]*</color>"))
        .collect();

    let mut result_vec = Vec::new();
    for i in 0..split_items[0].len() {
        if split_items[0][i].starts_with("<color") {
            let color = get_color_from_color_tag(split_items[0][i]);
            let mut content = String::new();
            for j in 0..split_items.len() {
                content.push_str(get_color_content(split_items[j][i]));
                content.push('-');
            }
            let tag = format!("<span style=\"color: {};\">{}</span>", color, &content[..content.len() - 1]);
            result_vec.push(tag);
        } else {
            result_vec.push(split_items[0][i].to_string());
        }
    }

    let mut result = String::new();
    for item in result_vec {
        result.push_str(&item);
    }

    result
}

pub fn split_keep<'a>(s: &'a str, pat: &str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    let p = Regex::new(pat).unwrap();
    for (index, matched) in s.match_indices(&p) {
        if last != index {
            result.push(&s[last..index]);
        }
        result.push(matched);
        last = index + matched.len();
    }
    if last < s.len() {
        result.push(&s[last..]);
    }
    result
}

pub fn get_color_from_color_tag(s: &str) -> &str {
    let re = Regex::new("#([0-9]|[a-f]|[A-F])+").unwrap();
    for cap in re.captures_iter(s) {
        return cap.get(0).unwrap().as_str()
    }

    return ""
}

pub fn get_color_content(s: &str) -> &str {
    let temp = split_keep(s, ">.+<");
    // println!("{:?}", temp);
    let x = temp[1];
    &x[1..x.len() - 1]
}

#[cfg(test)]
mod test {
    use crate::utils::common::split_keep;

    #[test]
    fn test1() {
        let s = "获得<color=#99FFFFFF>24%</color>所有元素伤害加成，并能获得「雾切之巴印」的威势。雾切之巴印：持有1/2/3层雾切之巴印时，获得<color=#99FFFFFF>16/32/56%</color>自己的元素类型的元素伤害加成。在下列情况下，角色将各获得1层雾切之巴印：普通攻击造成元素伤害时，持续5秒；施放元素爆发时，持续10秒；此外，角色元素能量低于100%时，将获得1层雾切之巴印，此雾切之巴印会在角色的元素能量充满时消失。每层雾切之巴印的持续时间独立计算。";
        let temp = split_keep(s, "<color=#([0-9]|[a-f]|[A-F])+>[^<]*</color>");
        println!("{:?}", temp);
        assert!(false);
    }
}