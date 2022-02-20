import Vuex from "vuex";
import Vue from "vue";

import accounts from "./modules/accounts";
import artifacts from "./modules/artifact";
import presets from "./modules/presets";
import kumi from "./modules/artifactKumi";

/**
 * localStorage scheme (version 1):
 * mona_meta: { version: 1 }
 * mona_accounts: { currentAccountId: Number, allAccounts: [{ id: Number, name: String }, ...] }
 * mona_account_artifacts_<id>: { flower: {...}, ... }
 * mona_account_presets_<id>: { ... }
 * mona_account_kumi_<id>: { ... }
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
            const id = state.accounts.currentAccountId;
            const ArtKey = `mona_account_artifacts_${id}`;
            commit('artifacts/set', JSON.parse(localStorage.getItem(ArtKey)));
            const PresetKey = `mona_account_presets_${id}`;
            commit('presets/set', JSON.parse(localStorage.getItem(PresetKey)));
            const KumiKey = `mona_account_kumi_${id}`;
            commit('kumi/set', JSON.parse(localStorage.getItem(KumiKey)));
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
    // _store.commit('presets/oldInit');
    // _store.commit('kumi/oldInit');

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

// watch accounts change
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

// watch artifacts change
_store.watch(
    state => ({
        flower: state.artifacts.flower,
        feather: state.artifacts.feather,
        sand: state.artifacts.sand,
        cup: state.artifacts.cup,
        head: state.artifacts.head,
    }),
    newValue => {
        if (loadingAccountData) {
            return;
        }
        const key = `mona_account_artifacts_${_store.state.accounts.currentAccountId}`;
        localStorage.setItem(key, JSON.stringify(newValue));
    },
    {
        deep: true,
        immediate: true,
    },
);

// watch presets change
_store.watch(
    state => state.presets.presets,
    newValue => {
        if (loadingAccountData) {
            return;
        }
        const key = `mona_account_presets_${_store.state.accounts.currentAccountId}`;
        localStorage.setItem(key, JSON.stringify(newValue));
    },
    {
        deep: true,
        immediate: true,
    }
)

// watch kumi change
_store.watch(
    state => state.kumi,
    newValue => {
        if (loadingAccountData) {
            return;
        }
        const key = `mona_account_kumi_${_store.state.accounts.currentAccountId}`;
        localStorage.setItem(key, JSON.stringify(newValue));
    },
    {
        deep: true,
        immediate: true,
    }
);

export default _store;
