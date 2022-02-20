import Vuex from "vuex";
import Vue from "vue";

import accounts from "./modules/accounts";
import artifacts from "./modules/artifact";
import presets from "./modules/presets";
import kumi from "./modules/artifactKumi";

/**
 * version 1 localStorage scheme:
 * mona_meta: { version: 1 }
 * mona_accounts: { currentAccountId: Number, allAccounts: [{ id: Number, name: String }, ...] }
 * mona_account_<name>: { artifacts: {...}, presets: {...}, kumiTree: {...} }
 */
const VERSION_STORAGE = 1;

Vue.use(Vuex);

let loadingAccountData = false;

const _store = new Vuex.Store({
    modules: {
        accounts,
        artifacts,
        presets,
        kumi,
    },
    actions: {
        loadAccountData({ commit, state }) {
            loadingAccountData = true;
            const key = `mona_account_${state.accounts.currentAccountId}`;
            const accountString = localStorage.getItem(key);
            const accountData = accountString ? JSON.parse(accountString) : {};
            commit('artifacts/set', accountData.artifacts);
            commit('presets/set', accountData.presets);
            commit('kumi/set', accountData.kumiTree);
            loadingAccountData = false;
        },
        changeAccount({ dispatch, commit }, { id }) {
            commit('accounts/setCurrentAccountId', { id });
            dispatch('loadAccountData');
        }
    }
});

// init from localStorage
const metaDataString = localStorage.getItem('mona_meta');
if (!metaDataString) {
    // load old data
    _store.commit('artifacts/oldInit');
    _store.commit('presets/oldInit');
    _store.commit('kumi/oldInit');

    const metaData = {
        version: VERSION_STORAGE,
    };
    localStorage.setItem('mona_meta', JSON.stringify(metaData));
} else {
    const metaData = JSON.parse(metaDataString);
    if (metaData.version !== VERSION_STORAGE) {
        // update local storage here
    } else {
        const payload = JSON.parse(localStorage.getItem('mona_accounts'));
        _store.commit('accounts/set', payload);
        _store.dispatch('loadAccountData');
    }
}

// watch accounts
_store.watch(
    state => state.accounts,
    newValue => {
        localStorage.setItem('mona_accounts', JSON.stringify(newValue));
    },
    {
        deep: true,
        immediate: true,
    }
);

// watch account state change
_store.watch(
    state => ({
        artifacts: state.artifacts,
        presets: state.presets,
        kumiTree: state.kumi,
        // artifacts: {
        //     flower: state.artifacts.flower,
        //     feather: state.artifacts.feather,
        //     sand: state.artifacts.sand,
        //     cup: state.artifacts.cup,
        //     head: state.artifacts.head,
        // },
        // presets: state.presets.presets,
        // kumiTree: {
        //     tree: state.kumi.tree,
        // },
    }),
    newValue => {
        if (loadingAccountData) {
            return;
        }
        const key = `mona_account_${_store.state.accounts.currentAccountId}`;
        localStorage.setItem(key, JSON.stringify(newValue));
    },
    {
        deep: true,
        immediate: true,
    },
);

export default _store;
