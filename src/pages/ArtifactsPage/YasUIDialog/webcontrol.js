/* eslint-disable @typescript-eslint/no-explicit-any */
/* eslint-disable max-params */
import { EventEmitter } from 'events';
import axios from 'axios';
export class CocogoatWebControl {
    constructor(_port = 32333) {
        this.port = 32333;
        this.token = '';
        this.hwnd = 0;
        this.version = '';
        this.ev = new EventEmitter();
        this.client = axios.create({
            baseURL: `http://localhost:${this.port}`,
        });
        this.MOUSEEVENTF_ABSOLUTE = 0x8000;
        this.MOUSEEVENTF_LEFTDOWN = 0x0002;
        this.MOUSEEVENTF_LEFTUP = 0x0004;
        this.MOUSEEVENTF_MIDDLEDOWN = 0x0020;
        this.MOUSEEVENTF_MIDDLEUP = 0x0040;
        this.MOUSEEVENTF_MOVE = 0x0001;
        this.MOUSEEVENTF_RIGHTDOWN = 0x0008;
        this.MOUSEEVENTF_RIGHTUP = 0x0010;
        this.MOUSEEVENTF_WHEEL = 0x0800;
        this.MOUSEEVENTF_XDOWN = 0x0080;
        this.MOUSEEVENTF_XUP = 0x0100;
        this.MOUSEEVENTF_HWHEEL = 0x01000;
        this.port = _port;
        this.client.interceptors.request.use((request) => {
            if (this.token) {
                request.headers['Authorization'] = `Bearer ${this.token}`;
            }
            if (request.headers['Content-Type'] === '') {
                delete request.headers['Content-Type'];
            }
            return request;
        });
        this.client.interceptors.response.use(undefined, (e) => {
            const er = e;
            if (er.status === 410) {
                console.warn('Control stopped by user');
                return new Error('ECANCEL');
            }
            return e;
        });
    }
    async check() {
        try {
            const { data } = await this.client.get(
                '/',
                {},
                {
                    timeout: 800,
                },
            );
            this.version = data.version;
            return true;
        } catch (e) {
            return false;
        }
    }
    async authorize() {
        if (this.ws) {
            return true;
        }
        try {
            const res = await this.client.post('/token');
            if (res instanceof Error) throw res;
            const data = res.data;
            this.token = data.token;
            this.hwnd = data.hwnd || 0;
            const ws = new WebSocket(`ws://localhost:${this.port}/ws/${this.token}`);
            ws.onmessage = (e) => {
                const data = JSON.parse(e.data);
                this.ev.emit(data.id || data.action, data.data);
            };
            ws.onclose = () => {
                this.ws = undefined;
            };
            await new Promise((resolve) => {
                ws.onopen = resolve;
            });
            this.ws = ws;
            return true;
        } catch (e) {
            if (e.response && e.response.status === 401) {
                return false;
            }
            throw e;
        }
    }
    wsInvoke(method, path, querystring, body) {
        if (!this.ws) throw new Error('WebSocket not connected');
        const uri = new URL(path, `http://localhost:${this.port}`);
        if (querystring) {
            uri.searchParams.append(...Object.entries(querystring));
        }
        const realpath = uri.pathname + uri.search;
        const id = Math.round(Date.now() * 1000 + Math.random() * 1000).toString(16);
        const reqjson = {
            id,
            action: 'api',
            data: {
                url: realpath,
                method,
                body: body ? JSON.stringify(body) : undefined,
            },
        };
        const resp = new Promise((resolve) => {
            this.ev.on(id, resolve);
        });
        this.ws.send(JSON.stringify(reqjson));
        return resp;
    }
    async listWindows() {
        return (await this.wsInvoke('GET', '/api/windows')).body;
    }
    async activateWindow(id) {
        return await this.wsInvoke('PATCH', '/api/windows/' + id);
    }
}
