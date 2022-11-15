import { computed, reactive, ref, watch, toRaw } from "vue"
import { deepCopy } from "@/utils/common"
import backend from "../backend"
import { useArtifactStore, watchContent as artifactWatchContent } from "./artifact"
import { useKumiStore, watchContent as kumiWatchContent } from "./kumi"
import { usePresetStore, watchContent as presetWatchContent } from "./preset"

interface Account {
    id: number;
    name: string;
}

function createAccountStore() {
    const syncStatus = ref<'no sync' | 'syncing' | 'synced'>('no sync')

    const currentAccountId = ref<number>(1)
    const allAccounts = reactive<Account[]>([{
        id: 1,
        name: 'default'
    }])
    let nextId = 2;

    function initNextId() {
        nextId = 0;
        for (const { id } of allAccounts) {
            nextId = Math.max(nextId, id);
        }
        nextId++;
    }

    function init(payload: { currentAccountId: number, allAccounts: Account[] } | null) {
        if (payload) {
            currentAccountId.value = payload.currentAccountId
            allAccounts.splice(0, allAccounts.length, ...payload.allAccounts)
            initNextId()
        } else {
            currentAccountId.value = 1
            allAccounts.splice(0, allAccounts.length, {
                id: 1,
                name: 'default'
            })
        }
    }

    function addAccount(name: string) {
        allAccounts.push({
            id: nextId++,
            name,
        })
    }

    function deleteAccount(id: number) {
        for (let i = 0; i < allAccounts.length; i++) {
            if (allAccounts[i].id === id) {
                allAccounts.splice(i, 1)
                break
            }
        }
    }

    function changeAccountName(id: number, name: string) {
        for (const account of allAccounts) {
            if (account.id === id) {
                account.name = name;
                break;
            }
        }
    }

    const currentAccountName = computed(() => {
        return allAccounts.find(a => a.id === currentAccountId.value)?.name
    })

    return {
        syncStatus,

        currentAccountId,
        allAccounts,
        currentAccountName,

        init,
        addAccount,
        deleteAccount,
        changeAccountName,
    }
}

const accountStore = createAccountStore()

export const useAccountStore = () => accountStore


const artifactStore = useArtifactStore()
const presetStore = usePresetStore()
const kumiStore = useKumiStore()

let loadingAccountData = false

function nextTick() {
    return new Promise((resolve) => {
        setTimeout(resolve, 0)
    })
}

async function loadAccountData() {
    // console.log('start to load')
    loadingAccountData = true
    const id = accountStore.currentAccountId.value
    const artKey = `mona_account_artifacts_${id}`
    artifactStore.init(await backend.getItem(artKey))
    const presetKey = `mona_account_presets_${id}`
    presetStore.init(await backend.getItem(presetKey))
    const kumiKey = `mona_account_kumi_${id}`
    kumiStore.init(await backend.getItem(kumiKey))
    await nextTick()
    loadingAccountData = false
    // console.log('loaded')
}

export async function changeAccount(id: number) {
    await backend.allReady()
    accountStore.currentAccountId.value = id
    await loadAccountData()
}

export async function deleteAccount(id: number) {
    if (id === accountStore.currentAccountId.value) {
        // this should not happen, but add a guard here
        return
    }
    accountStore.deleteAccount(id)
    const artKey = `mona_account_artifacts_${id}`
    await backend.removeItem(artKey)
    const presetKey = `mona_account_presets_${id}`
    await backend.removeItem(presetKey)
    const kumiKey = `mona_account_kumi_${id}`
    await backend.removeItem(kumiKey)
}

export async function reload() {
    accountStore.init(await backend.getItem('mona_accounts') as any)
    await loadAccountData()
}

async function initBackendFromLocalStorage() {
    backend.setItem('mona_accounts', deepCopy(accountWatchContent()))
    const artString = localStorage.getItem('artifacts')
    await backend.setItem('mona_account_artifacts_1', artString && JSON.parse(artString))
    const kumiString = localStorage.getItem('kumi2')
    await backend.setItem('mona_account_kumi_1', kumiString && JSON.parse(kumiString))
    const presetString = localStorage.getItem('presets5')
    await backend.setItem('mona_account_presets_1', presetString && JSON.parse(presetString))
}

/**
 * localStorage scheme (version 1):
 * mona_meta: { version: 1 }
 * mona_accounts: { currentAccountId: Number, allAccounts: [{ id: Number, name: String }, ...] }
 * mona_account_artifacts_<id>: { flower: {...}, ... }
 * mona_account_presets_<id>: { ... }
 * mona_account_kumi_<id>: { ... }
 */
const VERSION_STORAGE = 1

interface MonaMeta {
    version: number
}

async function init_store() {
    let metaData = await backend.getItem('mona_meta') as MonaMeta
    if (!metaData) {
        // load old data
        // _store.commit('artifacts/oldInit');
        // _store.commit('presets/oldInit');
        // _store.commit('kumi/oldInit');

        metaData = {
            version: VERSION_STORAGE,
        }
        await backend.setItem('mona_meta', metaData)

        // copy data from localStorage to backend
        await initBackendFromLocalStorage()
        await reload()
    } else {
        if (metaData.version !== VERSION_STORAGE) {
            // update local storage here
        } else {
            await reload()
        }
    }
}

init_store()

backend.on('cancelFileBackend', () => {
    accountStore.syncStatus.value = 'no sync'
})

function updateCurrentAccount(type: string, value: any) {
    if (loadingAccountData) {
        return
    }
    // console.log('update', type, loadingAccountData)
    const key = `mona_account_${type}_${accountStore.currentAccountId.value}`
    backend.setItem(key, deepCopy(value))
    if (accountStore.syncStatus.value === 'synced') {
        accountStore.syncStatus.value = 'syncing'
        backend.allReady().then(() => accountStore.syncStatus.value = 'synced')
    }
}

watch(artifactWatchContent, value => updateCurrentAccount('artifacts', value), { deep: true })
watch(kumiWatchContent, value => updateCurrentAccount('kumi', value), { deep: true })
watch(presetWatchContent, value => updateCurrentAccount('presets', value), { deep: true })

function accountWatchContent() {
    return {
        currentAccountId: accountStore.currentAccountId.value,
        allAccounts: accountStore.allAccounts,
    }
}

watch(accountWatchContent, value => {
    backend.setItem('mona_accounts', deepCopy(value))
}, { deep: true })
