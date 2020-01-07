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
  console.log("nearConfig", nearConfig);

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
    viewMethods: ["welcome", "listTemplate", "hello"],
    // Change methods can modify the state. But you don't receive the returned value when called.
    changeMethods: [
      "createTemplate",
      "createCard",
      "listCard",
      "getCardInfo",
      "listContract",
      "createContract"
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
    console.log("未登录");
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

function relationContacts() {
  let loading = $(document).dialog({
    type: "toast",
    infoIcon: "./assets/images/loading.gif",
    infoText: "正在加载中"
  });
  let acceptUserId = window.accountId;
  // console.log(acceptUserId);
  let cardcode = GetUrlString("cardcode");
  // console.log(cardcode);

  window.contract.getCardInfo({ cardCode: cardcode }).then(res => {
    let cardInfo = JSON.parse(res);
    let sendUserId = cardInfo.cardUser;
    listContractAction(sendUserId)
      .then(res => {
        // debugger;
        if (res !== "没有联系人") {
          // console.log("发送卡片的人的联系人");
          // console.log(JSON.parse(res));
          acceptUserContacts = JSON.parse(res);
        }
        let isHasAcceptUser = acceptUserContacts.some(ele => {
          if (ele.userId == acceptUserId) {
            return true;
          } else {
            return false;
          }
        });
        if (isHasAcceptUser) {
        } else {
          let acceptUserInfo = clone(userInfo);
          acceptUserInfo.userId = acceptUserId;
          acceptUserContacts.push(acceptUserInfo);
        }
        listContractAction(acceptUserId)
          .then(res => {
            if (res !== "没有联系人") {
              // console.log("接受卡片的人的联系人");
              // console.log(JSON.parse(res));
              sendUserContacts = JSON.parse(res);
            }
            let isHasSendUser = sendUserContacts.some(ele => {
              if (ele.userId == sendUserId) {
                ele.cardsNumber = ++ele.cardsNumber;
                ele.cardList.push(cardInfo);
                return true;
              } else {
                return false;
              }
              // return (ele.userId = sendUserId);
            });
            // console.log("是否存在发送的人" + isHasSendUser);
            // console.log("是否存在接受的人" + isHasAcceptUser);
            if (isHasSendUser) {
            } else {
              let sendUserInfo = clone(userInfo);
              sendUserInfo.userId = sendUserId;
              sendUserInfo.cardsNumber = ++sendUserInfo.cardsNumber;
              sendUserInfo.cardList.push(cardInfo);
              sendUserContacts.push(sendUserInfo);
            }
            // console.log(JSON.stringify(acceptUserContacts));
            // console.log(sendUserId);
            // console.log(JSON.stringify(sendUserContacts));
            createContractAction(
              JSON.stringify(sendUserContacts),
              sendUserId,
              JSON.stringify(acceptUserContacts)
            )
              .then(res => {
                loading.close();
                location.href = "./mycard.html";
              })
              .catch(err => {
                // console.log(3);
                console.log(err);
              });
          })
          .catch(err => {
            // console.log(2);
            console.log(err);
          });
      })
      .catch(err => {
        // console.log(1);
        console.log(err);
      });
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
