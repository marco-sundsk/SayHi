const config = {
    nodeUrl: "https://rpc.nearprotocol.com",
    deps: {
        keyStore: new nearlib.keyStores.BrowserLocalStorageKeyStore()
    },
    networkId: 'default',
    contractName: CONTRACT_NAME,
    walletUrl: 'https://wallet.cn.nearprotocol.com',
};