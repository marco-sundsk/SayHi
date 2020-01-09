$(document).ready(() => {
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
    // console.log(window.accountId)
    let w = window;
    window.contract
      .list_card({
        account_id: window.accountId
      })
      .then(res => {
        loading.close();
        // console.log(res)
        if (res) {
          let html = "";
          res.forEach(element => {
            html += `<div class="swiper-slide" data-cardInfo='${JSON.stringify(
              element
            )}' >
        <div class="card-name">${element.name}</div>
        <div class="id">${window.accountId}</div>
        <div class="card-info">
          <img src="./assets/images/card_number.png" />${element.public_message}
        </div>`;
            if (element.private_message) {
              html += `<div class="card-info">
            <img src="./assets/images/card_name.png" />${BASE64.decode(element.private_message)}
          </div>`;
            }
            if (element.total) {
              html += `<div class="red-bag">
            <img src="./assets/images/red2.png" />${parseInt(
              element.total
            ).toFixed(2)}
          </div>`;
            }
            html += `<div class="qrcode" data-id="${element.id}">
            <img src="./assets/images/qrcodes.png" />查看
          </div></div>`;
          });
          $(".swiper-wrapper").html(html);

          $(".swiper-slide").click(function() {
            let cardinfo = $(this)[0].dataset.cardinfo;
            localStorage.setItem("cardinfo", cardinfo);
            let cardinfoObj = JSON.parse(cardinfo);
            location.href = "./carddetail.html?cardcode=" + cardinfoObj.id;
          });

          $(".qrcode").click(function(e) {
            e.stopPropagation();
            let id = $(this)[0].dataset.id;
            location.href =
              "./qrcode.html?cardcode=" + id + "&id=" + window.accountId;
          });

          new Swiper(".card-container", {
            slidesPerView: 1.6,
            spaceBetween: 0,
            centeredSlides: true,
            loop: false
          });
        } else {
          $(document).dialog({
            type: "notice",
            infoText: "没有卡片",
            autoClose: 1500
          });
        }
      })
      .catch(err => {
        loading.close();
        console.log(err);
      });
  };

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
      card_type: 0,
      template_id: "template_1",
      name: cardNameValue,
      public_message: publicInfoValue,
      private_message: BASE64.encode(privateInfoValue), //暂时base64简单加密，之后使用公钥私钥加密解码
      count: parseInt(countValue),
      // isAvg: isAvgValue,
      total: parseInt(totalValue),
      duration: date,
      specify_account: ""
    };
    // console.log(cardInfo);
    window.contract
      .create_card(cardInfo)
      .then(res => {
        // console.log(res)
        if (res) {
          loading.close();
          $(document).dialog({
            type: "notice",
            infoText: "创建成功",
            autoClose: 1500,
            onClosed: () => {
              // location.reload();
              location.href =
                "./qrcode.html?cardcode=" +
                res +
                "&id=" +
                window.accountId;
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
