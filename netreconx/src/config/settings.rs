use std::time::Duration;

#[derive(Debug, Clone)]
pub struct ScanConfig {
    pub max_threads: usize,
    pub connection_timeout: Duration,
    pub dns_timeout: Duration,
    pub http_timeout: Duration,
    pub max_redirects: usize,
    pub rate_limit_delay: Duration,
}

impl Default for ScanConfig {
    fn default() -> Self {
        Self {
            max_threads: std::thread::available_parallelism()
                .map(|n| n.get() * 2)
                .unwrap_or(8),
            connection_timeout: Duration::from_secs(3),
            dns_timeout: Duration::from_secs(4),
            http_timeout: Duration::from_secs(30),
            max_redirects: 4,
            rate_limit_delay: Duration::from_millis(50),
        }
    }
}

impl ScanConfig {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self::default()
    }
    
    #[allow(dead_code)]
    pub fn with_max_threads(mut self, threads: usize) -> Self {
        self.max_threads = threads;
        self
    }
    
    #[allow(dead_code)]
    pub fn with_timeouts(mut self, connection: Duration, dns: Duration, http: Duration) -> Self {
        self.connection_timeout = connection;
        self.dns_timeout = dns;
        self.http_timeout = http;
        self
    }
}
