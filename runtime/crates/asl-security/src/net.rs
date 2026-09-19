//! Sandboxed HTTP networking client confined by domain allowlist.

use asl_core_traits::HttpResponsePayload;
use asl_spec::{AslError, Result};
use std::time::Duration;

/// Extracts hostname from a URL string (e.g. "https://api.github.com/v3" -> "api.github.com")
pub fn extract_domain(url_str: &str) -> Result<String> {
    let clean = url_str.trim();
    let without_proto = if let Some(stripped) = clean.strip_prefix("https://") {
        stripped
    } else if let Some(stripped) = clean.strip_prefix("http://") {
        stripped
    } else {
        return Err(AslError::CapabilityViolation(format!(
            "Invalid HTTP protocol in URL: '{}'. Only http:// and https:// are permitted.",
            url_str
        )));
    };

    let authority = without_proto
        .split(['/', '?', '#'])
        .next()
        .unwrap_or("")
        .trim();

    let host_port = authority
        .rsplit_once('@')
        .map(|(_, hp)| hp)
        .unwrap_or(authority);

    let host = if host_port.starts_with('[') {
        host_port
            .find(']')
            .map(|i| &host_port[1..i])
            .unwrap_or(host_port)
    } else {
        host_port.split(':').next().unwrap_or("")
    }
    .trim()
    .trim_end_matches('.');

    if host.is_empty() {
        return Err(AslError::CapabilityViolation(format!(
            "Could not determine host domain in URL: '{}'",
            url_str
        )));
    }

    Ok(host.to_lowercase())
}

/// Verifies whether the target domain is authorized by the capabilities allowlist
pub fn is_domain_allowed(domain: &str, allowed_domains: &[String]) -> bool {
    let target = domain.to_lowercase();
    allowed_domains.iter().any(|allowed| {
        let a = allowed.trim().trim_end_matches('.').to_lowercase();
        a == "*" || target == a || target.ends_with(&format!(".{}", a))
    })
}

fn convert_response(
    res: std::result::Result<ureq::http::Response<ureq::Body>, ureq::Error>,
) -> Result<HttpResponsePayload> {
    match res {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let mut res_headers = Vec::new();
            for (k, v) in resp.headers() {
                if let Ok(val_str) = v.to_str() {
                    res_headers.push((k.as_str().to_string(), val_str.to_string()));
                }
            }
            let mut body = resp.into_body();
            let body_str = body.read_to_string().map_err(|e| {
                AslError::Io(format!("Failed to read HTTP response body: {}", e))
            })?;
            Ok(HttpResponsePayload {
                status,
                headers: res_headers,
                body: body_str,
            })
        }
        Err(e) => Err(AslError::Io(format!("HTTP request failed: {}", e))),
    }
}

/// Executes an attenuated, sandboxed HTTP request
pub fn execute_http_request(
    method: &str,
    url_str: &str,
    headers: &[(String, String)],
    body: Option<&str>,
    allowed_domains: &[String],
    timeout_ms: u64,
) -> Result<HttpResponsePayload> {
    let domain = extract_domain(url_str)?;
    if !is_domain_allowed(&domain, allowed_domains) {
        return Err(AslError::CapabilityViolation(format!(
            "Network access denied: domain '{}' is not authorized in capabilities.net.allow_domains ({:?})",
            domain, allowed_domains
        )));
    }

    let host = domain.split(':').next().unwrap_or(&domain);
    if is_private_or_metadata_host(host) {
        return Err(AslError::CapabilityViolation(format!(
            "Access to private/metadata IP or localhost is prohibited: '{}'",
            host
        )));
    }

    let timeout = Duration::from_millis(if timeout_ms == 0 { 15000 } else { timeout_ms });
    let config = ureq::config::Config::builder()
        .timeout_global(Some(timeout))
        .max_redirects(0)
        .build();
    let agent: ureq::Agent = config.into();

    let method_upper = method.trim().to_uppercase();
    match method_upper.as_str() {
        "POST" => {
            let mut b = agent.post(url_str);
            for (k, v) in headers {
                b = b.header(k.as_str(), v.as_str());
            }
            let res = if let Some(text) = body {
                b.send(text)
            } else {
                b.send_empty()
            };
            convert_response(res)
        }
        "PUT" => {
            let mut b = agent.put(url_str);
            for (k, v) in headers {
                b = b.header(k.as_str(), v.as_str());
            }
            let res = if let Some(text) = body {
                b.send(text)
            } else {
                b.send_empty()
            };
            convert_response(res)
        }
        "PATCH" => {
            let mut b = agent.patch(url_str);
            for (k, v) in headers {
                b = b.header(k.as_str(), v.as_str());
            }
            let res = if let Some(text) = body {
                b.send(text)
            } else {
                b.send_empty()
            };
            convert_response(res)
        }
        "DELETE" => {
            let mut b = agent.delete(url_str);
            for (k, v) in headers {
                b = b.header(k.as_str(), v.as_str());
            }
            convert_response(b.call())
        }
        "HEAD" => {
            let mut b = agent.head(url_str);
            for (k, v) in headers {
                b = b.header(k.as_str(), v.as_str());
            }
            convert_response(b.call())
        }
        _ => {
            let mut b = agent.get(url_str);
            for (k, v) in headers {
                b = b.header(k.as_str(), v.as_str());
            }
            convert_response(b.call())
        }
    }
}

use std::net::ToSocketAddrs;

/// Checks if an IP address is private (RFC 1918), loopback, link-local (cloud metadata), or unspecified
pub fn is_prohibited_ip(ip: &std::net::IpAddr) -> bool {
    match ip {
        std::net::IpAddr::V4(ipv4) => {
            ipv4.is_loopback()
                || ipv4.is_private()
                || ipv4.is_link_local()
                || ipv4.is_broadcast()
                || ipv4.is_unspecified()
                || ipv4.octets() == [169, 254, 169, 254]
        }
        std::net::IpAddr::V6(ipv6) => {
            ipv6.is_loopback()
                || ipv6.is_unspecified()
                || (ipv6.segments()[0] & 0xfe00) == 0xfc00
                || (ipv6.segments()[0] & 0xffc0) == 0xfe80
        }
    }
}

/// Checks if a host/IP is private (RFC 1918), loopback, link-local (cloud metadata), or unspecified,
/// including pre-connection DNS resolution checks to mitigate DNS rebinding SSRF.
pub fn is_private_or_metadata_host(host: &str) -> bool {
    if host.eq_ignore_ascii_case("localhost") {
        return true;
    }
    if let Ok(ip) = host.parse::<std::net::IpAddr>() {
        return is_prohibited_ip(&ip);
    }
    let check_target = format!("{}:80", host);
    if let Ok(addrs) = check_target.to_socket_addrs() {
        for addr in addrs {
            if is_prohibited_ip(&addr.ip()) {
                return true;
            }
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_domain() {
        assert_eq!(extract_domain("https://api.github.com/repos").unwrap(), "api.github.com");
        assert_eq!(extract_domain("http://localhost:8080/test").unwrap(), "localhost");
        assert_eq!(extract_domain("https://user:pass@api.github.com:443/repos").unwrap(), "api.github.com");
        assert_eq!(extract_domain("https://api.github.com?query=val").unwrap(), "api.github.com");
        assert_eq!(extract_domain("https://api.github.com#frag").unwrap(), "api.github.com");
        assert_eq!(extract_domain("https://api.github.com.").unwrap(), "api.github.com");
        assert_eq!(extract_domain("http://[::1]:8080/test").unwrap(), "::1");
        assert!(extract_domain("ftp://ftp.example.com").is_err());
        assert!(extract_domain("https://").is_err());
    }

    #[test]
    fn test_is_domain_allowed() {
        let allowed = vec!["atlassian.net".to_string(), "api.github.com".to_string(), "example.org.".to_string()];
        assert!(is_domain_allowed("api.github.com", &allowed));
        assert!(is_domain_allowed("company.atlassian.net", &allowed));
        assert!(is_domain_allowed("example.org", &allowed));
        assert!(is_domain_allowed("sub.example.org", &allowed));
        assert!(!is_domain_allowed("evil.com", &allowed));
        assert!(!is_domain_allowed("notatlassian.net", &allowed));
    }

    #[test]
    fn test_ssrf_redirect_and_metadata_ip_blocked() {
        let caps = vec!["*".to_string()];
        let res = execute_http_request("GET", "http://169.254.169.254/latest/meta-data", &[], None, &caps, 1000);
        assert!(matches!(res, Err(AslError::CapabilityViolation(_))));

        let res_loopback = execute_http_request("GET", "http://127.0.0.1:8080/admin", &[], None, &caps, 1000);
        assert!(matches!(res_loopback, Err(AslError::CapabilityViolation(_))));

        let res_localhost = execute_http_request("GET", "http://localhost:8080/admin", &[], None, &caps, 1000);
        assert!(matches!(res_localhost, Err(AslError::CapabilityViolation(_))));
    }
}
