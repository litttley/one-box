const { invoke } = window.__TAURI__.tauri;

 
 
const basePath=window.location.href;
async function openApp(url,label,title) {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  // openAppMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });

 const s =  await invoke("open_app",{appConfig:  { url: url,label:label,base_path: basePath,title:title }} );

 
}

window.addEventListener("DOMContentLoaded", () => {
  
  document
    .querySelector("#baiduyunpan")
    .addEventListener("click", () => openApp("https://www.kelongwo.com/Resource_function/pan/baidu/","baiduyunpan","百度云盘"));

    document
    .querySelector("#zlibary-one")
    .addEventListener("click", () => openApp("https://tool.yibook.org/","zlibary","zlibary"))
    //  .addEventListener("click", () => openApp("https://libgen.ee/","zlibary","zlibary"))
    //  .addEventListener("click", () => openApp("https://node1.v5.zhelper.net/","zlibary","zlibary"))

    // document
    // .querySelector("#zlibary-two")
    // .addEventListener("click", () => openApp("https://node1.v5.zhelper.net/","zlibary","zlibary"))
    
    ;

    document
    .querySelector("#png-converter")
    .addEventListener("click", () => openApp("https://cloudconvert.com/","png-converter","png-converter"));

    
    document
    .querySelector("#typepad")
    .addEventListener("click", () => openApp("https://kylebing.cn/tools/typepad/","typepad","typepad"));

    document
    .querySelector("#neat-reader")
    .addEventListener("click", () => openApp("https://www.neat-reader.cn/webapp#/","neat-reader","neat-reader"));

    document
    .querySelector("#compiler-explorer")
    .addEventListener("click", () => openApp("https://godbolt.org/","compiler-explorer","compiler-explorer"));

    document
    .querySelector("#chatGPT")
    .addEventListener("click", () => openApp("https://gpt.chatapi.art/","chatGPT","chatGPT"));
});
