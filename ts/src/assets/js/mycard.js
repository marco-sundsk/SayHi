$(document).ready(() => {
  let cardList = [];

  $(".user-name").html(window.accountId);
  $(".add-icon").click(() => {
    $(".send-card-form").show(0, () => {
      $(".send-card-form").css({
        transform: "translateY(0vh)"
      });
    });
  });

  $(".close").click(() => {
    setTimeout(() => {
      $(".send-card-form").hide();
    }, 500);
    $(".send-card-form").css({
      transform: "translateY(76vh)"
    });
  });

  $(".my-contacts").click(() => {
    location.href = "./mycontacts.html";
  });

  let getMyCardList = () => {
    // $(".swiper-wrapper").empty();
    let loading = $(document).dialog({
      type: "toast",
      infoIcon: "./assets/images/loading.gif",
      infoText: "正在加载中"
    });
    window.contract.listCard().then(res => {
      loading.close();
      cardList = JSON.parse(res);
      // console.log(cardList);
      let html = "";
      cardList.forEach(element => {
        html += `<div class="swiper-slide" onclick="gotoDetail(${element.code})">
        <div class="card-name">${element.cardName}</div>
        <div class="id">${window.accountId}</div>
        <div class="card-info">
          <img src="./assets/images/card_number.png" />${element.publicInfo}
        </div>`;
        if (element.privateInfo) {
          html += `<div class="card-info">
            <img src="./assets/images/card_name.png" />${element.privateInfo}
          </div>`;
        }
        if (element.total) {
          html += `<div class="red-bag">
            <img src="./assets/images/red2.png" />${element.total}
          </div>`;
        }
        html += `</div>`;
      });
      $(".swiper-wrapper").html(html);
      new Swiper(".card-container", {
        slidesPerView: 1.6,
        spaceBetween: 0,
        centeredSlides: true,
        loop: false
      });
    });
  };

  // $('.swiper-slide').click(()=>{
  //   alert(1)
  //   console.log($(this).dataset.code)
  // })

  $(".send-btn").click(() => {
    let cardNameValue = $("#cardName").val();
    let publicInfoValue = $("#publicInfo").val();
    let privateInfoValue = $("#privateInfo").val();
    let countValue = $("#count").val();
    let isAvgValue = $("#isAvg").val() == "平均分配红包" ? true : false;
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
    if (countValue === "") {
      $(document).dialog({
        type: "notice",
        infoText: "请填写卡片数量",
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
              location.href = "./qrcode?cardcode=" + cardCode;
            }
          });
          setTimeout(() => {
            $(".send-card-form").hide();
          }, 500);
          $(".send-card-form").css({
            transform: "translateY(76vh)"
          });
        } else {
          $(document).dialog({
            type: "notice",
            infoText: "创建失败",
            autoClose: 1500
          });
        }
      });
  });
  getMyCardList();
});

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

let gotoDetail = code => {
  location.href = "./carddetail?cardcode=" + code;
};
