// Initializing contract
let sendUserContacts = [];
let acceptUserContacts = [];
let userInfo = {
  userId: "",
  cardsNumber: 0,
  date: "10000",
  cardList: []
};
async function InitContract() {
  // console.log("nearConfig", nearConfig);

  // Initializing connection to the NEAR DevNet.
  window.near = await nearlib.connect(
    Object.assign(
      {
        deps: { keyStore: new nearlib.keyStores.BrowserLocalStorageKeyStore() }
      },
      nearConfig
    )
  );

  // Initializing Wallet based Account. It can work with NEAR DevNet wallet that
  // is hosted at https://wallet.nearprotocol.com
  window.walletAccount = new nearlib.WalletAccount(window.near);

  // Getting the Account ID. If unauthorized yet, it's just empty string.
  window.accountId = window.walletAccount.getAccountId();

  // Initializing our contract APIs by contract name and configuration.
  window.contract = await near.loadContract(nearConfig.contractName, {
    // eslint-disable-line require-atomic-updates
    // NOTE: This configuration only needed while NEAR is still in development
    // View methods are read only. They don't modify the state, but usually return some value.
    viewMethods: ["list_card", "find_account_by_card", "list_contract_person"],
    // Change methods can modify the state. But you don't receive the returned value when called.
    changeMethods: [
      "create_card",
      "create_contract_person",
      "create_contract_person_for_sender"
    ],
    // Sender is the account ID to initialize transactions.
    sender: window.accountId
  });
}

// Using initialized contract
async function doWork() {
  // Based on whether you've authorized, checking which flow we should go.
  if (!window.walletAccount.isSignedIn()) {
    if (location.pathname !== "/") {
      location.href = "/";
    }
    // console.log("未登录");
  } else {
    if (location.pathname == "/") {
      if (GetUrlString("cardcode") !== "null") {
        relationContacts();
      } else {
        location.href = "./mycard.html";
      }
    }
  }
}

$("#login").click(() => {
  window.walletAccount.requestSignIn(
    // The contract name that would be authorized to be called by the user's account.
    window.nearConfig.contractName,
    // This is the app name. It can be anything.
    "Welcome to NEAR"
  );
});

window.nearInitPromise = InitContract()
  .then(doWork)
  .catch(console.error);

function GetUrlString(param) {
  var sValue = location.search.match(
    new RegExp("[?&]" + param + "=([^&]*)(&?)", "i")
  );
  return sValue ? decodeURI(sValue[1]) : decodeURI(sValue);
}

async function relationContacts() {
  let loading = $(document).dialog({
    type: "toast",
    infoIcon: "./assets/images/loading.gif",
    infoText: "正在加载中"
  });
  let cardcode = GetUrlString("cardcode");
  let sender = await listContractPerson(cardcode);
  // console.log(sender);
  await createContractPersonAciton(sender, cardcode)
    .then(res => {
      // console.log(res);
    })
    .catch(err => {
      loading.close();
      console.log("扫卡人创建联系人：" + err);
    });
  await createContractPersonForSenderAction(sender)
    .then(res => {
      loading.close();
      // console.log(res);
      location.href = "./mycard.html";
    })
    .catch(err => {
      loading.close();
      console.log("扫描人为发卡人创建联系人：" + err);
    });
}

async function listContractAction(id) {
  return window.contract.listContract({ contract: id });
}

async function createContractAction(sendUserInfo, sendUserId, acceptUserInfo) {
  return window.contract.createContract({
    contractInfo: sendUserInfo,
    newContract: sendUserId,
    newContractInfo: acceptUserInfo
  });
}

function clone(target) {
  if (typeof target === "object") {
    let cloneTarget = Array.isArray(target) ? [] : {};
    for (const key in target) {
      cloneTarget[key] = clone(target[key]);
    }
    return cloneTarget;
  } else {
    return target;
  }
}

function createContractPersonForSenderAction(sender, duration = 1000000) {
  // console.log("sender：" + sender);
  // console.log("duration：" + duration);
  return window.contract.create_contract_person_for_sender({
    sender: sender,
    duration: duration
  });
}

function createContractPersonAciton(sender, cardid, duration = 1000000) {
  // console.log("contact_person：" + sender);
  // console.log("card_id：" + cardid);
  // console.log("duration：" + duration);
  return window.contract.create_contract_person({
    contact_person: sender,
    card_id: cardid,
    duration: duration
  });
}

function listContractPerson(cardid) {
  return new Promise((resolve, reject) => {
    window.contract
      .find_account_by_card({
        card_id: cardid
      })
      .then(res => {
        resolve(res);
      })
      .catch(err => {
        loading.close();
        console.log(err);
      });
  });
}

window.addEventListener('pageshow', function(event) {
  if(event.persisted) {
   location.reload();
  } else { 
   if(sessionStorage.getItem('refresh') === 'true') {
    location.reload();
   }
  }
  sessionStorage.removeItem('refresh');
 });