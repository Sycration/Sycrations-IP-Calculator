#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use egui::*;
use env_logger::*;
use log::*;
use std::{net::*, str::FromStr};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([450.0, 240.0]),
        ..Default::default()
    };

    // Our application state:
    let mut host_ip_str = "192.168.0.1".to_owned();
    let mut mask_bits = 24;
    let mut subnet_mask_str = "255.255.255.0".to_owned();
    let mut network_ip_str = "192.168.0.0".to_owned();
    let mut broadcast_ip_str = "192.168.0.255".to_owned();

    let mut host_ip = Ipv4Addr::from_str(&host_ip_str);
    let mut subnet_mask = Ipv4Addr::from_str(&subnet_mask_str);
    let mut network_ip = Ipv4Addr::from_str(&network_ip_str);
    let mut broadcast_ip = Ipv4Addr::from_str(&broadcast_ip_str);
    eframe::run_simple_native("Sycration's IP Calculator", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Host IP address");
            ui.horizontal(|ui| {
                if ui.text_edit_singleline(&mut host_ip_str).changed() {
                    host_ip = Ipv4Addr::from_str(&host_ip_str);
                    if let Ok(ip) = host_ip {
                        if let Ok(sm) = subnet_mask {
                            calculate_addresses(
                                sm,
                                ip,
                                &mut network_ip,
                                &mut network_ip_str,
                                &mut broadcast_ip,
                                &mut broadcast_ip_str,
                            );
                        }
                    }
                };
                ui.label("/");
                if ui
                    .add(
                        widgets::DragValue::new(&mut mask_bits)
                            .speed(1)
                            .range(0..=32),
                    )
                    .changed()
                {
                    let mut new_sm = 0;

                    for b in 0..mask_bits {
                        new_sm += 2u32.pow(31 - b);
                    }
                    subnet_mask = Ok(Ipv4Addr::from_bits(new_sm));
                    subnet_mask_str = Ipv4Addr::from_bits(new_sm).to_string();
                    if let Ok(ip) = host_ip {
                        if let Ok(sm) = subnet_mask {
                            calculate_addresses(
                                sm,
                                ip,
                                &mut network_ip,
                                &mut network_ip_str,
                                &mut broadcast_ip,
                                &mut broadcast_ip_str,
                            );
                        }
                    }
                };
            });
            ui.separator();
            ui.label("Subnet mask");
            ui.horizontal(|ui| {
                if ui.text_edit_singleline(&mut subnet_mask_str).changed() {
                    subnet_mask = Ipv4Addr::from_str(&subnet_mask_str);
                    if let Ok(sm) = subnet_mask {
                        let mut new_sm = 0u32;
                        for b in 0..sm.to_bits().count_ones() {
                            new_sm += 2u32.pow(31 - b);
                        }
                        if new_sm == sm.to_bits() {
                            mask_bits = sm.to_bits().leading_ones();
                            subnet_mask = Ok(sm);
                            subnet_mask_str = sm.to_string();
                            if let Ok(ip) = host_ip {
                                calculate_addresses(
                                    sm,
                                    ip,
                                    &mut network_ip,
                                    &mut network_ip_str,
                                    &mut broadcast_ip,
                                    &mut broadcast_ip_str,
                                );
                            }
                        }
                    }
                };
                if let Ok(sm) = subnet_mask {
                    let mut new_sm = 0u32;
                    for b in 0..sm.to_bits().leading_ones() {
                        new_sm += 2u32.pow(31 - b);
                    }
                    if new_sm != sm.to_bits() {
                        if ui.button("invalid mask, click to fix").clicked() {
                            subnet_mask = Ok(Ipv4Addr::from_bits(new_sm));
                            subnet_mask_str = Ipv4Addr::from_bits(new_sm).to_string();
                        }
                    }
                }
            });

            ui.separator();
            ui.label("Network address");
            ui.text_edit_singleline(&mut network_ip_str.clone());
            ui.separator();
            ui.label("Broadcast address");
            ui.text_edit_singleline(&mut broadcast_ip_str.clone());
            ui.separator();
            ui.label(format!("{} host addresses available", {
                2u64.pow(32u32.saturating_sub(mask_bits)).saturating_sub(2)
            }));
        });
    })
}

fn calculate_addresses(
    sm: Ipv4Addr,
    ip: Ipv4Addr,
    network_ip: &mut Result<Ipv4Addr, AddrParseError>,
    network_ip_str: &mut String,
    broadcast_ip: &mut Result<Ipv4Addr, AddrParseError>,
    broadcast_ip_str: &mut String,
) {
    let new_bits = sm.to_bits() & ip.to_bits();
    let new_ip = Ipv4Addr::from_bits(new_bits);
    *network_ip = Ok(new_ip);
    *network_ip_str = new_ip.to_string();

    let new_bits = !sm.to_bits() | ip.to_bits();
    let new_ip = Ipv4Addr::from_bits(new_bits);
    *broadcast_ip = Ok(new_ip);
    *broadcast_ip_str = new_ip.to_string();
}
