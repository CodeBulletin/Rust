use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use url::Url;

#[derive(Debug, PartialEq)]
pub struct Config {
    pub address: Url,
    pub ipv4: bool,
    pub ipv6: bool
}

impl Config {
    pub fn new(args: &Vec<String>) -> Result<Self, String> {

        let mut address: Url = Url::parse("https://localhost").unwrap();
        let mut ipv4 = true;
        let mut ipv6 = false;

        if args.len() <= 1 {
            return Err(String::from("Please Enter the URL"));
        }

        else if args.len() >= 2 {
            let parsed_url = check_url_validator(&args[1]);
            if parsed_url.is_err() {
                return Err(format!("{:?}", parsed_url.err().unwrap()));
            }
            address = parsed_url.unwrap();

            if args.len() >= 3 {
                ipv4 = false;
                for i in &args[2..] {
                    match i.as_str() {
                        "--ipall" => {
                            ipv6 = true;
                            ipv4 = true;
                        }
                        "--ipv4" => ipv4 = true,
                        "--ipv6" => ipv6 = true,
                        _ => return Err(String::from(format!("Incorrect Input {}", i)))
                    }
                }
            }
        }

        else if args.len() >= 5 {
            return Err(String::from("Incorrect Inputs"));
        }

        Ok(Config { address, ipv4, ipv6 })
    }
}

pub fn check_url_validator(url: &String) -> Result<Url, String> {
    let parsed_url = Url::parse(url);
    if parsed_url.is_err() {
        return Err(
            format!("Error Parsing the url: {}, err: {}", url, parsed_url.err().unwrap())
        );
    }
    Ok(parsed_url.unwrap())
}

pub fn run(config: &Config) -> Result<(), String> {
    let hostname = config.address.host();
    if let Option::None = hostname {
        return Err(format!("Hostname not provided"));
    }
    let hostname = hostname.unwrap().to_string();

    let ips = dns_lookup::lookup_host(&hostname);
    if let Result::Err(err) = ips {
        return Err(format!("{}", err));
    }
    let ips = ips.unwrap();

    let mut ip4: Vec<Ipv4Addr> = vec![];
    let mut ip6: Vec<Ipv6Addr> = vec![];

    for i in ips {
        match i {
            IpAddr::V4(ip) => ip4.push(ip),
            IpAddr::V6(ip) => ip6.push(ip),
        }
    }

    if config.ipv4 {
        for i in ip4 {
            println!("{}", i);
        }
    }

    if config.ipv6 {
        for i in ip6 {
            println!("{}", i);
        }
    }

    Ok(())
}

#[cfg(test)]
mod config_tests {
    use crate::{Config, check_url_validator};

    fn config_genrator(args: &Vec<String>) -> Config {
        Config::new(args).unwrap_or_else(|err| {
            panic!("failed for args: {:?}, Error: {}", args, err);
        })
    }

    #[test]
    #[should_panic]
    fn no_args() {
        let args = vec![String::from("00")];
        config_genrator(&args);
    }

    #[test]
    fn multiple_arg() {
        let args0 = vec![String::from("00"), String::from("https://www.google.com")];
        assert_eq!(config_genrator(&args0), Config { address: check_url_validator(&args0[1]).unwrap(), ipv4: true, ipv6: false });

        let args1 = vec![String::from("00"), String::from("https://www.google.com"), String::from("--ipv4")];
        assert_eq!(config_genrator(&args1), Config { address: check_url_validator(&args1[1]).unwrap(), ipv4: true, ipv6: false });

        let args2 = vec![String::from("00"), String::from("https://www.google.com"), String::from("--ipv6")];
        assert_eq!(config_genrator(&args2), Config { address: check_url_validator(&args1[1]).unwrap(), ipv4: false, ipv6: true });

        let args3 = vec![String::from("00"), String::from("https://www.google.com"), String::from("--ipv4"), String::from("--ipv6")];
        assert_eq!(config_genrator(&args3), Config { address: check_url_validator(&args1[1]).unwrap(), ipv4: true, ipv6: true });

        let args4 = vec![String::from("00"), String::from("https://www.google.com"), String::from("--ipv6"), String::from("--ipv4")];
        assert_eq!(config_genrator(&args4), Config { address: check_url_validator(&args1[1]).unwrap(), ipv4: true, ipv6: true });
    }

    #[test]
    #[should_panic]
    fn wrong_arg() {
        let args0 = vec![String::from("00"), String::from("https://www.google.com"), String::from("--ip4")];
        config_genrator(&args0);
    }

    #[test]
    #[should_panic]
    fn wrong_order() {
        let args0 = vec![String::from("00"), String::from("--ipv4"), String::from("www.google.com")];
        config_genrator(&args0);
    }

    #[test]
    fn url_test() {
        let url1 = String::from("https://www.google.com");
        let res1 = check_url_validator(&url1);

        if res1.is_err() {
            panic!("check_url_validator not working says {} is incorrect, err: {}", url1, res1.err().unwrap())
        }
    }
}