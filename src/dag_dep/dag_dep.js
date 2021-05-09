import Vue from "vue";


const depScope = {};

export default class DagDep {
    constructor() {
        // this._vm = new Vue({
        //     data() {
        //         return {
        //             _state: {}
        //         }
        //     }
        // });
        this.dep = {};
    }

    static scope(name) {
        if (Object.prototype.hasOwnProperty.call(depScope, name)) {
            return depScope[name];
        } else {
            let newScope = new DagDep();

            depScope[name] = newScope;
            return newScope;
        }
    }

    dep(name, value) {
        if (typeof value === "function") {
            Object.defineProperty(this.dep, name, {
                get: () => value()
            })
        }
    }

    get(name) {
        value = this.dep[name];
        return value();
    }
}