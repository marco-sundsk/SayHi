$(document).ready(() => {
  var swiper = new Swiper(".card-container", {
    slidesPerView: 1.6,
    spaceBetween: 0,
    centeredSlides: true,
    loop: false
  });

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
    var loading = $(document).dialog({
      type: "toast",
      infoIcon: "./assets/images/loading.gif",
      infoText: "正在加载中"
    });
    let cardInfo = {
      cardName: cardNameValue,
      publicInfo: publicInfoValue,
      privateInfo: privateInfoValue,
      count: countValue,
      isAvg: isAvgValue,
      total: parseInt(totalValue).toFixed(2),
      expirationDate: date
    };
    let params = JSON.stringify(cardInfo);
    window.contract.createCard({ cardInfo: params }).then(res => {
      if (res) {
        loading.close();
        $(document).dialog({
          type: "notice",
          infoText: "创建成功",
          autoClose: 1500
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

  window.contract.listCard().then(res => {
    console.log(res);
  });

  window.contract.hello().then(res => {
    console.log(res);
  });
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
