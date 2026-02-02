$(document).ready(function () {
  // change this to be the server where blt is hosted
  const fqdn = 'https://www.hosted-blt-server.com';
  const button = document.createElement('Button');
  button.style = 'bottom:15px;left:15px;position:fixed;z-index: 12;border-radius:100%;background-color:white; background: url(\'https://api.screenshotlayer.com/api/capture?access_key=<YOUR_ACCESS_KEY>&url=' + encodeURIComponent(window.location.href) + '\') no-repeat center; height: 50px; width: 50px; outline: none;background-size: 30px 30px;';
  document.body.appendChild(button);
  const url = window.location.href;
  const redirect_url = fqdn + url;
  button.onclick = function () {
    const redirectWindow = window.open(redirect_url, '_blank');
    redirectWindow.location;
  };
});
