const snowContainer = document.getElementById('snow-container');

function createSnowflake () {
  const snowflake = document.createElement('img');
  snowflake.src = './snowflake.svg';
  snowflake.classList.add('snowflake');
  snowflake.style.left = (Math.random() + 0.0625) * window.innerWidth * 0.9 + 'px';

  const random = Math.random();
  var rotate = random * 360;
  snowflake.style.transform = 'rotate(' + rotate + 'deg)';

  var size = random * 16 + 4;
  snowflake.style.width = size + 'px';
  snowflake.style.height = size + 'px';
  snowflake.style.animationDuration = Math.random() * 5 + 5 + 's';

  snowContainer.appendChild(snowflake);

  setTimeout(() => {
    snowflake.style.opacity = 0.0;
    snowflake.style.visibility = false;
  }, 5000);

  setTimeout(() => {
    snowflake.remove();
  }, 10000);
}

setInterval(createSnowflake, 200);
