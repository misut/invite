var container = document.getElementById('map'); //지도를 담을 영역의 DOM 레퍼런스
var position = new kakao.maps.LatLng(37.378064340731456, 127.13860371326346);
var options = { //지도를 생성할 때 필요한 기본 옵션
  center: position, //지도의 중심좌표.
  level: 2//지도의 레벨(확대, 축소 정도)
};
var map = new kakao.maps.Map(container, options);
var marker = new kakao.maps.Marker({ position: position });
marker.setMap(map)
