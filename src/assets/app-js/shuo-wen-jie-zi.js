window.addEventListener('DOMContentLoaded', (_event) => {
    const style = document.createElement('style');
    style.innerHTML = `
    
    *::-webkit-scrollbar {
      display: none;
    }

    iframe{
      display: none;
    }
 
  `;
    document.head.append(style);
    document.addEventListener("click", (e) =>  {
      console.log(e.target.parentNode.parentNode.className);
      var list_group_node = e.target.parentNode.parentNode;
      console.log(list_group_node)
      if(list_group_node.className == 'list-group'){
       var  child_nodes =list_group_node.getElementsByTagName('a');
       console.log(child_nodes);
       for(var i=0;i<child_nodes.length;i++){ 
        var a_node = child_nodes[i];
        var a_href =a_node.href;
        console.log(a_href)
        if(a_href.indexOf("https://download.v4.zhelper.net")== 0 ){
          a_node.href=a_href.replace("https://download.v4.zhelper.net","https://zlib.download")
          console.log(   a_node.href)
        }
     }
      }
      // e.stopPropagation();
    });
});

 