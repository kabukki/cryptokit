use trust_dns_resolver::Resolver;
use std::net::IpAddr;

use crate::ActionResult;

pub fn resolve (host: String) -> ActionResult<Vec<IpAddr>> {
    let resolver = Resolver::from_system_conf().map_err(|err| err.to_string())?;
    let response = resolver.lookup_ip(host).map_err(|err| err.to_string())?;

    Ok(response.iter().collect())
}
