$(document).ready(() => {
  $(".back").click(() => {
    history.back();
  });
  let cardcode = GetUrlString("cardcode");
  let loading = $(document).dialog({
    type: "toast",
    infoIcon: "./assets/images/loading.gif",
    infoText: "正在加载中"
  });
  window.contract.getCardInfo({ cardCode: cardcode }).then(res => {
    loading.close();
    let data = JSON.parse(res);
    $(".title").html(data.cardName);
    $(".name").html(data.cardUser);
    $("#public").html(data.publicInfo);
    if (data.privateInfo) {
      $("#private").html(data.privateInfo);
    } else {
      $("#privateWrap").hide();
    }
    if (data.count) {
      $("#red").html(data.count);
    } else {
      $(".red-bag").hide();
    }
  });
});
