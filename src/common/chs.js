const _primaryTag = {
    "attack": "攻击力",
    "defend": "防御力",
    "life": "生命值"
};

const _secondaryTag = {
    "cureEffect": "治疗效果",
    "life1": "生命值",
    "life2": "%生命值",
    "attack1": "攻击力",
    "attack2": "%攻击力",
    "defend1": "防御力",
    "defend2": "%防御力",
    "critical": "暴击率",
    "bCritical": "重击暴击率",
    "criticalDamage": "暴击伤害",
    "elementalMastery": "元素精通",
    "recharge": "元素充能效率",
    "thunderBonus": "雷元素伤害加成",
    "fireBonus": "火元素伤害加成",
    "waterBonus": "水元素伤害加成",
    "iceBonus": "冰元素伤害加成",
    "windBonus": "风元素伤害加成",
    "rockBonus": "岩元素伤害加成",
    "physicalBonus": "物理伤害加成",
};

const _weapon = {
    "wufengjian": "无锋剑",
    "yinjian": "银剑",
    "antiejian": "暗铁剑",
    "lengren": "冷刃",
    "feitianyujian": "飞天御剑",
    "chihuyudao": "吃虎鱼刀",
    "limingshenjian": "黎明神剑",
    "lvxingjian": "旅行剑",
    "tiefengci": "铁蜂刺",
    "dijian": "笛剑",
    "xialilongyin": "匣里龙吟",
    "jilijian": "祭礼剑",
    "jianglinzhijian": "降临之剑",
    "xifengjian": "西风剑",
    "heiyanchangjian": "黑岩长剑",
    "heijian": "黑剑",
    "shizuozhanyan": "试作斩岩",
    "anxiangshanguang": "暗巷闪光",
    "zongshichangjian": "宗室长剑",
    "tiankongzhiren": "天空之刃",
    "fengyingjian": "风鹰剑",
    
    "xunliandajian": "训练大剑",
    "yongbingzhongjian": "佣兵重剑",
    "feitiandayujian": "飞天大御剑",
    "yilifuren": "以理服人",
    "muyulongxiedejian": "沐浴龙血的剑",
    "baitiedajian": "白铁大剑",
    "tieyingkuojian": "铁影阔剑",
    "xifengdajian": "西风大剑",
    "zhongjian": "钟剑",
    "yucai": "雨裁",
    "baiyingjian": "白影剑",
    "zongshidajian": "宗室大剑",
    "jilidajian": "祭礼大剑",
    "heiyanzhandao": "黑岩斩刀",
    "shizuoguhua": "试作古华",
    "chigujian": "螭骨剑",
    "tiankongzhiao": "天空之傲",
    "langdemolu": "狼的末路",

    "liegong": "猎弓",
    "liliandeliegong": "历练的猎弓",
    "fanqugong": "反曲弓",
    "shensheshouzhishi": "神射手之誓",
    "yayugong": "鸦羽弓",
    "xinshi": "信使",
    "heitangong": "黑檀弓",
    "tangong": "弹弓",
    "jiligong": "祭礼弓",
    "juexian": "绝弦",
    "cangcuiliegong": "苍翠猎弓",
    "xifengliegong": "西风猎弓",
    "gongcang": "弓藏",
    "ganglungong": "钢轮弓",
    "zongshichanggong": "宗室长弓",
    "heiyanzhangong": "黑岩战弓",
    "shizuodanyue": "试作澹月",
    "tiankongzhiyi": "天空之翼",
    "amosizhigong": "阿莫斯之弓",
    
    "xuetubiji": "学徒笔记",
    "koudaimodaoshu": "口袋魔导书",
    "jiajibaojue": "甲级宝珏",
    "modaoxulun": "魔导绪论",
    "taolongyingjietan": "讨龙英杰谭",
    "yishijiexingji": "异世界行记",
    "feiyufaqiu": "翡玉法球",
    "xifengmidian": "西风秘典",
    "xialiriyue": "匣里日月",
    "jilicanzhang": "祭礼残章",
    "heiyanfeiyu": "黑岩绯玉",
    "shizuojinpo": "试作金珀",
    "wanguozhuhaitupu": "万国诸海图谱",
    "zhaoxin": "昭心",
    "zongshimifalu": "宗室秘法录",
    "liulangyuezhang": "流浪乐章",
    "sifengyuandian": "四风原典",
    "tiankongzhijuan": "天空之卷",
    "chenshizhisuo": "尘世之锁",

    "xinshouchangqiang": "新手长枪",
    "tiejianqiang": "铁尖枪",
    "yuemao": "钺矛",
    "baiyingqiang": "白缨枪",
    "heiyingqiang": "黑缨枪",
    "shizuoxinglian": "试作星镰",
    "xifengchangqiang": "西风长枪",
    "heiyanciqiang": "黑岩刺枪",
    "juedouzhiqiang": "决斗之枪",
    "liuyuezhen": "流月针",
    "xialimiechen": "匣里灭辰",
    "tiankongzhiji": "天空之脊",
    "hepuyuan": "和璞鸢",
}

const _character = {
    "diluke": "迪卢克",
    "keli": "可莉",
    "anbo": "安柏",
    "bannite": "班尼特",
    "xiangling": "香菱",
    // "xinyan": "辛"
    
    "mona": "莫娜",
    "dadaliya": "达达利亚",
    "babala": "芭芭拉",
    "xingqiu": "行秋",

    "qin": "琴",
    "wendi": "温迪",
    "fengzhu": "风主",
    "shatang": "砂糖",

    "keqing": "刻晴",
    "beidou": "北斗",
    "lisha": "丽莎",
    "leize": "雷泽",
    "feixieer": "菲谢尔",

    "qiqi": "七七",
    "kaiya": "凯亚",
    "chongyun": "重云",
    "diaona": "迪奥娜",

    "yanzhu": "岩主",
    "nuoaier": "诺艾尔",
    "ningguang": "凝光",
}

export function chsPrimaryTag(tag) {
    return _primaryTag[tag] || _secondaryTag[tag];
}

export function chsSecondaryTag(tag) {
    return _secondaryTag[tag];
}

export function chsWeapon(tag) {
    return _weapon[tag] || null;
}

export function chsCharacter(tag) {
    return _character[tag] || null;
}