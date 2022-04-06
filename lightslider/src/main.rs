#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use std::io::BufRead;
use std::io::BufReader;
use std::fs;
use std::fs::File;
use eframe::{egui, epi};
use std::path::Path;
use egui::Color32;
//use egui::widgets::Slider;
use std::process::Command;
struct MyApp {
    light: f64,
    vol: f64,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            light: String::from_utf8_lossy(&Command::new("light").arg("-G").output().unwrap().stdout).trim().parse().unwrap(),
            vol: String::from_utf8_lossy(&Command::new("pamixer").arg("--get-volume").output().unwrap().stdout).trim().parse().unwrap()
        }
    }
}

impl epi::App for MyApp {
    fn name(&self) -> &str {
        "Brightness"
    }
    
    fn clear_color(&self) ->  egui::Rgba {
    	egui::Rgba::TRANSPARENT
    }

    fn update(&mut self, ctx: &egui::Context, frame: &epi::Frame) {
        let mut style: egui::Style = (*ctx.style()).clone();
        //frame.set_window_size(egui::vec2(20., 802.));
        //style.spacing.slider_width = frame.take_app_output().window_size.y;
		style.spacing.slider_width = 500.;
		ctx.set_style(style);
		
        egui::SidePanel::left("my_left_panel").frame(egui::Frame::default().fill(egui::Color32::TRANSPARENT)).show(ctx, |ui| {
        	if ui.add(egui::Slider::new(&mut self.light, 1.0..=99.9).smart_aim(false).vertical().step_by(0.1).show_value(false)).changed(){
        		Command::new("light").args(["-S", &self.light.to_string()]).status();
        	}   
		});	
		egui::SidePanel::left("my_2left_panel").frame(egui::Frame::default().fill(egui::Color32::TRANSPARENT)).show(ctx, |ui| {
        	if ui.button("Close").clicked() {
		        frame.quit();
		    }
		});
		egui::SidePanel::right("my_right_panel").frame(egui::Frame::default().fill(egui::Color32::TRANSPARENT)).show(ctx, |ui| {
        	if ui.add(egui::Slider::new(&mut self.vol, 1.0..=99.9).smart_aim(false).vertical().step_by(0.1).show_value(false)).changed(){
        		Command::new("pamixer").args(["--set-volume", &self.vol.to_string()]).status();
        	}   
		});	
        //frame.set_window_size(ctx.used_size());
    }
}

fn main() {
    let mut options = eframe::NativeOptions::default();
    options.resizable=false;
    //options.decorated=false;
    options.transparent=true;
    //options.maximized=true;
    
    
    eframe::run_native(Box::new(MyApp::default()), options);
}
