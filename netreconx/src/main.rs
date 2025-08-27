use crate::clients::client::build_client;
use crate::errors::error::Errors;
use crate::scanners::scanner::print_scan_result;
use crate::threads::thread::build_thread_pool;

mod certificates;
mod clients;
mod config;
mod errors;
mod ports;
mod subdomains;
mod scanners;
mod threads;

fn main() -> Result<(), anyhow::Error> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return Err(Errors::CliErr.into());
    }

    let domain = &args[1];
    
    // Validate domain format
    if !is_valid_domain(domain) {
        return Err(Errors::InvalidDomain(domain.to_string()).into());
    }

    let client = build_client()?;
    let thread_pool = build_thread_pool()?;
    thread_pool.install(|| print_scan_result(&client, domain))?;

    Ok(())
}

fn is_valid_domain(domain: &str) -> bool {
    !domain.is_empty() && 
    domain.len() <= 253 &&
    !domain.starts_with('.') &&
    !domain.ends_with('.') &&
    domain.chars().all(|c| c.is_alphanumeric() || c == '.' || c == '-')
}
