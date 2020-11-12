import { supportedWeapons } from "genshin_panel";
import { chsWeapon } from "@/common/chs";


let sword = new Set([
    "wufengjian",
    "yinjian",
    "antiejian",
    "lengren",
    "feitianyujian",
    "chihuyudao",
    "limingshenjian",
    "lvxingjian",
    "tiefengci",
    "dijian",
    "xialilongyin",
    "jilijian",
    "heijian",
    "shizuozhanyan",
    "jianglinzhijian",
    "xifengjian",
    "heiyanchangjian",
    "anxiangshanguang",
    "zongshichangjian",
    "tiankongzhiren",
    "fengyingjian",
]);

let sword2 = new Set([
    "xunliandajian",
    "yongbingzhongjian",
    "feitiandayujian",
    "yilifuren",
    "muyulongxiedejian",
    "baitiedajian",
    "tieyingkuojian",
    "xifengdajian",
    "zhongjian",
    "yucai",
    "zongshidajian",
    "baiyingjian",
    "heiyanzhandao",
    "shizuoguhua",
    "jilidajian",
    "chigujian",
    "tiankongzhiao",
    "langdemolu",
]);

let stick = new Set([
    "xinshouchangqiang",
    "tiejianqiang",
    "yuemao",
    "baiyingqiang",
    "heiyingqiang",
    "heiyanciqiang",
    "shizuoxinglian",
    "xifengchangqiang",
    "liuyuezhen",
    "xialimiechen",
    "juedouzhiqiang",
    "tiankongzhiji",
    "hepuyuan",
]);

let book = new Set([
    "xuetubiji",
    "koudaimodaoshu",
    "jiajibaojue",
    "feiyufaqiu",
    "yishijiexingji",
    "taolongyingjietan",
    "modaoxulun",
    "xifengmidian",
    "liulangyuezhang",
    "shizuojinpo",
    "zongshimifalu",
    "zhaoxin",
    "wanguozhuhaitupu",
    "xialiriyue",
    "heiyanfeiyu",
    "jilicanzhang",
    "chenshizhisuo",
    "tiankongzhijuan",
    "sifengyuandian",
]);

let bow = new Set([
    "liegong",
    "liliandeliegong",
    "yayugong",
    "heitangong",
    "shensheshouzhishi",
    "fanqugong",
    "tangong",
    "xinshi",
    "gongcang",
    "zongshichanggong",
    "shizuodanyue",
    "xifengliegong",
    "heiyanzhangong",
    "ganglungong",
    "juexian",
    "cangcuiliegong",
    "jiligong",
    "gangcang",
    "amosizhigong",
    "tiankongzhiyi",
]);

function getType(name) {
    if (sword.has(name)) {
        return "sword";
    }
    if (sword2.has(name)) {
        return "sword2";
    }
    if (stick.has(name)) {
        return "stick";
    }
    if (book.has(name)) {
        return "book";
    }
    if (bow.has(name)) {
        return "bow";
    }
}

let _preset = {
    "sword": {
        "chs": "单手剑",
        "weapons": {

        }
    },
    "sword2": {
        "chs": "双手剑",
        "weapons": {}
    },
    "stick": {
        "chs": "长柄武器",
        "weapons": {}
    },
    "book": {
        "chs": "法器",
        "weapons": {}
    },
    "bow": {
        "chs": "弓",
        "weapons": {}
    }
};

const support = supportedWeapons();
// window.console.log(support);
for (let i = 0; i < support.length; i++) {
    let item = support[i];
    let temp = item.split("-");

    let chs = chsWeapon(temp[0]);
    let type = getType(temp[0]);
    if (type === undefined) {
        window.console.log(temp[0]);
    }
    let weapons = _preset[type].weapons;

    let s1 = chs;
    let s2 = temp[1] + "级";
    let s3 = temp[2] === 0 ? "未突破" : "突破";

    if (chs === null) {
        continue;
    }

    if (weapons[temp[0]]) {
        weapons[temp[0]].presets.push({
            value: item,
            title: s1 + s2 + s3,
        })
    } else {
        weapons[temp[0]] = {
            chs,
            presets: [
                {
                    value: item,
                    title: s1 + s2 + s3,
                }
            ]
        }
    }
}

// window.console.log(_preset);
Object.freeze(_preset);
export const preset = _preset;