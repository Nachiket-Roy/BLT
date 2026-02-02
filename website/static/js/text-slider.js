$(document).ready(function () {
  const lines = $('.hero .text-slider-line');
  const n = lines.length;
  let i = 0;
  $(lines[0]).fadeIn();
  const changeText = setInterval(function () {
    $(lines[i]).fadeOut();
    if (i == (n - 1)) {
      i = 0;
    } else {
      i++;
    }
    setTimeout(function () {
      $(lines[i]).fadeIn();
    }, 500);
  }, 2000);
});
