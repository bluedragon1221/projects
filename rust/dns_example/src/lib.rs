use std::{collections::HashMap, net::Ipv4Addr};

#[derive(Debug)]
pub enum Error {
    LookupError,
    ParseIpError,
}

#[derive(Debug)]
pub struct LookupResult {
    website: String,
    ip: Ipv4Addr,
    hops: u8,
}

impl LookupResult {
    pub fn get_website(&self) -> String {
        self.website.clone()
    }

    pub fn get_ip(&self) -> Ipv4Addr {
        self.ip
    }

    pub fn get_hops(&self) -> u8 {
        self.hops
    }
}

#[derive(Debug)]
pub struct Dns<'a> {
    lookup_table: HashMap<String, Ipv4Addr>,
    contacts: Vec<&'a Dns<'a>>,
}

pub struct DnsBuilder<'a> {
    lookup_table: HashMap<String, Ipv4Addr>,
    contacts: Vec<&'a Dns<'a>>,
}

impl<'a> Dns<'a> {
    pub fn new() -> DnsBuilder<'a> {
        DnsBuilder {
            lookup_table: HashMap::new(),
            contacts: Vec::new(),
        }
    }

    fn lookup_internal(&self, website: &str, hops: u8) -> Result<LookupResult, Error> {
        match self.lookup_table.get(website) {
            Some(ip) => Ok(LookupResult {
                website: website.to_owned(),
                ip: ip.clone(),
                hops,
            }),
            None => {
                for dns in self.contacts.iter() {
                    match dns.lookup_internal(website, hops + 1) {
                        Ok(result) => return Ok(result),
                        Err(_) => continue,
                    }
                }
                Err(Error::LookupError)
            }
        }
    }

    pub fn lookup(&self, website: &str) -> Result<LookupResult, Error> {
        self.lookup_internal(website, 0)
    }
}

impl<'a> DnsBuilder<'a> {
    pub fn contact(&mut self, contact: &'a Dns<'a>) -> &mut Self {
        self.contacts.push(contact);
        self
    }

    pub fn website(&mut self, website: &str, ip: Ipv4Addr) -> &mut Self {
        self.lookup_table.insert(String::from(website), ip);
        self
    }

    pub fn lookup_table(&mut self, lookup_table: HashMap<&str, Ipv4Addr>) -> &mut Self {
        for (website, ip) in lookup_table {
            self.website(website, ip);
        }
        self
    }

    pub fn build(&self) -> Dns<'a> {
        Dns {
            lookup_table: self.lookup_table.clone(),
            contacts: self.contacts.clone(),
        }
    }
}
