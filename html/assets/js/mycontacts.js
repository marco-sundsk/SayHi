$(document).ready(() => {
  let currentUser = "";
  let loading = $(document).dialog({
    type: "toast",
    infoIcon: "./assets/images/loading.gif",
    infoText: "正在加载中"
  });
  $(".back").click(() => {
    history.back();
  });

  listContractAction()
    .then(res => {
      // console.log(res);
      if (res) {
        let data = res;
        let html = "";
        data.forEach(
          ({ contact_person, id, card_count, duration, card_list, name }) => {
            html += `<div class="one-contacts" data-cardList='${card_list}' data-uid='${contact_person}'>
      <img src="./assets/images/userimg.png" class="user-img" />
      <img src="./assets/images/send_user.png" class="contacts-send-btn" data-uid="${contact_person}"/>
      <div class="id">${contact_person}</div>
      <div class="desc"></div>
      <div class="bottom">
        <div class="left">
          <div class="tit">收到的卡</div>
          <div class="info">${card_count}</div>
        </div>`;
            // if (cardList.length != 0) {
            html += `<div class="right">
          <div class="tit">到期时间</div>
          <div class="info">${duration}</div>
        </div>`;
            // }
            html += ` </div>
        </div>`;
          }
        );
        $(".list-wrapper").html(html);
        $(".contacts-send-btn").click(function(e) {
          let uid = $(this)[0].dataset.uid;
          $(".name").html(uid);
          currentUser = uid;
          $(".send-card-form").show(0, () => {
            $(".send-card-form").css({
              transform: "translateY(0vh)"
            });
          });
          e.stopPropagation();
        });
        $(".close").click(() => {
          setTimeout(() => {
            $(".send-card-form").hide();
          }, 500);
          $(".send-card-form").css({
            transform: "translateY(76vh)"
          });
        });
        $(".one-contacts").click(function(e) {
          let cardList = $(this)[0].dataset.cardlist;
          let uid = $(this)[0].dataset.uid;
          // console.log(cardList);
          if (cardList !== "[{}]") {
            localStorage.setItem("cardList", cardList);
            location.href = "./contactscard.html?uid=" + uid;
          }
        });
        loading.close();
      } else {
        loading.close();
        $(document).dialog({
          type: "notice",
          infoText: "没有联系人",
          autoClose: 1500
        });
      }
    })
    .catch(err => {
      console.log(err);
    });

  $(".send-btn").click(() => {
    let cardNameValue = $("#cardName").val();
    let publicInfoValue = $("#publicInfo").val();
    let privateInfoValue = $("#privateInfo").val();
    let countValue = "1";
    let isAvgValue = true;
    let totalValue = $("#total").val();
    let date = getExpirationDate(
      $("#expirationDate").val(),
      $("#dateType").val()
    );
    if (cardNameValue === "") {
      $(document).dialog({
        type: "notice",
        infoText: "请填写卡片名称",
        autoClose: 1500
      });
      return;
    }
    if (publicInfoValue === "") {
      $(document).dialog({
        type: "notice",
        infoText: "请填写公开信息",
        autoClose: 1500
      });
      return;
    }
    if ($("#expirationDate").val() === "") {
      $(document).dialog({
        type: "notice",
        infoText: "请填写过期时间",
        autoClose: 1500
      });
      return;
    }
    let loading = $(document).dialog({
      type: "toast",
      infoIcon: "./assets/images/loading.gif",
      infoText: "正在加载中"
    });
    let cardInfo = {
      card_type: 0,
      template_id: "template_1",
      name: cardNameValue,
      public_message: publicInfoValue,
      private_message: BASE64.encode(privateInfoValue), //暂时base64简单加密，之后使用公钥私钥加密解码
      count: 1,
      // isAvg: isAvgValue,
      total: parseInt(totalValue) ? parseInt(totalValue) : 0,
      duration: date,
      specify_account: currentUser
    };
    console.log(cardInfo);
    window.contract
      .create_card(cardInfo)
      .then(res => {
        if (res) {
          loading.close();
          $(document).dialog({
            type: "notice",
            infoText: "创建成功",
            autoClose: 1500,
            onClosed: () => {
              // location.reload();
              location.href =
                "./qrcode.html?cardcode=" + res + "&id=" + window.accountId;
            }
          });
        } else {
          $(document).dialog({
            type: "notice",
            infoText: "创建失败",
            autoClose: 1500
          });
        }
      })
      .catch(err => {
        loading.close();
        $(document).dialog({
          type: "notice",
          infoText: "创建失败",
          autoClose: 1500
        });
        console.log(err);
      });
  });
});

async function listContractAction() {
  // console.log("account_id：" + window.accountId);
  return window.contract.list_contract_person({
    account_id: window.accountId
  });
}

let getExpirationDate = (number, type) => {
  switch (type) {
    case "天":
      let day = 86400;
      return (day * number) / 2;
      break;
    case "周":
      let week = 604800;
      return (week * number) / 2;
      break;
    case "月":
      let month = 2592000;
      return (month * number) / 2;
      break;
    case "年":
      let year = 31536000;
      return (year * number) / 2;
      break;
  }
};
