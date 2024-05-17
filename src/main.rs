#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Instant;

use anyhow::Result;
use rfd::FileDialog;
use rgb565::{rgb565_u16_image_to_rgb888, slice_u8_to_slice_u16};
use slint::{Image, SharedPixelBuffer, SharedString, Timer, TimerMode};

mod rgb565;

slint::slint!{
    import { Button, LineEdit, HorizontalBox, ComboBox } from "std-widgets.slint";
    export component App inherits Window {
        title: "RGB565 Previewer";
        icon: @image-url("icon.png");
        width: 640px;
        height: 480px;
        in property <image> image;
        in property <string> path;
        in-out property <string> image-width;
        in-out property <string> image-height;
        in-out property <string> update-delay;

        callback open-file();

        VerticalLayout {
            HorizontalBox {
                height: 60px;
                Button {
                    text: "打开";
                    width: 100px;
                    clicked => { open-file() }
                }
                LineEdit {
                    text <=> image-width;
                    input-type: number;
                    placeholder-text: "输入宽度";
                }
                LineEdit {
                    text <=> image-height;
                    input-type: number;
                    placeholder-text: "输入高度";
                }
                Text { text: "更新:"; height: 60px; vertical-alignment: TextVerticalAlignment.center;  }
                LineEdit {
                    width: 50px;
                    text <=> update-delay;
                    input-type: number;
                    placeholder-text: "5秒";
                }
            }
            HorizontalBox{
                height: 30px;
                Text { text: "文件:"+path; }
            }
            Rectangle {
                HorizontalBox {
                    Image { source: image; }
                }
            }
        }
    }
}

fn main() -> Result<()> {
    let app = App::new()?;
    let app_clone = app.as_weak();
    app.on_open_file(move ||{
        let path = FileDialog::new().pick_file();
        if let Some(path) = path{
            if let Some(path) = path.to_str(){
                if let Some(app) = app_clone.upgrade(){
                    if let Some(ext) = path.split(".").last(){
                        if ext.contains("x"){
                            let mut arr = ext.split("x");
                            let w = arr.next().unwrap_or("");
                            let h = arr.next().unwrap_or("");
                            if let Ok(w) = w.parse::<i32>(){
                                app.set_image_width(SharedString::from(format!("{w}")));
                            }
                            if let Ok(h) = h.parse::<i32>(){
                                app.set_image_height(SharedString::from(format!("{h}")));
                            }
                        }
                    }
                    app.set_path(SharedString::from(path));
                    if let Err(err) = show_image(app){
                        eprintln!("{:?}", err);
                    }
                }
            }
        }
    });
    let timer = Timer::default();
    let app_clone = app.as_weak();
    let mut t = Instant::now();
    timer.start(TimerMode::Repeated, std::time::Duration::from_millis(500), move || {
        if let Some(app) = app_clone.upgrade(){
            let delay = app.get_update_delay().to_string().parse::<u64>().unwrap_or(5);
            if t.elapsed().as_secs() > delay{
                t = Instant::now();
                if let Err(err) = show_image(app){
                    eprintln!("{:?}", err);
                }
            }
        }
    });

    // use image::open;
    // use rgb565::{rgb888_to_rgb565_u16, slice_u16_to_slice_u8};
    // let bmp = open("rust-pride.bmp")?.to_rgb8();
    // let raw = rgb888_to_rgb565_u16(&bmp, bmp.width() as usize, bmp.height() as usize);
    // let raw = slice_u16_to_slice_u8(&raw);
    // std::fs::write("rust-pride.64x64", raw)?;
    app.run()?;
    Ok(())
}

fn show_image(app: App) -> Result<()>{
    let path = app.get_path().to_string();
    let width = app.get_image_width().to_string();
    let height = app.get_image_height().to_string();
    if path.len() == 0{
        return Ok(())
    }
    let data = std::fs::read(path)?;
    let data = slice_u8_to_slice_u16(&data);
    let width = width.parse()?;
    let height = height.parse()?;
    if data.len() != (width*height) as usize{
        return Ok(())
    }

    let pixels = rgb565_u16_image_to_rgb888(data, width, height);
    let shared = SharedPixelBuffer::clone_from_slice(&pixels, pixels.width(), pixels.height());
    let img = Image::from_rgb8(shared);
    app.set_image(img);

    Ok(())
}