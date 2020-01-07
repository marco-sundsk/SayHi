$(document).ready(() => {
  let cardList = [];
  let loading = $(document).dialog({
    type: "toast",
    infoIcon: "./assets/images/loading.gif",
    infoText: "正在加载中"
  });
  $(".back").click(() => {
    history.back();
  });

  let getMyCardList = () => {
    window.contract
      .listCard()
      .then(res => {
        loading.close();
        cardList = JSON.parse(res);
        // console.log(cardList);
      })
      .catch(err => {
        loading.close();
        console.log(err);
      });
  };

  listContractAction()
    .then(res => {
      getMyCardList();
      let data = JSON.parse(res);
      console.log(data);
      let html = "";
      data.forEach(element => {
        html += `<div class="one-contacts">
      <img src="./assets/images/userimg.png" class="user-img" />
      <img src="./assets/images/send_user.png" class="contacts-send-btn" onclick="sendAction('${element.userId}')"/>
      <div class="id">${element.userId}</div>
      <div class="desc"></div>
      <div class="bottom">
        <div class="left">
          <div class="tit">收到的卡</div>
          <div class="info">${element.cardsNumber}</div>
        </div>
        <div class="right">
          <div class="tit">到期时间</div>
          <div class="info">${element.date}</div>
        </div>
      </div>
    </div>`;
      });
      $(".list-wrapper").html(html);
      $(".close").click(() => {
        setTimeout(() => {
          $(".send-card-form").hide();
        }, 500);
        $(".send-card-form").css({
          transform: "translateY(76vh)"
        });
      });
    })
    .catch(err => {
      loading.close();
      $(document).dialog({
        type: "notice",
        infoText: "没有联系人",
        autoClose: 1500
      });
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
    let cardCode = new Date().getTime().toString();
    let cardInfo = {
      code: cardCode,
      cardUser: window.accountId,
      cardName: cardNameValue,
      publicInfo: publicInfoValue,
      privateInfo: privateInfoValue,
      count: countValue,
      isAvg: isAvgValue,
      total: parseInt(totalValue).toFixed(2),
      expirationDate: date
    };
    cardList.push(cardInfo);
    let params = JSON.stringify(cardList);
    window.contract
      .createCard({
        cardInfo: params,
        cardCode: cardCode,
        newCardInfo: JSON.stringify(cardInfo)
      })
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
                "./qrcode?cardcode=" + cardCode + "&id=" + window.accountId;
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
  return window.contract.listContract();
}

let sendAction = id => {
  $("#name").html(id);
  $(".send-card-form").show(0, () => {
    $(".send-card-form").css({
      transform: "translateY(0vh)"
    });
  });
};

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
