$(document).ready(() => {
  $(".back").click(() => {
    history.back();
  });
  let cardcode = GetUrlString("cardcode");
  window.contract
    .find_account_by_card({
      card_id: cardcode
    })
    .then(res => {
      $(".name").html(res);
    })
    .catch(err => {
      loading.close();
      console.log(err);
    });
  let cardinfo = localStorage.getItem("cardinfo");
  let data = JSON.parse(cardinfo);
  $(".title").html(data.name);
  $("#public").html(data.public_message);
  if (data.private_message) {
    $("#private").html(BASE64.decode(data.private_message));
  } else {
    $("#privateWrap").hide();
  }
  if (data.total) {
    $("#red").html(parseInt(data.total).toFixed(2));
  } else {
    $(".red-bag").hide();
  }
});
