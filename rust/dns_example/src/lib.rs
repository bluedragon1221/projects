use std::{collections::HashMap, net::Ipv4Addr};

#[derive(Debug)]
pub enum LookupError {
    NotFound,
}

#[derive(Debug)]
pub struct Dns<'a> {
    // website, IP Address
    pub lookup_table: HashMap<String, Ipv4Addr>,
    pub contacts: Vec<&'a Dns<'a>>,
}

pub struct DnsBuilder<'a> {
    // website, IP Address
    pub lookup_table: HashMap<String, Ipv4Addr>,
    pub contacts: Vec<&'a Dns<'a>>,
}

impl<'a> Dns<'a> {
    pub fn new() -> DnsBuilder<'a> {
        DnsBuilder {
            lookup_table: HashMap::new(),
            contacts: Vec::new(),
        }
    }

    pub fn lookup(&self, website: &str) -> Result<Ipv4Addr, LookupError> {
        match self.lookup_table.get(website) {
            Some(ip) => Ok(ip.clone()),
            None => self
                .contacts
                .iter()
                .find_map(|dns| dns.lookup(website).ok())
                .ok_or(LookupError::NotFound),
        }
    }
}

impl<'a> DnsBuilder<'a> {
    pub fn contact(&mut self, contact: &'a Dns<'a>) -> &mut Self {
        self.contacts.push(contact);
        self
    }

    pub fn website(&mut self, website: &str, ip: &str) -> &mut Self {
        self.lookup_table
            .insert(String::from(website), ip.parse::<Ipv4Addr>().unwrap());
        self
    }

    pub fn build(&self) -> Dns<'a> {
        Dns {
            lookup_table: self.lookup_table.clone(),
            contacts: self.contacts.clone(),
        }
    }
}
