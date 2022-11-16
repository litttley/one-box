window.addEventListener('DOMContentLoaded', (_event) => {
    const style = document.createElement('style');
    style.innerHTML = `
    
    *::-webkit-scrollbar {
      display: none;
    }
 
  `;
    document.head.append(style);
});
 