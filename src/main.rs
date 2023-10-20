use regex::Regex;
use std::io;
use std::fs;
use std::io::prelude::*;
use handlebars::Handlebars;
use std::collections::HashMap;
use tokio::io::{AsyncWriteExt};
use clap::{Command, arg};

fn generate_munin_file(ip: &str) -> String {
    let mut tmpl = Handlebars::new();
    tmpl.register_template_string("munin", include_str!("templates/munin-node.conf")).expect("template parse");
    let mut inputs = HashMap::new();
    inputs.insert("ip", ip);
    let conf = tmpl.render("munin", &inputs).expect("template render");
    conf
}

#[tokio::main]
async fn main() {

    // parse our arguments

    let matches = Command::new("munin-node-access")
        .version("0.1.0")
        .author("Paul Jungwirth <pj@illuminatedcomputing.com>")
        .about("Injects an IP into your munin-node.conf file")
        .args(&[
              arg!(<ip> "the ip address")
        ]).get_matches();


    // get our current ip

    let mut ip = matches.value_of("ip").expect("ip address").trim().to_string();
    let valid_ip = Regex::new(r"^\d+(\.\d+){3}$").expect("valid IP regex");
    if !valid_ip.is_match(&ip) {
        panic!("Argument is not an IP: {:?}", ip);
    }
    // Add escaping because munin treats it as a regex:
    ip = ip.replace(".", "\\.");
    ip = format!("^{}$", ip);
    // println!("{:?}", ip);

    // build a new munin-node.conf

    let conf = generate_munin_file(&ip);
    // println!("{}", conf);

    // Refuse to parameterize this
    // since we want to make it safe for passwordless sudo!:
    fs::write("/etc/munin/munin-node.conf", conf).expect("writing to file");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_munin_file() {
        let conf = generate_munin_file("^1\\.2\\.3\\.4$");
        assert!(conf.contains("^1\\.2\\.3\\.4$"));
        assert!(conf.contains("Example config-file for munin-node"));
    }
}
