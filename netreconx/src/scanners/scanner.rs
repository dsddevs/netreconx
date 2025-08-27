use std::fs::File;
use std::io::{BufWriter, Write};
use crate::ports::port::Port;
use crate::ports::port_addr::PORTS_100;
use crate::subdomains::subdomain::Subdomains;
use crate::config::settings::ScanConfig;
use rayon::prelude::*;
use reqwest::blocking::Client;
use std::net::{SocketAddr, ToSocketAddrs};
use std::thread;

const NON_REDUNDANT_PORT: i16 = 2025;

pub fn print_scan_result(client: &Client, domain: &str) -> Result<(), anyhow::Error> {
    print_scan_result_with_config(client, domain, &ScanConfig::default())
}

pub fn print_scan_result_with_config(client: &Client, domain: &str, config: &ScanConfig) -> Result<(), anyhow::Error> {
    let scanned_subdomains = scan_subdomains(client, domain, config)?;
    let file = File::create("results/subdomains.text");
    let mut recorder = BufWriter::new(file?);

    for target in scanned_subdomains {
        if target.open_ports.is_empty() {
            continue
        }
        let ports: Vec<String> = target.open_ports.iter().map(|p| p.port.to_string()).collect();
        let subdomains = format!("{0}:{1}\n", target.subdomain, ports.join(","));
        recorder.write_all(subdomains.as_bytes())?;
        
        // Apply rate limiting between subdomain processing
        thread::sleep(config.rate_limit_delay);
    }
    println!("[+] Successfully subdomains scanned");
    Ok(())
}

fn scan_subdomains(client: &Client, domain: &str, config: &ScanConfig) -> Result<Vec<Subdomains>, anyhow::Error> {
    let subdomains = Subdomains::create(client, domain)?;
    let scanned_subdomains = subdomains.into_par_iter().map(|subdomain| with_open_ports(subdomain, config)).collect();
    Ok(scanned_subdomains)
}

fn with_open_ports(mut subdomains: Subdomains, config: &ScanConfig) -> Subdomains {
    let socket_addrs = create_socket_addrs(&mut subdomains);
    if socket_addrs.is_empty() {
        return subdomains;
    }
    subdomains.open_ports = get_open_ports(&socket_addrs, config);
    subdomains
}

fn get_open_ports(socket_addrs: &[SocketAddr], config: &ScanConfig) -> Vec<Port> {
    PORTS_100
        .par_iter()
        .map(|port| Port::connect_with_timeout(socket_addrs[0], *port, config.connection_timeout))
        .filter(|port| port.is_open)
        .collect()
}

fn create_socket_addrs(subdomains: &mut Subdomains) -> Vec<SocketAddr> {
    match format!("{0}:{1}", subdomains.subdomain, NON_REDUNDANT_PORT)
        .to_socket_addrs() {
        Ok(addrs) => addrs.collect(),
        Err(_) => Vec::new(),
    }
}
