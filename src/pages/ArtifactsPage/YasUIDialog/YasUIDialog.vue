<template>
    <el-dialog
        title="使用YAS WebUI扫描圣遗物"
        :model-value="visible"
        width="80%"
        @update:modelValue="$emit('update:visible', $event)"
    >
        <div v-if="win7">
            <el-alert type="warning" center show-icon :closable="false" title="YAS暂不支持Windows 7系统">
                您可以换用天目以扫描圣遗物。
            </el-alert>
        </div>
        <div v-else :class="$style.webcapturer">
            <client-comp v-if="visible && !connected" :control="control" @done="onConnected" />
            <div v-if="connected" :class="$style.uimain">
                <div v-if="step === 2">
                    <el-progress type="circle" :percentage="statusProgress"></el-progress>
                    <br />
                    <div class="progress-text">
                        {{ statusText }}
                    </div>
                </div>
                <div v-if="step === 3">
                    <el-form>
                        <el-form-item label="最小等级">
                            <el-input-number v-model="form['min-level']" placeholder="请输入最小等级" />
                        </el-form-item>
                        <el-form-item label="最小星级">
                            <el-input-number v-model="form['min-star']" placeholder="请输入最小星级" />
                        </el-form-item>
                        <el-form-item label="扫描数量">
                            <el-input-number v-model="form['number']" placeholder="请输入扫描数量" />
                            <br />
                            <small>0为无限制</small>
                        </el-form-item>
                        <div>
                            <el-checkbox v-model="importDeleteUnseen" style="margin-top: 12px"
                                >删除不存在的圣遗物</el-checkbox
                            >
                            <el-checkbox v-model="importBackupKumiDir" style="margin-top: 12px"
                                >备份“游戏中导入”收藏夹</el-checkbox>
                        </div>
                        <el-button class="start-btn start-gray" @click="startScan">
                            <div class="l">
                                <i-ep-check />
                            </div>
                            <div class="m">
                                <div class="t">开始扫描</div>
                                <div class="d">点击后YAS将自动运行...</div>
                            </div>
                            <div class="r">
                                <i-ep-arrow-right />
                            </div>
                        </el-button>
                    </el-form>
                </div>
                <div v-if="step === 4" style="width: 100%">
                    <textarea ref="outputArea" class="output" :value="output.join('\n')" readonly />
                </div>
            </div>
        </div>
    </el-dialog>
</template>

<script>
import ClientComp from './Client';
import { CocogoatWebControl } from './webcontrol';
import { importMonaJson } from '@util/artifacts';
const delay = (ms) => new Promise((resolve) => setTimeout(resolve, ms));
export default {
    components: {
        ClientComp,
    },
    props: {
        visible: {
            type: Boolean,
        },
    },
    emits: ['update:visible'],
    data() {
        return {
            control: new CocogoatWebControl(),
            connected: false,
            step: 1,
            statusText: '正在检查更新...',
            statusProgress: 0,
            form: {
                'min-level': 0,
                'min-star': 5,
                number: 0,
            },
            output: [],
            importDeleteUnseen: false,
            importBackupKumiDir: false,
            hwnd: -1,
            win7: navigator.userAgent.includes('NT 6'),
        };
    },
    watch: {
        visible(val) {
            if (val) {
                this.connected = !!this.control.ws;
                this.step = this.connected ? 2 : 1;
            }
        },
        step(val) {
            if (val === 2) {
                this.loopForUpdate();
            }
        },
        output: {
            async handler(val) {
                await this.$nextTick();
                this.$refs.outputArea.scrollTop = this.$refs.outputArea.scrollHeight;
            },
            deep: true,
        },
    },
    methods: {
        onConnected(hwnd) {
            this.connected = true;
            this.step = 2;
            this.hwnd = hwnd;
        },
        async startScan() {
            this.step++;
            this.control.ev.removeAllListeners('yas');
            this.control.ev.removeAllListeners('yas-output');
            this.output = [];
            this.control.ev.on('yas', async (data) => {
                this.output.push(`[YAS STAT] ${data}`);
                if (data === 'exit') {
                    this.output.push(`[YAS PROG] 正在获取圣遗物数据...`);
                    const res = await this.control.client.get('/api/yas');
                    if (res instanceof Error) {
                        this.output.push(`[YAS ERR!] 获取圣遗物数据失败，错误详情如下：\n${res.message}`);
                    } else {
                        const yasJson = res.data;
                        try {
                            importMonaJson(yasJson, this.importDeleteUnseen, this.importBackupKumiDir);
                            this.output.push(`[YAS DONE] 圣遗物扫描完成，您可以关闭本窗口和辅助插件。`);
                        } catch (e) {
                            this.output.push(`[YAS ERR!] 导入圣遗物数据失败，错误详情如下：\n${res.message}`);
                        }
                    }
                }
            });
            this.control.ev.on('yas-output', (data) => {
                this.output.push(data);
            });
            if (this.hwnd > -1) {
                await this.control.activateWindow(this.hwnd);
            }
            this.control.wsInvoke('POST', '/api/yas', '', {
                argv: Object.keys(this.form).reduce((acc, key) => {
                    if (this.form[key] === 0) return acc;
                    acc += `--${key} ${this.form[key]} `;
                    return acc;
                }, ''),
            });
        },
        async loopForUpdate() {
            const msglist = {
                started: '正在检查更新...[0]',
                prechecking: '正在检查更新...[1]',
                prehashing: '正在检查更新...[2]',
                prechecksum: '正在检查更新...[3]',
                downloading: '正在下载更新...',
                checksum: '正在校验更新...',
                done: '更新完成',
                noupdate: '没有更新',
            };
            const res = await this.control.wsInvoke('POST', '/api/upgrade/yas');
            if (res.status === 201) {
                while (1) {
                    const {
                        body: { total, downloaded, msg },
                    } = await this.control.wsInvoke('GET', '/api/upgrade/yas');
                    if (msg === 'done' || msg === 'noupdate') {
                        this.step++;
                        this.statusText = msglist[msg];
                        this.statusProgress = 100;
                        break;
                    } else {
                        const percent = Math.floor((downloaded / total) * 100) || 0;
                        this.statusProgress = percent;
                        if (msglist[msg]) {
                            this.statusText = msglist[msg] + '  ' + percent + '%';
                        } else {
                            this.statusText = 'ERROR: ' + msg;
                            break;
                        }
                    }
                    await delay(80);
                }
            }
        },
    },
};
</script>

<style lang="scss" module>
.webcapturer {
    text-align: center;
    :global {
        .output {
            width: 100%;
            box-sizing: border-box;
            font-family: Consolas, Monaco, Microsoft Yahei, monospace;
            color: #666;
            position: absolute;
            top: 65px;
            left: 0;
            right: 0;
            bottom: 0;
            height: 100%;
            border: 0;
            padding: 20px;
            border-top: 1px solid #eee;
            resize: none;
            overflow-y: scroll;
        }
        .start-btn {
            margin-top: 10px;
            height: 60px;
            text-align: left;
            font-size: 16px;
            transition: all 0.3s;
            width: 490px;
            max-width: 95%;
            box-sizing: border-box;
            &:hover {
                transform: translateY(-5px);
            }
            &.start-gray {
                --el-button-hover-text-color: #333;
                --el-button-hover-border-color: #aaa;
                --el-button-hover-bg-color: #fafafa;
            }
            & > span {
                display: flex;
                width: 100%;
                height: 100%;
                justify-content: flex-start;
            }

            .r {
                opacity: 0.8;
                svg {
                    font-size: 18px;
                    padding-top: 6px;
                }
            }

            .m {
                flex-grow: 1;
            }

            .l svg {
                width: 40px;
                height: 30px;
                padding-right: 10px;
                font-size: 30px;
                padding-top: 5px;
            }

            .d {
                font-size: 12px;
                margin-top: 3px;
                opacity: 0.8;
            }
        }
    }
}
.uimain {
    min-height: calc(10vh + 250px);
    display: flex;
    align-items: center;
    justify-content: center;
}
</style>
