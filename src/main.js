const { invoke } = window.__TAURI__.tauri;

 
 
const basePath=window.location.href;
async function greet(url,label,title) {
  // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
  // greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });

 const s =  await invoke("greet",{appConfig:  { url: url,label:label,base_path: basePath,title:title }} );
 
}

window.addEventListener("DOMContentLoaded", () => {
  
  document
    .querySelector("#baiduyunpan")
    .addEventListener("click", () => greet("https://www.kelongwo.com/Resource_function/pan/baidu/","baiduyunpan","百度云盘"));

    document
    .querySelector("#zlibary")
    .addEventListener("click", () => greet("https://node1.v4.zhelper.net/","zlibary","zlibary"));

    document
    .querySelector("#png-converter")
    .addEventListener("click", () => greet("https://cloudconvert.com/","png-converter","png-converter"));

    
    document
    .querySelector("#typepad")
    .addEventListener("click", () => greet("https://kylebing.cn/tools/typepad/","typepad","typepad"));
});
