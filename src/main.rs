// 更屌的ui                         ////
// 通过ui更新json                   ////
// 在没有json的时候生成json         ////
//json 路径                         ////
//定时清空                          ////
//只start一次                       ////
//#![allow(unused)]
use anyhow::{Context, Result};
use clap::Parser;
use eframe::egui;
use egui::{Color32, RichText, ViewportCommand};
use std::fs::File;
use std::io::{self, BufReader, Write};
use std::path::PathBuf;
use std::process::{self, Command, Stdio};
use std::time::{Duration, Instant};
use std::usize::MAX;
use std::vec;
use std::{env, os::unix::ffi::OsStrExt, ffi::CString};
use nix::unistd;

//#[derive(Parser, Debug)] // 添加Debug trait方便调试
//struct Claws {
//    #[clap(short, long)]
//secret_key: String,
//}
const GREEN_OP: Color32 = Color32::GREEN;
const WHITE_OP: Color32 = Color32::WHITE;
const GRAY_OP: Color32 = Color32::DARK_GRAY;

#[derive(Parser, Debug)]
struct Claws {
    #[clap(short, long, default_value = "default_secret")]
    secret_key: String,
    #[clap(short, long)]
    config: Option<PathBuf>,
}

//#[derive(serde::Deserialize)]
//#[derive(serde::Deserialize, serde::Serialize)]
#[derive(serde::Deserialize, serde::Serialize, Default)]
struct Config {
    a3_1: Vec<String>,
    a3_2: Vec<String>,
    a3_3: Vec<String>,
    a3_path: Vec<String>,

    a3_c: Vec<String>,
    b4_1: Vec<String>,
    b4_2: Vec<String>,
    b4_3: Vec<String>,
    b4_4: Vec<String>,
    b4_path: Vec<String>,
    b4_c: Vec<String>,

    c5_1: Vec<String>,
    c5_2: Vec<String>,
    c5_3: Vec<String>,
    c5_4: Vec<String>,
    c5_5: Vec<String>,
    c5_path: Vec<String>,
    c5_c: Vec<String>,

    d_file1_json: Vec<String>,
    d_file2_json: Vec<String>,
    d_file3_json: Vec<String>,
    d_file4_json: Vec<String>,
    d_file5_json: Vec<String>,
    d_file6_json: Vec<String>,
    d_file_path: Vec<String>,
    d6_c: Vec<String>,

    e7_1: Vec<String>,
    e7_2: Vec<String>,
    e7_3: Vec<String>,
    e7_4: Vec<String>,
    e7_5: Vec<String>,
    e7_6: Vec<String>,
    e7_7: Vec<String>,
    e7_path: Vec<String>,
    e7_c: Vec<String>,

    a3_name: Vec<String>,
    b4_name: Vec<String>,
    c5_name: Vec<String>,
    d_file_name: Vec<String>,
    e7_name: Vec<String>,
    shell: Vec<String>,
}

fn main() -> eframe::Result {
    let args = Claws::parse();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([896.0, 580.0]),
        ..Default::default()
    };
    eframe::run_native(
        "i need new help",
        options,
        Box::new(|cc| {
            setup_custom_fonts(&cc.egui_ctx);
            Ok(Box::new(KeyboardApp::new(args)))
        }),
    )
}

struct KeyboardApp {
    focused: bool,
    config: Config,
    input_log: Vec<&'static str>,
    open: bool,
    max_input_len: usize,
    log_timer: Option<Instant>,
    time_clear: bool,
    add_in: bool,
    config_path: PathBuf,
    name: String,
    stbrt: String,
    peth: String,
    yon: bool,
    no_have_shell: bool,
    del: bool,
    del_date: usize,
}

impl KeyboardApp {
    fn new(args: Claws) -> Self {
        let config_path = args.config.clone().unwrap_or_else(|| {
            dirs::home_dir()
                .expect("你有家吗?你有没有家啊!!回答我!!look at my eyes")
                .join(".config/user_date.json")
        });

        let config = read_json(&config_path).unwrap_or_else(|e| {
            eprintln!("读取配置文件失败 {}", e);
            if let Some(io_err) = e.downcast_ref::<std::io::Error>() {
                if io_err.kind() == std::io::ErrorKind::NotFound {
                    println!("没找到json文件将生成于{}", config_path.display());
              
                    let default_config = Config::default(); // 假设 Config 实现了 Default trait 哦喵~
           
                    match serde_json::to_string_pretty(&default_config) {
                   
                        Ok(json_string) => {
                    
                            match std::fs::File::create(&config_path) {
                                Ok(mut file) => {
                     
                                    if let Err(write_err) = file.write_all(json_string.as_bytes()) {
                       
                                        eprintln!("写入默认配置文件失败 {}", write_err);
                   
                                    } else {
                                        println!("成功创建了默认配置文件");
                                    }
                                }
                                Err(create_err) => {
               
                                    eprintln!("创建配置文件失败{}", create_err);
                                }
                            }
                        }
                        Err(ser_err) => {
                   
                            eprintln!("生成默认配置失败 {}", ser_err);
              
                            process::exit(1);
                        }
                    }
  
                    default_config
                } else {
                
                    eprintln!("未知错误");
                    process::exit(1);                }
            } else {
                eprintln!("未知错误");
                process::exit(114514);
            }
        });
        let  no_have_shell =  config.shell.is_empty();
        let mut max_input_len = 0;                                                                                                                                                                                   
        max_input_len = max_input_len
            + config.a3_name.len()
            + config.b4_name.len()
            + config.c5_name.len()
            + config.d_file_name.len()
            + config.e7_name.len();
        Self {
            config,
            config_path,
            input_log: Vec::new(),
            focused: false,
            open: true,
            max_input_len,
            log_timer: None,
            time_clear: false,
            add_in: false,
            name: "".to_string(),
            stbrt: "".to_string(),
            peth: "".to_string(),
            yon: false,
            no_have_shell,
            del: false,
            del_date: MAX,
        }
    }
}

impl eframe::App for KeyboardApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_visuals(egui::Visuals::dark());
        self.focused = ctx.input(|i| i.focused);
        ctx.input(|input| {
            for event in &input.events {
                if let egui::Event::Key {
                    key, pressed: true, ..
                } = event
                {
                    let arrow = match key {
                        egui::Key::ArrowLeft => "⇦",
                        egui::Key::ArrowRight => "⇨",
                        egui::Key::ArrowUp =>"⇧",
                        egui::Key::ArrowDown =>   "⇩",
                        egui::Key::Escape => "Escape",
                        egui::Key::Delete => "Delete",
                        egui::Key::Enter => "ADD",
                        egui::Key::Tab => "exit",
                        _ => continue,
                    };
                    self.input_log.push(arrow);
                    if arrow == "Escape" {
                        self.open = false;
                    } else if arrow == "Delete" && self.input_log.len() == 1 {
                        self.del = true;
                        self.input_log.clear();
                    } else if arrow == "Delete" {
                        self.input_log.clear();
                    } else if arrow == "exit" {
                        self.add_in = false;
                        self.del = false;
                        self.yon = false;
                        self.input_log.clear();
                    }
                }
            }
        });
        for yes_add in 0..self.input_log.len() {
            if self.input_log[yes_add] == "ADD" {
                self.input_log.pop();
                let mut yes_yes = false;
                if !self.add_in {
                    self.input_log.clear();
                    self.add_in = true;
                } else if !self.name.is_empty() && !self.stbrt.is_empty() && !self.peth.is_empty()&& !self.peth.contains('"')  && !self.name.contains('"'){
                    match self.input_log.len() {
                        3 => {
                            self.config.a3_1.push(self.input_log[0].to_string());
                            self.config.a3_2.push(self.input_log[1].to_string());
                            self.config.a3_3.push(self.input_log[2].to_string());
                            self.config.a3_name.push(self.name.to_string());
                            self.config.a3_path.push(self.peth.to_string());
                            self.config.a3_c.push(self.stbrt.to_string());
                            self.name = "".to_string();
                            self.peth = "".to_string();
                            self.stbrt = "".to_string();
                            self.yon = false;
                        }
                        4 => {
                            self.config.b4_1.push(self.input_log[0].to_string());
                            self.config.b4_2.push(self.input_log[1].to_string());
                            self.config.b4_3.push(self.input_log[2].to_string());
                            self.config.b4_4.push(self.input_log[3].to_string());
                            self.config.b4_name.push(self.name.to_string());
                            self.config.b4_path.push(self.peth.to_string());
                            self.config.b4_c.push(self.stbrt.to_string());
                            self.name = "".to_string();
                            self.peth = "".to_string();
                            self.stbrt = "".to_string();
                            self.yon = false;
                        }
                        5 => {
                            self.config.c5_1.push(self.input_log[0].to_string());
                            self.config.c5_2.push(self.input_log[1].to_string());
                            self.config.c5_3.push(self.input_log[2].to_string());
                            self.config.c5_4.push(self.input_log[3].to_string());
                            self.config.c5_5.push(self.input_log[4].to_string());
                            self.config.c5_name.push(self.name.to_string());
                            self.config.c5_path.push(self.peth.to_string());
                            self.config.c5_c.push(self.stbrt.to_string());
                            self.name = "".to_string();
                            self.peth = "".to_string();
                            self.stbrt = "".to_string();
                            self.yon = false;
                        }
                        6 => {
                            self.config.d_file1_json.push(self.input_log[0].to_string());
                            self.config.d_file2_json.push(self.input_log[1].to_string());
                            self.config.d_file3_json.push(self.input_log[2].to_string());
                            self.config.d_file4_json.push(self.input_log[3].to_string());
                            self.config.d_file5_json.push(self.input_log[4].to_string());
                            self.config.d_file6_json.push(self.input_log[5].to_string());
                            self.config.d_file_name.push(self.name.to_string());
                            self.config.d_file_path.push(self.peth.to_string());
                            self.config.d6_c.push(self.stbrt.to_string());
                            self.name = "".to_string();
                            self.peth = "".to_string();
                            self.stbrt = "".to_string();
                        }
                        7 => {
                            self.config.e7_1.push(self.input_log[0].to_string());
                            self.config.e7_2.push(self.input_log[1].to_string());
                            self.config.e7_3.push(self.input_log[2].to_string());
                            self.config.e7_4.push(self.input_log[3].to_string());
                            self.config.e7_5.push(self.input_log[4].to_string());
                            self.config.e7_6.push(self.input_log[5].to_string());
                            self.config.e7_7.push(self.input_log[6].to_string());
                            self.config.e7_name.push(self.name.to_string());
                            self.config.e7_path.push(self.peth.to_string());
                            self.config.e7_c.push(self.stbrt.to_string());
                            self.name = "".to_string();
                            self.peth = "".to_string();
                            self.stbrt = "".to_string();
                            self.yon = false;
                        }
                        _ => self.yon = true,
                    }
                    yes_yes = true;
                } else {
                    self.yon = true;
                }
                if yes_yes && !self.yon && self.add_in {
                    let mut file_config = File::create(&self.config_path).expect("error code 1111");
                    let updated_config_json = serde_json::to_string_pretty(&self.config)
                    .expect("无法将 config 转换成 JSON 字符串");
                    file_config 
                        .write_all(updated_config_json.as_bytes()) 
                        .expect("写入文件失败了 error code 1919810"); 
                    self.add_in = false;
                    let current_exe_path = env::current_exe().expect("not get in path");
                    let args: Vec<String> = env::args().collect(); 
                    let path_cstr = CString::new(current_exe_path.as_os_str().as_bytes())
                        .expect("error in not get path");
                    let args_cstr: Vec<CString> = args
                        .iter()
                        .map(|arg| CString::new(arg.as_bytes()).expect("error in restart"))
                        .collect();
                    let args_ptr: Vec<&std::ffi::CStr> = args_cstr.iter().map(|c| c.as_c_str()).collect();
                    unistd::execv(&path_cstr, &args_ptr).expect("execy erroe");
                }
                break;
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            if !self.open {
                ctx.send_viewport_cmd(ViewportCommand::Close);
            }
            //ui.heading("");
            ui.horizontal(|ui| {
                ui.label(
                    "esc 键退出软件"
                    //"esc key to exit the app,"
                );
                ui.label(
                    "del 键进入删除模式"
                    //"del key to delete mod,"
                );
                ui.label(
                    "Enter 键进入添加模式"
                    //"enter key to add mod,"
                );
                ui.label(
                    "tap 键退出当前模式"
                    //"tap key to exit mod"
                );
            });
            if self.add_in {
                ui.label(RichText::new(
                    "添加模式"
                    //"add mod"
                ).color(Color32::from_rgb(255, 192, 203)));
                ui.label(RichText::new(
                    "添加成功后会重启软件应用更改"
                    //"add succeed the software will be restarted"
                ).color(Color32::PURPLE));
            } else if self.del {
                ui.label(RichText::new(
                    "删除模式"
                    //"delete mod"
                ).color(Color32::RED));
                ui.label(RichText::new(
                    "删除成功后会重启软件应用更改"
                    //"delete succeed The software will be restarted"
                ).color(Color32::PURPLE));
            }
            let mut jsq: usize = 0;
            for ai in 0..self.config.a3_name.len() {
                ui.horizontal(|ui| {
                    //let mut a_vec: Vec<Color32> = Vec::new();
                    let mut a_vec = vec![WHITE_OP, WHITE_OP, WHITE_OP, WHITE_OP];
                    /////////////////////////////////////////////////////////////////////////////////////////////////1_color
                    //if !self.input_log.is_empty() {
                    //    a_1 = if self.input_log[0] == self.config.a3_1[ai] {
                    //        GREEN_OP
                    //    } else {
                    //        BLUE_OP
                    //    };
                    //}
                    //
                    //////////////////////////////////////////////////////////////////////////////////////////////////2_color
                    //if self.input_log.len() > 1 {
                    //    a_2 = if self.input_log[1] == self.config.a3_2[ai] && a_1 == GREEN_OP {
                    //        GREEN_OP
                    //    } else {
                    //        BLUE_OP
                    //    };
                    //    if a_2 != GREEN_OP {
                    //        a_1 = BLUE_OP;
                    //    }
                    //}
                    //////////////////////////////////////////////////////////////////////////////////////////////////3_color
                    //if self.input_log.len() > 2 {
                    //    a_3 = if self.input_log[2] == self.config.a3_3[ai]
                    //        && a_1 == GREEN_OP
                    //        && a_2 == GREEN_OP
                    //    {
                    //        GREEN_OP
                    //    } else {
                    //        BLUE_OP
                    //    };
                    //   if a_3 != GREEN_OP {
                    //        a_1 = BLUE_OP;
                    //        a_2 = BLUE_OP;
                    //    }
                    //}
                    for aai in 0..3 {
                        input_rx(
                            ai,
                            self.input_log.clone(),
                            match aai {
                                0 => self.config.a3_1.clone(),
                                1 => self.config.a3_2.clone(),
                                2 => self.config.a3_3.clone(),
                                _ => unreachable!(),
                            },
                            &mut a_vec,
                            aai,
                        );
                    }
                    ///////////////////////////////////////////////////////////////////////////////////////////////name_color
                    if self.input_log.len() >= 3
                        && a_vec[1] == GREEN_OP
                        && a_vec[3] == GREEN_OP
                        && a_vec[2] == GREEN_OP
                    {
                        a_vec[0] = GREEN_OP;

                        if self.log_timer.is_none() && !self.add_in {
                            if !self.del {
                                self.log_timer = Some(Instant::now() + Duration::from_secs(1));
                                let _ = start(
                                    self.config.a3_path[ai].clone(),
                                    self.config.a3_c[ai].clone(),
                                    self.config.shell[0].clone(),
                                );
                            } else {
                                self.del_date = ai;
                            }
                        }
                    } else if a_vec[0] == GRAY_OP {
                        jsq += 1;
                    }
                    ////////////////////////////////////////////////////////////////////////////////////////////////////print
                    ui.label(RichText::new(&self.config.a3_1[ai]).color(a_vec[1]));
                    ui.label(RichText::new(&self.config.a3_2[ai]).color(a_vec[2]));
                    ui.label(RichText::new(&self.config.a3_3[ai]).color(a_vec[3]));
                    ui.label(RichText::new(&self.config.a3_name[ai]).color(a_vec[0]));
                });
            }
            ///////////////////////////////////////////////////////////////////////////////////////////////////b3
            for bi in 0..self.config.b4_name.len() {
                ui.horizontal(|ui| {
                    //let mut b_vec: Vec<Color32> = Vec::new();
                    let mut  b_vec = vec![WHITE_OP, WHITE_OP, WHITE_OP, WHITE_OP, WHITE_OP];
                    for bbi in 0..4 {
                        input_rx(
                            bi,
                            self.input_log.clone(),
                            match bbi {
                                0 => self.config.b4_1.clone(),
                                1 => self.config.b4_2.clone(),
                                2 => self.config.b4_3.clone(),
                                3 => self.config.b4_4.clone(),
                                _ => unreachable!(),
                            },
                            &mut b_vec,
                            bbi,
                        );
                    }

                    if self.input_log.len() > 3 && (1..=4).all(|i| b_vec[i] == GREEN_OP) {
                        b_vec[0] = GREEN_OP;
                        if self.log_timer.is_none() && !self.add_in {
                            if !self.del {
                                self.log_timer = Some(Instant::now() + Duration::from_secs(1));
                                let _ = start(
                                    self.config.b4_path[bi].clone(),
                                    self.config.b4_c[bi].clone(),
                                    self.config.shell[0].clone(),
                                );
                            } else {
                                self.del_date = self.config.a3_name.len() + bi;
                            }
                        }
                    } else if b_vec[0] == GRAY_OP {
                        jsq += 1;
                    }
                    ui.label(RichText::new(&self.config.b4_1[bi]).color(b_vec[1]));
                    ui.label(RichText::new(&self.config.b4_2[bi]).color(b_vec[2]));
                    ui.label(RichText::new(&self.config.b4_3[bi]).color(b_vec[3]));
                    ui.label(RichText::new(&self.config.b4_4[bi]).color(b_vec[4]));
                    ui.label(RichText::new(&self.config.b4_name[bi]).color(b_vec[0]));
                });
            }
            for ci in 0..self.config.c5_name.len() {
                ui.horizontal(|ui| {
                    //let mut c_vec: Vec<Color32> = Vec::new();
                    let mut c_vec = vec![WHITE_OP; 6];
                    for cci in 0..5 {
                        input_rx(
                            ci,
                            self.input_log.clone(),
                            match cci {
                                0 => self.config.c5_1.clone(),
                                1 => self.config.c5_2.clone(),
                                2 => self.config.c5_3.clone(),
                                3 => self.config.c5_4.clone(),
                                4 => self.config.c5_5.clone(),
                                _ => unreachable!(),
                            },
                            &mut c_vec,
                            cci,
                        );
                    }
                    if self.input_log.len() > 4 && (1..6).all(|i| c_vec[i] == GREEN_OP) {
                        c_vec[0] = GREEN_OP;
                        if self.log_timer.is_none() && !self.add_in {
                            if !self.del {
                                self.log_timer = Some(Instant::now() + Duration::from_secs(1));
                                let _ = start(
                                    self.config.c5_path[ci].clone(),
                                    self.config.c5_c[ci].clone(),
                                    self.config.shell[0].clone(),
                                );
                            } else {
                                self.del_date =
                                    self.config.a3_name.len() + self.config.b4_name.len() + ci;
                            }
                        }
                    } else if c_vec[0] == GRAY_OP {
                        jsq += 1;
                    }
                    ui.label(RichText::new(&self.config.c5_1[ci]).color(c_vec[1]));
                    ui.label(RichText::new(&self.config.c5_2[ci]).color(c_vec[2]));
                    ui.label(RichText::new(&self.config.c5_3[ci]).color(c_vec[3]));
                    ui.label(RichText::new(&self.config.c5_4[ci]).color(c_vec[4]));
                    ui.label(RichText::new(&self.config.c5_5[ci]).color(c_vec[5]));
                    ui.label(RichText::new(&self.config.c5_name[ci]).color(c_vec[0]));
                });
            }
            for di in 0..self.config.d_file_name.len() {
                ui.horizontal(|ui| {
                    //let mut d_vec: Vec<Color32> = Vec::new();
                    let mut d_vec = vec![WHITE_OP; 7];
                    for ddi in 0..6 {
                        input_rx(
                            di,
                            self.input_log.clone(),
                            match ddi {
                                0 => self.config.d_file1_json.clone(),
                                1 => self.config.d_file2_json.clone(),
                                2 => self.config.d_file3_json.clone(),
                                3 => self.config.d_file4_json.clone(),
                                4 => self.config.d_file5_json.clone(),
                                5 => self.config.d_file6_json.clone(),
                                _ => unreachable!(),
                            },
                            &mut d_vec,
                            ddi,
                        )
                    }
                    if self.input_log.len() > 5 && (1..7).all(|i| d_vec[i] == GREEN_OP) {
                        d_vec[0] = GREEN_OP;
                        if self.log_timer.is_none() && !self.add_in {
                            if !self.del {
                                self.log_timer = Some(Instant::now() + Duration::from_secs(1));
                                let _ = start(
                                    self.config.d_file_path[di].clone(),
                                    self.config.d6_c[di].clone(),
                                    self.config.shell[0].clone(),
                                );
                            } else {
                                self.del_date = self.config.a3_name.len()
                                    + self.config.b4_name.len()
                                    + self.config.c5_name.len()
                                    + di;
                            }
                        }
                    } else if d_vec[0] == GRAY_OP {
                        jsq += 1;
                    }
                    ui.label(RichText::new(&self.config.d_file1_json[di]).color(d_vec[1]));
                    ui.label(RichText::new(&self.config.d_file2_json[di]).color(d_vec[2]));
                    ui.label(RichText::new(&self.config.d_file3_json[di]).color(d_vec[3]));
                    ui.label(RichText::new(&self.config.d_file4_json[di]).color(d_vec[4]));
                    ui.label(RichText::new(&self.config.d_file5_json[di]).color(d_vec[5]));
                    ui.label(RichText::new(&self.config.d_file6_json[di]).color(d_vec[6]));
                    ui.label(RichText::new(&self.config.d_file_name[di]).color(d_vec[0]));
                });
            }
            for ei in 0..self.config.e7_name.len() {
                ui.horizontal(|ui| {
                    //let mut e_vec: Vec<Color32> = Vec::new();
                    let mut e_vec = vec![WHITE_OP; 8];
                    for eei in 0..7 {
                        input_rx(
                            ei,
                            self.input_log.clone(),
                            match eei {
                                0 => self.config.e7_1.clone(),
                                1 => self.config.e7_2.clone(),
                                2 => self.config.e7_3.clone(),
                                3 => self.config.e7_4.clone(),
                                4 => self.config.e7_5.clone(),
                                5 => self.config.e7_6.clone(),
                                6 => self.config.e7_7.clone(),
                                _ => unreachable!(),
                            },
                            &mut e_vec,
                            eei,
                        )
                    }
                    if self.input_log.len() > 6 && (1..8).all(|i| e_vec[i] == GREEN_OP) {
                        e_vec[0] = GREEN_OP;
                        if self.log_timer.is_none() && !self.add_in {
                            if !self.del {
                                self.log_timer = Some(Instant::now() + Duration::from_secs(1));
                                let _ = start(
                                    self.config.e7_path[ei].clone(),
                                    self.config.e7_c[ei].clone(),
                                    self.config.shell[0].clone(),
                                );
                            } else {
                                self.del_date = self.config.a3_name.len()
                                    + self.config.b4_name.len()
                                    + self.config.c5_name.len()
                                    + self.config.d_file_name.len()
                                    + ei;
                            }
                        }
                    } else if e_vec[0] == GRAY_OP {
                        jsq += 1;
                    }

                    ui.label(RichText::new(&self.config.e7_1[ei]).color(e_vec[1]));
                    ui.label(RichText::new(&self.config.e7_2[ei]).color(e_vec[2]));
                    ui.label(RichText::new(&self.config.e7_3[ei]).color(e_vec[3]));
                    ui.label(RichText::new(&self.config.e7_4[ei]).color(e_vec[4]));
                    ui.label(RichText::new(&self.config.e7_5[ei]).color(e_vec[5]));
                    ui.label(RichText::new(&self.config.e7_6[ei]).color(e_vec[6]));
                    ui.label(RichText::new(&self.config.e7_7[ei]).color(e_vec[7]));
                    ui.label(RichText::new(&self.config.e7_name[ei]).color(e_vec[0]));
                });
            }
            ui.horizontal(|ui| {
                for arrow in &self.input_log {
                    ui.label(*arrow);
                }
            });

            if self.add_in {
                ui.horizontal(|ui| {
                    ui.label(
                        "设置名称"
                    //    "config name"
                    );
                    ui.text_edit_singleline(&mut self.name);
                });
                ui.horizontal(|ui| {
                  if ui.button(
                        "使用终端运行命令"
                    //    "use Command line"
                    ).clicked() {
                        self.stbrt = "t".to_string();
                    } else if ui.button(
                        "使用默认方式打开软件或文件"
                    //    "use xdg-open for file or .disktop"
                    ).clicked() {
                        self.stbrt = "x".to_string();
                    }
                });
                ui.horizontal(|ui| {
                    ui.label(
                        "设置命令或软件路径"
                    //    "config Command or app path"
                    );
                    ui.text_edit_singleline(&mut self.peth);
                });
            }

            if self.yon {
                ui.label(RichText::new(
                    "请设置命令或路径以及名称"
                //  "please config Command or path and name"
                ).color(Color32::YELLOW));
                ui.label(RichText::new(
                    "事件索引长度应该大于等于3小于等于7"
                //        "input long time in 3 or 7 between"
                ).color(Color32::YELLOW));
                ui.label(RichText::new(
                    "命令或路径或路径不要输入英文双引号但可以用单引号"
                //    "Command or path is not have Double quotation marks"
                ).color(Color32::YELLOW));
            }

            if self.del_date < MAX {
                ui.horizontal(|ui| {
                    ui.label(
                        "确定删除全为绿色的索引吗"
                    //    "Are you sure to delete the indexes that are all green?"
                    );
                    if ui.button("yes").clicked() {
                        if self.del_date < self.config.a3_name.len() {
                            let temp = self.del_date;
                            self.config.a3_1.drain(temp..temp + 1);
                            self.config.a3_2.drain(temp..temp + 1);
                            self.config.a3_3.drain(temp..temp + 1);
                            self.config.a3_name.drain(temp..temp + 1);
                            self.config.a3_c.drain(temp..temp + 1);
                            self.config.a3_path.drain(temp..temp + 1);
                            self.del_date = 101;
                        } else if self.del_date
                            < self.config.a3_name.len() + self.config.b4_name.len()
                        {
                            let temp = self.del_date;
                            self.config.b4_1.drain(temp..temp + 1);
                            self.config.b4_2.drain(temp..temp + 1);
                            self.config.b4_3.drain(temp..temp + 1);
                            self.config.b4_4.drain(temp..temp + 1);
                            self.config.b4_name.drain(temp..temp + 1);
                            self.config.b4_path.drain(temp..temp + 1);
                            self.config.b4_c.drain(temp..temp + 1);
                            self.del_date = 101;
                        } else if self.del_date
                            < self.config.a3_name.len()
                                + self.config.b4_name.len()
                                + self.config.c5_name.len()
                        {
                            let temp = self.del_date;
                            self.config.c5_1.drain(temp..temp + 1);
                            self.config.c5_2.drain(temp..temp + 1);
                            self.config.c5_3.drain(temp..temp + 1);
                            self.config.c5_4.drain(temp..temp + 1);
                            self.config.c5_5.drain(temp..temp + 1);
                            self.config.c5_name.drain(temp..temp + 1);
                            self.config.c5_path.drain(temp..temp + 1);
                            self.config.c5_c.drain(temp..temp + 1);
                            self.del_date = 101;
                        } else if self.del_date
                            < self.config.a3_name.len()
                                + self.config.b4_name.len()
                                + self.config.c5_name.len()
                                + self.config.d_file_name.len()
                        {
                            let temp = self.del_date;
                            self.config.d_file1_json.drain(temp..temp + 1);
                            self.config.d_file2_json.drain(temp..temp + 1);
                            self.config.d_file3_json.drain(temp..temp + 1);
                            self.config.d_file4_json.drain(temp..temp + 1);
                            self.config.d_file5_json.drain(temp..temp + 1);
                            self.config.d_file6_json.drain(temp..temp + 1);
                            self.config.d_file_name.drain(temp..temp + 1);
                            self.config.d_file_path.drain(temp..temp + 1);
                            self.config.d6_c.drain(temp..temp + 1);
                            self.del_date = 101;
                        } else if self.del_date
                            < self.config.a3_name.len()
                                + self.config.b4_name.len()
                                + self.config.c5_name.len()
                                + self.config.d_file_name.len()
                                + self.config.e7_name.len()
                        {
                            let temp = self.del_date;
                            self.config.e7_1.drain(temp..temp + 1);
                            self.config.e7_2.drain(temp..temp + 1);
                            self.config.e7_3.drain(temp..temp + 1);
                            self.config.e7_4.drain(temp..temp + 1);
                            self.config.e7_5.drain(temp..temp + 1);
                            self.config.e7_6.drain(temp..temp + 1);
                            self.config.e7_7.drain(temp..temp + 1);
                            self.config.e7_name.drain(temp..temp + 1);
                            self.config.e7_c.drain(temp..temp + 1);
                            self.config.e7_path.drain(temp..temp + 1);
                            self.del_date = 101;
                        }
                        if self.del_date == 101 {
                            let mut file_config =
                                File::create(&self.config_path).expect("error code 1111");
                            let updated_config_json = serde_json::to_string_pretty(&self.config)
                            .expect("无法将 config 转换成 JSON 字符串");
                            file_config // 假设 file_config 是打开的文件句柄喵~
                            .write_all(updated_config_json.as_bytes()) 
                            .expect("写入文件失败了 error code 1919810"); 
                            self.del = false;
                            self.del_date = MAX;
                            let current_exe_path = env::current_exe().expect("not get in path");
                            let args: Vec<String> = env::args().collect(); 
                            let path_cstr = CString::new(current_exe_path.as_os_str().as_bytes())
                                .expect("error in not get path");
                            let args_cstr: Vec<CString> = args
                                .iter()
                                .map(|arg| CString::new(arg.as_bytes()).expect("error in restart"))
                                .collect();
                            let args_ptr: Vec<&std::ffi::CStr> = args_cstr.iter().map(|c| c.as_c_str()).collect();
                            unistd::execv(&path_cstr, &args_ptr).expect("execy erroe");
                        }
                    }else if ui.button("no").clicked() {
                        self.del = false;
                        self.del_date = MAX;
                        self.input_log.clear();
                    }
                });
            }

            if self.no_have_shell {
                ui.label(
                    "检查到没有设置默认虚拟终端请设置"
                //    "Check that no virtual emulation terminal is set up,please input terminal name"
                );
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.name);
                    if ui.button("yes").clicked() {
                        self.config.shell.push(self.name.to_string());
                        let mut file_config =
                            File::create(&self.config_path).expect("error code 1111");
                        let updated_config_json = serde_json::to_string_pretty(&self.config)
                            .expect("无法将 config 转换成 JSON 字符串");
                        file_config // 假设 file_config 是打开的文件句柄喵~
                            .write_all(updated_config_json.as_bytes())
                            .expect("写入文件失败error code 1919810");
                        self.name = "".to_string();
                        self.no_have_shell = false;
                        let current_exe_path = env::current_exe().expect("not get in path");
                        let args: Vec<String> = env::args().collect(); 
                        let path_cstr = CString::new(current_exe_path.as_os_str().as_bytes())
                            .expect("error in not get path");
                        let args_cstr: Vec<CString> = args
                            .iter()
                            .map(|arg| CString::new(arg.as_bytes()).expect("error in restart"))
                            .collect();
                        let args_ptr: Vec<&std::ffi::CStr> = args_cstr.iter().map(|c| c.as_c_str()).collect();
                        unistd::execv(&path_cstr, &args_ptr).expect("execv erroe");
                    }
                });
            };

            if jsq == self.max_input_len && self.log_timer.is_none() {
                self.log_timer = Some(Instant::now() + Duration::from_secs(1));
            }

            if let Some(target_time) = self.log_timer {
                if !self.add_in && Instant::now() >= target_time {
                    self.time_clear = true; //通过时间判断清除
                    self.log_timer = None; 
                }
            }

            if self.time_clear {
                self.time_clear = false;
                self.input_log.clear();
            }

            ui.separator();
        });
        ctx.request_repaint();
    }
}

fn input_rx(
    xi: usize,
    input: Vec<&'static str>,
    xx_x: Vec<String>,
    xx: &mut [Color32],
    number: usize,
) {
    if input.len() > number && input[number] == xx_x[xi] {
        if (1..number + 1).all(|i| xx[i] == GREEN_OP) {
            xx[number + 1] = GREEN_OP;
        } else {
            xx[0..number + 2].fill(GRAY_OP);
        }
    } else if input.len() > number && input[number] != xx_x[xi] {
        let ccc = xx.len();
        xx[0..ccc].fill(GRAY_OP);
    } else if xx[number] == GRAY_OP {
        xx[number + 1] = GRAY_OP;
    } else {
        xx[number + 1] = WHITE_OP;
    }
}

fn start(input_path: String, ccc: String, sh: String) -> io::Result<()> {
    if ccc == "t" {
        let escaped_input_path = input_path.replace("'", "'\\''");
        let mut cmd = Command::new(&sh);
        cmd.arg("-e"); 
        cmd.arg("sh");
        cmd.arg("-c");
        cmd.arg(&escaped_input_path);
        cmd.stdin(Stdio::null());
        match cmd.spawn() {
            Ok(_child) => Ok(()),
            Err(e) => {
                if e.kind() == io::ErrorKind::NotFound {
    
                    println!("终端 '{}' 未找到", &sh);
                    Ok(())
                } else {
        
                    println!("未知错误{}", e);
                    Err(e)
                }
            }
        }
    } else {
        let fill_path = shellexpand::tilde(&input_path).to_string();
        
        match Command::new("xdg-open").arg(fill_path.clone()).spawn() {
            Ok(_child) => {
                
                println!("尝试使用 xdg-open 启动...");
                Ok(()) 
            }
            Err(e) => {
    
                let err_msg = format!("启动 xdg-open 失败 {}", e);
                println!("{}", err_msg);
                Err(e) 
            }
        }
    }
}

fn read_json<P: AsRef<std::path::Path>>(path: P) -> Result<Config> {
    let file = File::open(path).with_context(|| "无法找的配置文件")?;
    let reader = BufReader::new(file);
    serde_json::from_reader(reader).with_context(|| "文件打开错误")
}

//添加字体
fn setup_custom_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "my_font".to_owned(),
        egui::FontData::from_static(include_bytes!("../src/noto.ttf")).into(),
    );

    // Put my font first (highest priority) for proportional text:
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        //.insert(0, "my_font".to_owned());
        .push("my_font".to_owned());

    // Put my font as last fallback for monospace:
    fonts
        .families
        .entry(egui::FontFamily::Monospace)
        .or_default()
        .push("my_font".to_owned());

    // Tell egui to use these fonts:
    ctx.set_fonts(fonts);
}
