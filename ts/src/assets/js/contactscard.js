$(document).ready(() => {
  $(".back").click(() => {
    history.back();
  });
  let uid = GetUrlString("uid");
  $(".user-name").html(uid);
  let cardList = localStorage.getItem("cardList");
  cardList = JSON.parse(cardList);
  console.log(cardList);
  let html = "";
  cardList.forEach(element => {
    html += `<div class="swiper-slide" onclick="gotoDetail(${element.code})">
    <div class="card-name">${element.cardName}</div>
    <div class="id">${uid}</div>
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

let gotoDetail = code => {
  location.href = "./carddetail?cardcode=" + code;
};
