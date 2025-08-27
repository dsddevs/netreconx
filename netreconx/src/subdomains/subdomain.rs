use crate::certificates::crt::Certificates;
use crate::config::settings::ScanConfig;
use crate::ports::port::Port;
use rayon::prelude::*;
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashSet;
use trust_dns_resolver::config::{ResolverConfig, ResolverOpts};
use trust_dns_resolver::proto::rr::RecordType;
use trust_dns_resolver::Resolver;

#[derive(Debug, Deserialize, Clone)]
pub struct Subdomains {
    pub subdomain: String,
    pub open_ports: Vec<Port>,
}

impl Subdomains {
    pub fn create(client: &Client, domain: &str) -> Result<Vec<Subdomains>, anyhow::Error> {
        let url = format!("https://crt.sh/?q=%25.{}&output=json", domain);
        println!("[*] Querying crt.sh for domain: {}", domain);
        
        // Try multiple times with exponential backoff
        let mut _last_error: Option<anyhow::Error> = None;
        for attempt in 1..=3 {
            println!("[*] Attempt {} of 3", attempt);
            
            match client.get(&url).send() {
                Ok(response) => {
                    let status = response.status();
                    println!("[*] Response status: {}", status);
                    
                    match response.text() {
                        Ok(response_text) => {
                            println!("[*] Response length: {} bytes", response_text.len());
                            
                            if !status.is_success() {
                                println!("[!] API returned error status: {}", status);
                                println!("[!] Response body: {}", &response_text[..std::cmp::min(200, response_text.len())]);
                                break;
                            }
                            
                            if response_text.trim().is_empty() {
                                println!("[!] Empty response from crt.sh for domain: {}", domain);
                                break;
                            }
                            
                            match serde_json::from_str::<Vec<Certificates>>(&response_text) {
                                Ok(certificates) => {
                                    println!("[+] Successfully parsed {} certificates", certificates.len());
                                    let mut filtered_domains = filter_domains(certificates, domain);
                                    filtered_domains.insert(domain.into());
                                    let subdomains = add_port(filtered_domains);
                                    return Ok(subdomains);
                                },
                                Err(e) => {
                                    println!("[!] Failed to parse JSON response: {}", e);
                                    println!("[!] Response preview: {}", &response_text[..std::cmp::min(200, response_text.len())]);
                                    break;
                                }
                            };
                        },
                        Err(e) => {
                            println!("[!] Failed to read response body: {}", e);
                            _last_error = Some(e.into());
                        }
                    }
                },
                Err(e) => {
                    println!("[!] Request failed on attempt {}: {}", attempt, e);
                    _last_error = Some(e.into());
                    
                    if attempt < 3 {
                        let delay = std::time::Duration::from_secs(attempt * 2);
                        println!("[*] Waiting {} seconds before retry...", delay.as_secs());
                        std::thread::sleep(delay);
                    }
                }
            }
        }
        
        println!("[!] All attempts failed, falling back to scanning main domain only");
        Ok(vec![Subdomains {
            subdomain: domain.to_string(),
            open_ports: Vec::new(),
        }])
    }
}
fn filter_domains(crts: Vec<Certificates>, domain: &str) -> HashSet<String> {
    crts.into_par_iter()
        .flat_map(|crt| {
            crt.name_value
                .split("\n")
                .map(|subdomains| subdomains.trim().to_string())
                .collect::<Vec<String>>()
        })
        .filter(|subdomain| subdomain != domain)
        .filter(|subdomain| !subdomain.contains('*'))
        .collect()
}

fn add_port(subdomains: HashSet<String>) -> Vec<Subdomains> {
    subdomains
        .into_iter()
        .map(|subdomain| Subdomains {
            subdomain,
            open_ports: Vec::new(),
        })
        .filter(resolves)
        .collect()
}

fn resolves(subdomain: &Subdomains) -> bool {
    resolves_with_config(subdomain, &ScanConfig::default())
}

fn resolves_with_config(subdomain: &Subdomains, config: &ScanConfig) -> bool {
    let mut resolver_opts = ResolverOpts::default();
    let resolver_config = ResolverConfig::default();

    resolver_opts.timeout = config.dns_timeout;
    let dns_resolve = Resolver::new(resolver_config, resolver_opts).unwrap();
    dns_resolve.lookup(&subdomain.subdomain, RecordType::A).is_ok()
}
