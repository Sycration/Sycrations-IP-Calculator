#![windows_subsystem = "windows"]

use iced::widget::text_input::{Icon, Status};
use iced::widget::{
    button, center, checkbox, column, horizontal_rule, pick_list, progress_bar, row, rule,
    scrollable, slider, text, text_input, toggler, vertical_rule, vertical_space,
};
use iced::{Center, Element, Fill, Size, Theme};

use iced_aw::{iced_fonts, number_input, NumberInput};
use std::net::{AddrParseError, Ipv4Addr};
use std::str::FromStr;
use std::sync::LazyLock;

pub fn main() -> iced::Result {
    iced::application("Sycration's IP calculator", App::update, App::view)
        .theme(App::theme)
        .window_size(Size::new(350f32, 430f32))
        .resizable(false)
        .font(iced_fonts::REQUIRED_FONT_BYTES)
        .run()
}

struct App {
    host_ip_str: String,
    subnet_mask_str: String,
    subnet_mask_bits: u32,
    network_ip_str: String,
    broadcast_ip_str: String,
    //
    host_ip: Result<Ipv4Addr, AddrParseError>,
    subnet_mask: Result<Ipv4Addr, AddrParseError>,
    network_ip: Result<Ipv4Addr, AddrParseError>,
    broadcast_ip: Result<Ipv4Addr, AddrParseError>,
}

impl Default for App {
    fn default() -> Self {
        let host_ip_str = "192.168.0.1".to_owned();
        let subnet_mask_str = "255.255.255.0".to_owned();
        let network_ip_str = "192.168.0.0".to_owned();
        let broadcast_ip_str = "192.168.0.255".to_owned();
        App {
            host_ip_str: host_ip_str.clone(),
            subnet_mask_str: subnet_mask_str.clone(),
            subnet_mask_bits: 24,
            network_ip_str: network_ip_str.clone(),
            broadcast_ip_str: broadcast_ip_str.clone(),
            host_ip: Ipv4Addr::from_str(&host_ip_str),
            subnet_mask: Ipv4Addr::from_str(&subnet_mask_str),
            network_ip: Ipv4Addr::from_str(&network_ip_str),
            broadcast_ip: Ipv4Addr::from_str(&broadcast_ip_str),
        }
    }
}

static THEME: LazyLock<Theme> = LazyLock::new(|| {
    let mode = dark_light::detect();
    match mode {
        dark_light::Mode::Dark => Theme::Dracula,
        dark_light::Mode::Light => Theme::Light,
        dark_light::Mode::Default => Theme::Dracula,
    }
});

#[derive(Debug, Clone)]
enum Message {
    HostIpChanged(String),
    SMBitsChanged(u32),
    SubnetMaskChanged(String),
    NetworkIpChanged(String),
    BroadcastIpChanged(String),
}

impl App {
    fn update(&mut self, message: Message) {
        match message {
            Message::HostIpChanged(ip_str) => {
                self.host_ip_str = ip_str;
                self.host_ip = Ipv4Addr::from_str(&self.host_ip_str);
                if let Ok(ip) = self.host_ip {
                    if let Ok(sm) = self.subnet_mask {
                        self.calculate_addresses(sm, ip);
                    }
                }
            }
            Message::SMBitsChanged(new_bits) => {
                self.subnet_mask_bits = new_bits;
                let mut new_sm = 0;

                for b in 0..new_bits {
                    new_sm += 2u32.pow(31 - b);
                }
                self.subnet_mask = Ok(Ipv4Addr::from_bits(new_sm));
                self.subnet_mask_str = Ipv4Addr::from_bits(new_sm).to_string();
                if let Ok(ip) = self.host_ip {
                    if let Ok(sm) = self.subnet_mask {
                        self.calculate_addresses(sm, ip);
                    }
                }
            }
            Message::SubnetMaskChanged(sm_str) => {
                self.subnet_mask_str = sm_str;
                self.subnet_mask = Ipv4Addr::from_str(&self.subnet_mask_str);
                if let Ok(sm) = self.subnet_mask {
                    let mut new_sm = 0u32;
                    for b in 0..sm.to_bits().count_ones() {
                        new_sm += 2u32.pow(31 - b);
                    }
                    if new_sm == sm.to_bits() {
                        self.subnet_mask_bits = sm.to_bits().leading_ones();
                        self.subnet_mask = Ok(sm);
                        self.subnet_mask_str = sm.to_string();
                        if let Ok(ip) = self.host_ip {
                                self.calculate_addresses(sm, ip);
                        }
                    }
                }
            }
            Message::NetworkIpChanged(_) => {}
            Message::BroadcastIpChanged(_) => {}
        }
    }

    fn view(&self) -> Element<Message> {
        let content = column![
            text("Host IP address"),
            row![
                text_input("", &self.host_ip_str).on_input(Message::HostIpChanged),
                text("/").size(25),
                number_input(self.subnet_mask_bits, 0..=32, Message::SMBitsChanged)
            ]
            .spacing(10),
            horizontal_rule(2),
            text("Network mask"),
            text_input("", &self.subnet_mask_str).on_input(Message::SubnetMaskChanged),
            horizontal_rule(2),
            text("Network address"),
            text_input("", &self.network_ip_str).on_input(Message::NetworkIpChanged),
            horizontal_rule(2),
            text("Broadcast address"),
            text_input("", &self.broadcast_ip_str).on_input(Message::BroadcastIpChanged),
            horizontal_rule(2),
            text(format!("{} host addresses available", {
                2u64.pow(32u32.saturating_sub(self.subnet_mask_bits))
                    .saturating_sub(2)
            }))
        ]
        .spacing(10)
        .padding(20)
        .max_width(600);

        center(content).into()
    }

    fn theme(&self) -> Theme {
        THEME.clone()
    }

    fn calculate_addresses(
        &mut self,
        sm: Ipv4Addr,
        ip: Ipv4Addr,
    ) {
        let new_bits = sm.to_bits() & ip.to_bits();
        let new_ip = Ipv4Addr::from_bits(new_bits);
        self.network_ip = Ok(new_ip);
        self.network_ip_str = new_ip.to_string();
    
        let new_bits = !sm.to_bits() | ip.to_bits();
        let new_ip = Ipv4Addr::from_bits(new_bits);
        self.broadcast_ip = Ok(new_ip);
        self.broadcast_ip_str = new_ip.to_string();
    }
    


}

