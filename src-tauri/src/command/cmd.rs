use std::{
    collections::HashMap,
    fmt::format,
    fs::File,
    io::{BufReader, Read},
    path::Path,
};

use image::ImageFormat;
use tauri::{AppHandle, Icon, Manager};
use url::Url;

use crate::{errors::custome_error::CustomeErrors, modal::modal::AppConfig};

//"https://www.kelongwo.com/Resource_function/pan/baidu/".parse().unwrap()
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
pub async fn greet(
    app_config: AppConfig,
    handle: tauri::AppHandle,
) -> Result<String, CustomeErrors> {
    let AppConfig {
        url,
        label,
        base_path,
        title,
    } = app_config;
 
    //开发环境使用dev server加载
    #[cfg(debug_assertions)]
    let icon = load_icon_by_url(format!("{}/assets/app-icon/{}.png", base_path, label)).await?;
    #[cfg(debug_assertions)]
    let script = load_script_by_url(format!("{}/assets/app-js/{}.js", base_path, label)).await?;
    //tauri build 使用asset_resolver
    #[cfg(not(debug_assertions))]
    let icon =
        load_icon_by_asset_resolver(format!("/assets/app-icon/{}.png", label), &handle).await?;
    #[cfg(not(debug_assertions))]
    let script =
        load_script_by_asset_resolver(format!("/assets/app-js/{}.js", label), &handle).await?;

    //使用resolve_resource的读取资源
    // let icon_path = handle
    //     .path_resolver()
    //     .resolve_resource(format!("assets/icons/{}.png", label))
    //     .unwrap();

    // let script_path = handle
    //     .path_resolver()
    //     .resolve_resource(format!("assets/js/{}.js", label))
    //     .unwrap();

    //    let s =  handle.asset_resolver().get("/index.html".to_string()).unwrap().mime_type;
    //    println!("{:#?}",s);

    // let icon = load_icon(icon_path.as_path())?;
    // let script = load_script(script_path.as_path())?;
   let window =  handle.app_handle().get_window(&label);
    // if let Some(win)=window{
    //     win.show();
    // }

    let Some(win)=window else{
      
        let docs_window = tauri::WindowBuilder::new(
            &handle,
            &label, /* the unique window label */
            tauri::WindowUrl::External(
                Url::parse(&url).map_err(|_| CustomeErrors::CustomError("url不正确".to_string()))?,
            ),
        )
        .initialization_script(&script)
        .title(title)
        .icon(icon)
        
        .unwrap()
        .build()
        
        .unwrap();
         
       
        return    Ok(script)
    };
 
   let s =  win.maximize();
 
   println!("{:#?}",s);
    return Ok(script)
 

}

async fn load_icon_by_asset_resolver(
    assets_path: String,
    handle: &AppHandle,
) -> Result<Icon, CustomeErrors> {
    let (icon_rgba, icon_width, icon_height) = {
        let bytes = handle.asset_resolver().get(assets_path).unwrap().bytes;

        let imagebuffer = image::load_from_memory_with_format(&bytes, ImageFormat::Png)
            .map_err(|_| CustomeErrors::CustomError("icon二进制转换异常".to_string()))?
            .into_rgba8();

        let (width, height) = imagebuffer.dimensions();
        let rgba = imagebuffer.into_raw();
        (rgba, width, height)
    };

    Ok(Icon::Rgba {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    })
}

async fn load_icon_by_url(url: String) -> Result<Icon, CustomeErrors> {
    let (icon_rgba, icon_width, icon_height) = {
        let bytes = reqwest::get(url)
            .await
            .map_err(|_| CustomeErrors::CustomError("reqwest加载icon异常".to_string()))?
            .bytes()
            .await
            .map_err(|_| CustomeErrors::CustomError("reqwest加载icon异常".to_string()))?
            .to_vec();
        // let imagebuffer = image::open(path)
        //     .expect("Failed to open icon path")
        //     .into_rgba8();
        let imagebuffer = image::load_from_memory_with_format(&bytes, ImageFormat::Png)
            .map_err(|_| CustomeErrors::CustomError("icon二进制转换异常".to_string()))?
            .into_rgba8();

        let (width, height) = imagebuffer.dimensions();
        let rgba = imagebuffer.into_raw();
        (rgba, width, height)
    };

    Ok(Icon::Rgba {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    })
}
fn load_icon(path: &Path) -> Result<Icon, CustomeErrors> {
    let (icon_rgba, icon_width, icon_height) = {
        let imagebuffer = image::open(path)
            .expect("Failed to open icon path")
            .into_rgba8();

        let (width, height) = imagebuffer.dimensions();
        let rgba = imagebuffer.into_raw();
        (rgba, width, height)
    };

    Ok(Icon::Rgba {
        rgba: icon_rgba,
        width: icon_width,
        height: icon_height,
    })
    // Icon::from_rgba(icon_rgba, icon_width, icon_height)?
}

async fn load_script_by_url(url: String) -> Result<String, CustomeErrors> {
    let script = reqwest::get(url)
        .await
        .map_err(|_| CustomeErrors::CustomError("reqwest加载icon异常".to_string()))?
        .text()
        .await
        .map_err(|_| CustomeErrors::CustomError("reqwest加载icon异常".to_string()))?;
    Ok(script)
}

async fn load_script_by_asset_resolver(
    url: String,
    handle: &AppHandle,
) -> Result<String, CustomeErrors> {
    let script = handle.asset_resolver().get(url).unwrap().mime_type;
    //    println!("{:#?}",s);
    Ok(script)
}
fn load_script(path: &Path) -> Result<String, CustomeErrors> {
    let mut contents = String::new();
    let mut file =
        File::open(path).map_err(|_| CustomeErrors::CustomError("js脚本加载失败!".to_string()))?;
    let _ = file
        .read_to_string(&mut contents)
        .map_err(|_| CustomeErrors::CustomError("js脚本加载失败!".to_string()));
    Ok(contents)
}
