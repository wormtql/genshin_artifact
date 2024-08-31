const { parse } = require("node-html-parser");
const axios = require("axios");

let url = process.argv[2];

function toStr(arr) {
    let temp = "[";
    for (let i = 0; i < arr.length; i++) {
        let item = arr[i];
        if (i === arr.length - 1) {
            temp = temp + item + "]";
        } else {
            temp = temp + item + ", ";
        }
    }
    return temp;
}

function removeZero(str) {
    return str.replace(/0*$/, "");
}

async function func() {
    response = await axios.get(url);

    // console.log(response.data)

    const regex = /<table .*skill_dmg_table.*>.*?<\/table>/g
    let matches = [...response.data.matchAll(regex)]
    console.log(matches.length)

    const root = parse(matches[0][0])
    // console.log(root)
    // let temp = ""
    // // 把<color="xxx"></color>预处理掉
    // for (let line of response.data.split(/\r?\n/)) {
    //     if (line.indexOf("<color") !== -1) {
    //         continue
    //     }
    //     if (line.indexOf("</color>") !== -1) {
    //         continue
    //     }
    //     temp = temp + line + "\n"
    // }

    // root = parse(response.data);

    // console.log(root.toString())
    // console.log(root.querySelectorAll("table.genshin_table.skill_dmg_table"))

    let tables = root.querySelectorAll("table.skill_dmg_table")
        .filter(item => {
        if (!item) {
            return false;
        }
        let text = item.rawText.toLowerCase();
        return text.indexOf("lv11") !== -1;
    });
    


    // console.log(tables);
    // return;

    for (let i = 0; i < tables.length; i++) {
        let table = tables[i];
        let rows = table.querySelectorAll("tr").filter(row => {
            let text = row.rawText.toLowerCase();
            return text.indexOf("%") !== -1;
        });

        for (let row of rows) {
            let cells = row.querySelectorAll("td").filter(cell => cell.rawText.indexOf("%") !== -1);
            
            let datas = [];
            for (let cell of cells) {
                // let matches = cell.innerHTML.match(/(\d+(\.\d+)?%)|(\+\d+)/g);
                let matches = cell.innerHTML.match(/(\d+(\.\d+)?%)/g);
                let data = [];
                for (let match of matches) {
                    let value;
                    if (match.indexOf("%") !== -1) {
                        value = (parseFloat(match) / 100).toFixed(4);
                        data.push(removeZero(value));
                    } else {
                        value = parseInt(match.slice(1));
                        data.push(value);
                    }
                }
                datas.push(data);
            }
            
            for (let i = 0; i < datas[0].length; i++) {
                let temp = datas.map(item => item[i]);
                console.log(toStr(temp));
            }
        }
    }

    // console.log(skill);
}

function inBroswer() {
    function removeZero(str) {
        return str.replace(/0*$/, "");
    }

    function toStr(arr) {
        let temp = "[";
        for (let i = 0; i < arr.length; i++) {
            let item = arr[i];
            if (i === arr.length - 1) {
                temp = temp + item + "]";
            } else {
                temp = temp + item + ", ";
            }
        }
        return temp;
    }

    const tables = [...document.querySelectorAll("table.skill_dmg_table")].filter(item => {
        if (!item) {
            return false;
        }
        return item.textContent.indexOf("Lv11") !== -1;
    });

    for (let i = 0; i < tables.length; i++) {
        let table = tables[i];
        let rows = [...table.querySelectorAll("tr")].filter(row => {
            return row.textContent.indexOf("%") !== -1
            // return text.indexOf("%") !== -1;
        });

        for (let row of rows) {
            let cells = [...row.querySelectorAll("td")].filter(cell => cell.textContent.indexOf("%") !== -1);

            let datas = [];
            for (let cell of cells) {
                // let matches = cell.innerHTML.match(/(\d+(\.\d+)?%)|(\+\d+)/g);
                let matches = cell.textContent.match(/(\d+(\.\d+)?%)/g);
                let data = [];
                for (let match of matches) {
                    let value;
                    if (match.indexOf("%") !== -1) {
                        value = (parseFloat(match) / 100).toFixed(4);
                        data.push(removeZero(value));
                    } else {
                        value = parseInt(match.slice(1));
                        data.push(value);
                    }
                }
                datas.push(data);
            }

            for (let i = 0; i < datas[0].length; i++) {
                let temp = datas.map(item => item[i]);
                console.log(toStr(temp));
                // console.log(temp)
            }
        }
    }
}

func();
