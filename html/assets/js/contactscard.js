$(document).ready(() => {
  $(".back").click(() => {
    history.back();
  });
  let uid = GetUrlString("uid");
  $(".user-name").html(uid);
  let cardList = localStorage.getItem("cardList");
  cardList = JSON.parse(cardList);
  cardList = cardList.slice(1, cardList.length);
  // console.log(cardList);
  let html = "";
  cardList.forEach(element => {
    html += `<div class="swiper-slide" data-cardInfo='${JSON.stringify(
      element
    )}'>
    <div class="card-name">${element.name ? element.name : "-"}</div>
    <div class="id">${uid}</div>
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
        <img src="./assets/images/red2.png" />${element.total}
      </div>`;
    }
    html += `</div>`;
  });
  $(".swiper-wrapper").html(html);
  $(".swiper-slide").click(function() {
    let cardinfo = $(this)[0].dataset.cardinfo;
    localStorage.setItem("cardinfo", cardinfo);
    let cardinfoObj = JSON.parse(cardinfo);
    location.href = "./carddetail.html?cardcode=" + cardinfoObj.id;
  });
  new Swiper(".card-container", {
    slidesPerView: 1.6,
    spaceBetween: 0,
    centeredSlides: true,
    loop: false
  });
});
