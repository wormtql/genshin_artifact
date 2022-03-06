let nextId = 2;

function initNextId(allAccounts) {
    for (const {id} of allAccounts) {
        nextId = Math.max(nextId, id);
    }
    nextId++;
}

const accounts = {
    namespaced: true,
    state: {
        currentAccountId: 1,
        allAccounts: [
            { id: 1, name: '默认' },
        ],
    },
    mutations: {
        set(state, payload) {
            state.currentAccountId = payload.currentAccountId;
            state.allAccounts = payload.allAccounts;
            initNextId(state.allAccounts);
        },

        setCurrentAccountId(state, { id }) {
            state.currentAccountId = id;
        },

        addAccount(state, { name }) {
            const account = {
                id: nextId++,
                name,
            }
            state.allAccounts = [...state.allAccounts, account];
        },

        deleteAccount(state, { id }) {
            const allAccounts = state.allAccounts;
            for (let i = 0, l = allAccounts.length; i < l; i++) {
                if (allAccounts[i].id === id) {
                    allAccounts.splice(i, 1);
                    break;
                }
            }
        },

        changeAccountName(state, { id, name }) {
            for (const account of state.allAccounts) {
                if (account.id === id) {
                    account.name = name;
                    break;
                }
            }
        }
    },
    getters: {
        currentAccountName(state) {
            const id = state.currentAccountId;
            for (const account of state.allAccounts) {
                if (account.id === id) {
                    return account.name;
                }
            }
        }
    }
};

export default accounts;
