window.addEventListener('DOMContentLoaded', (_event) => {
    const style = document.createElement('style');
    style.innerHTML = `
    
    *::-webkit-scrollbar {
      display: none;
    }
 
  `;
    document.head.append(style);
});

// setDefaultZoom();

// function setDefaultZoom() {
//   const htmlZoom = window.localStorage.getItem('htmlZoom');
//   if (htmlZoom) {
//     document.getElementsByTagName('html')[0].style.zoom = htmlZoom;
//   }
// }

// function zoomIn() {
//   const htmlZoom = window.localStorage.getItem('htmlZoom') || '100%';
//   const html = document.getElementsByTagName('html')[0];
//   const zoom = parseInt(htmlZoom) < 200 ? (parseInt(htmlZoom) + 10 +'%') : '200%';
//   html.style.zoom = zoom;
//   window.localStorage.setItem('htmlZoom', zoom);
// }

// function zoomOut() {
//   const htmlZoom = window.localStorage.getItem('htmlZoom') || '100%';
//   const html = document.getElementsByTagName('html')[0];
//   const zoom = parseInt(htmlZoom) > 30 ? (parseInt(htmlZoom) - 10 +'%') : '30%';
//   html.style.zoom = zoom;
//   window.localStorage.setItem('htmlZoom', zoom);
// }