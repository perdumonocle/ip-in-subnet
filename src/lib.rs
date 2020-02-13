//! # ip-in-subnet
//!
//! Checking that subnet contains an IP address.
//!
//! ## Usage
//! 
//! To use `sql-builder`, first add this to your `Cargo.toml`:
//! 
//! ```toml
//! [dependencies]
//! sql-builder = "0.1"
//! ```
//! 
//! # Examples
//!
//! ## Check that subnet contains an interface.
//!
//! ```
//! extern crate ip_in_subnet;
//!
//! use ip_in_subnet::iface_in_subnet;
//!
//! let res = iface_in_subnet("192.168.182.1", "192.168.182.0/24").unwrap();
//! assert!(res);
//! ```
//!
//! ```
//! extern crate ip_in_subnet;
//!
//! use ip_in_subnet::iface_in_subnet;
//!
//! let res = iface_in_subnet("192.168.183.1", "192.168.182.0/24").unwrap();
//! assert!(!res);
//! ```
//!
//! ## Check that any subnet contains an interface.
//!
//! ```
//! extern crate ip_in_subnet;
//!
//! use ip_in_subnet::iface_in_any_subnet;
//!
//! let subnets = vec!["192.168.181.0/24", "192.168.182.0/24"];
//! let res = iface_in_any_subnet("192.168.182.1", &subnets).unwrap();
//! assert!(res);
//! ```
//!
//! ```
//! extern crate ip_in_subnet;
//!
//! use ip_in_subnet::iface_in_any_subnet;
//!
//! let subnets = vec!["192.168.181.0/24", "192.168.182.0/24"];
//! let res = iface_in_any_subnet("192.168.183.1", &subnets).unwrap();
//! assert!(!res);
//! ```
//!
//! ## Check that all subnets contains an interface.
//!
//! ```
//! extern crate ip_in_subnet;
//!
//! use ip_in_subnet::iface_in_all_subnets;
//!
//! let subnets = vec!["192.168.182.0/24", "192.168.182.1/32"];
//! let res = iface_in_all_subnets("192.168.182.1", &subnets).unwrap();
//! assert!(res);
//! ```
//!
//! ```
//! extern crate ip_in_subnet;
//!
//! use ip_in_subnet::iface_in_all_subnets;
//!
//! let subnets = vec!["192.168.182.0/24", "192.168.182.2/32"];
//! let res = iface_in_all_subnets("192.168.182.1", &subnets).unwrap();
//! assert!(!res);
//! ```
//!
//! ## Check that any subnet contains any interface.
//!
//! ```
//! extern crate ip_in_subnet;
//!
//! use ip_in_subnet::any_iface_in_any_subnet;
//!
//! let ifaces = vec!["192.168.182.1", "192.168.182.2"];
//! let subnets = vec!["192.168.181.0/24", "192.168.182.2/32"];
//! let res = any_iface_in_any_subnet(&ifaces, &subnets).unwrap();
//! assert!(res);
//! ```
//!
//! ```
//! extern crate ip_in_subnet;
//!
//! use ip_in_subnet::any_iface_in_any_subnet;
//!
//! let ifaces = vec!["192.168.182.1", "192.168.182.2"];
//! let subnets = vec!["192.168.181.0/24", "192.168.182.3/32"];
//! let res = any_iface_in_any_subnet(&ifaces, &subnets).unwrap();
//! assert!(!res);
//! ```

use netaddr2::{Contains, Error as NetError, NetAddr};
use std::error::Error;
use std::net::{Ipv4Addr, Ipv6Addr};

/// Check that subnet contains an interface.
///
/// # Examples
///
/// ```
/// extern crate ip_in_subnet;
///
/// use ip_in_subnet::iface_in_subnet;
///
/// let res = iface_in_subnet("192.168.182.1", "192.168.182.0/24").unwrap();
/// assert!(res);
/// ```
///
/// ```
/// extern crate ip_in_subnet;
///
/// use ip_in_subnet::iface_in_subnet;
///
/// let res = iface_in_subnet("192.168.183.1", "192.168.182.0/24").unwrap();
/// assert!(!res);
/// ```
pub fn iface_in_subnet(iface: &str, subnet: &str) -> Result<bool, Box<dyn Error>> {
    match subnet.parse::<NetAddr>() {
        Ok(NetAddr::V4(subnet4)) => {
            if let Ok(iface) = iface.parse::<Ipv4Addr>() {
                let is_in = subnet4.contains(&iface);
                Ok(is_in)
            } else {
                Ok(false)
            }
        }
        Ok(NetAddr::V6(subnet6)) => {
            if let Ok(iface) = iface.parse::<Ipv6Addr>() {
                let is_in = subnet6.contains(&iface);
                Ok(is_in)
            } else {
                Ok(false)
            }
        }
        Err(NetError::ParseError(e)) => Err(e.into()),
    }
}

/// Check that any subnet contains an interface.
///
/// # Examples
///
/// ```
/// extern crate ip_in_subnet;
///
/// use ip_in_subnet::iface_in_any_subnet;
///
/// let subnets = vec!["192.168.181.0/24", "192.168.182.0/24"];
/// let res = iface_in_any_subnet("192.168.182.1", &subnets).unwrap();
/// assert!(res);
/// ```
///
/// ```
/// extern crate ip_in_subnet;
///
/// use ip_in_subnet::iface_in_any_subnet;
///
/// let subnets = vec!["192.168.181.0/24", "192.168.182.0/24"];
/// let res = iface_in_any_subnet("192.168.183.1", &subnets).unwrap();
/// assert!(!res);
/// ```
pub fn iface_in_any_subnet(iface: &str, subnets: &[&str]) -> Result<bool, Box<dyn Error>> {
    let mut iface4 = None;
    let mut iface6 = None;
    for subnet in subnets.iter() {
        match subnet.parse::<NetAddr>() {
            Ok(NetAddr::V4(subnet4)) => {
                if iface4.is_none() {
                    if let Ok(iface) = iface.parse::<Ipv4Addr>() {
                        iface4 = Some(iface);
                    } else {
                        continue;
                    }
                }
                if subnet4.contains(&iface4.unwrap()) {
                    return Ok(true);
                }
            }
            Ok(NetAddr::V6(subnet6)) => {
                if iface6.is_none() {
                    if let Ok(iface) = iface.parse::<Ipv6Addr>() {
                        iface6 = Some(iface);
                    } else {
                        continue;
                    }
                }
                if subnet6.contains(&iface6.unwrap()) {
                    return Ok(true);
                }
            }
            Err(NetError::ParseError(e)) => return Err(e.into()),
        }
    }
    Ok(false)
}

/// Check that all subnets contains an interface.
///
/// # Examples
///
/// ```
/// extern crate ip_in_subnet;
///
/// use ip_in_subnet::iface_in_all_subnets;
///
/// let subnets = vec!["192.168.182.0/24", "192.168.182.1/32"];
/// let res = iface_in_all_subnets("192.168.182.1", &subnets).unwrap();
/// assert!(res);
/// ```
///
/// ```
/// extern crate ip_in_subnet;
///
/// use ip_in_subnet::iface_in_all_subnets;
///
/// let subnets = vec!["192.168.182.0/24", "192.168.182.2/32"];
/// let res = iface_in_all_subnets("192.168.182.1", &subnets).unwrap();
/// assert!(!res);
/// ```
pub fn iface_in_all_subnets(iface: &str, subnets: &[&str]) -> Result<bool, Box<dyn Error>> {
    for subnet in subnets.iter() {
        let res = iface_in_subnet(&iface, &subnet);
        if let Ok(false) = res {
            return Ok(false);
        }
    }
    Ok(true)
}

/// Check that any subnet contains any interface.
///
/// # Examples
///
/// ```
/// extern crate ip_in_subnet;
///
/// use ip_in_subnet::any_iface_in_any_subnet;
///
/// let ifaces = vec!["192.168.182.1", "192.168.182.2"];
/// let subnets = vec!["192.168.181.0/24", "192.168.182.2/32"];
/// let res = any_iface_in_any_subnet(&ifaces, &subnets).unwrap();
/// assert!(res);
/// ```
///
/// ```
/// extern crate ip_in_subnet;
///
/// use ip_in_subnet::any_iface_in_any_subnet;
///
/// let ifaces = vec!["192.168.182.1", "192.168.182.2"];
/// let subnets = vec!["192.168.181.0/24", "192.168.182.3/32"];
/// let res = any_iface_in_any_subnet(&ifaces, &subnets).unwrap();
/// assert!(!res);
/// ```
pub fn any_iface_in_any_subnet(ifaces: &[&str], subnets: &[&str]) -> Result<bool, Box<dyn Error>> {
    for subnet in subnets.iter() {
        match subnet.parse::<NetAddr>() {
            Ok(NetAddr::V4(subnet4)) => {
                for iface in ifaces.iter() {
                    if let Ok(iface) = iface.parse::<Ipv4Addr>() {
                        if subnet4.contains(&iface) {
                            return Ok(true)
                        }
                    }
                }
            }
            Ok(NetAddr::V6(subnet6)) => {
                for iface in ifaces.iter() {
                    if let Ok(iface) = iface.parse::<Ipv6Addr>() {
                        if subnet6.contains(&iface) {
                            return Ok(true)
                        }
                    }
                }
            }
            Err(NetError::ParseError(e)) => return Err(e.into()),
        }
    }
    Ok(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_iface_in_subnet() {
        let res = iface_in_subnet("192.168.182.1", "192.168.182.0/24").unwrap();
        assert!(res);
    }

    #[test]
    fn test_iface_not_in_subnet() {
        let res = iface_in_subnet("192.168.183.1", "192.168.182.0/24").unwrap();
        assert!(!res);
    }

    #[test]
    fn test_iface_in_any_subnet() {
        let subnets = vec!["192.168.181.0/24", "192.168.182.0/24"];
        let res = iface_in_any_subnet("192.168.182.1", &subnets).unwrap();
        assert!(res);
    }

    #[test]
    fn test_iface_not_in_any_subnet() {
        let subnets = vec!["192.168.181.0/24", "192.168.182.0/24"];
        let res = iface_in_any_subnet("192.168.183.1", &subnets).unwrap();
        assert!(!res);
    }

    #[test]
    fn test_iface_in_all_subnets() {
        let subnets = vec!["192.168.182.0/24", "192.168.182.1/32"];
        let res = iface_in_all_subnets("192.168.182.1", &subnets).unwrap();
        assert!(res);
    }

    #[test]
    fn test_iface_not_in_all_subnets() {
        let subnets = vec!["192.168.182.0/24", "192.168.182.2/32"];
        let res = iface_in_all_subnets("192.168.182.1", &subnets).unwrap();
        assert!(!res);
    }

    #[test]
    fn test_any_iface_in_any_subnet() {
        let ifaces = vec!["192.168.182.1", "192.168.182.2"];
        let subnets = vec!["192.168.181.0/24", "192.168.182.2/32"];
        let res = any_iface_in_any_subnet(&ifaces, &subnets).unwrap();
        assert!(res);
    }

    #[test]
    fn test_any_iface_not_in_any_subnet() {
        let ifaces = vec!["192.168.182.1", "192.168.182.2"];
        let subnets = vec!["192.168.181.0/24", "192.168.182.3/32"];
        let res = any_iface_in_any_subnet(&ifaces, &subnets).unwrap();
        assert!(!res);
    }
}
