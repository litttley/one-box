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
    .querySelector("#zlibary")
    .addEventListener("click", () => openApp("https://node1.v4.zhelper.net/","zlibary","zlibary"));

    document
    .querySelector("#png-converter")
    .addEventListener("click", () => openApp("https://cloudconvert.com/","png-converter","png-converter"));

    
    document
    .querySelector("#typepad")
    .addEventListener("click", () => openApp("https://kylebing.cn/tools/typepad/","typepad","typepad"));

    document
    .querySelector("#neat-reader")
    .addEventListener("click", () => openApp("https://www.neat-reader.cn/webapp#/","typepad","typepad"));

    document
    .querySelector("#compiler-explorer")
    .addEventListener("click", () => openApp("https://godbolt.org/","compiler-explorer","compiler-explorer"));
});
