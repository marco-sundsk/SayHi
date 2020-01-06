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
  let url =
    window.location.protocol + "//" + location.host + "?cardcode=" + cardcode;
  console.log(url);
  var qrcode = new QRCode("qrcode", {
    text: url,
    width: 168,
    height: 168,
    colorDark: "#000000",
    colorLight: "#ffffff",
    correctLevel: QRCode.CorrectLevel.H
  });
  window.contract.getCardInfo({ cardCode: cardcode }).then(res => {
    loading.close();
    let data = JSON.parse(res);
    $("#name").html(data.cardUser);
  });
});
