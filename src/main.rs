use regex::Regex;
use std::io;
use std::io::prelude::*;
use handlebars::Handlebars;
use std::collections::HashMap;
use tokio::io::{AsyncWriteExt};
use clap::{Command, arg};

#[tokio::main]
async fn main() {

    // parse our arguments

    let matches = Command::new("munin-node-access")
        .version("0.1.0")
        .author("Paul Jungwirth <pj@illuminatedcomputing.com>")
        .about("Pushed a munin-node.conf file to your server with access for your current IP")
        .args(&[
              arg!(<login> "the ssh destination")
        ]).get_matches();


    // get our current ip

    let valid_ip = Regex::new(r"^\d+(\.\d+){3}$").expect("valid IP regex");
    let resp = reqwest::get("https://icanhazip.com").await.expect("getting IP")
        .text().await.expect("reading http body");
    let mut ip = resp.trim().to_string();
    if !valid_ip.is_match(&ip) {
        panic!("Response is not an IP: {:?}", resp);
    }
    ip = ip.replace(".", "\\.");
    ip = format!("^{}$", ip);
    // println!("{:?}", ip);

    // build a new munin-node.conf

    let mut tmpl = Handlebars::new();
    tmpl.register_template_string("munin", include_str!("templates/munin-node.conf")).expect("template parse");
    let mut inputs = HashMap::new();
    inputs.insert("ip", ip);
    let conf = tmpl.render("munin", &inputs).expect("template render");

    // ssh to the remote host

    let sess = openssh::Session::connect(matches.value_of("login").expect("ssh login"), openssh::KnownHosts::Strict).await.expect("ssh session");

    // Upload the new munin-node.conf file,
    // and if it's different then restart munin-node:

    let mut cmd = sess.command("bash")
        .arg("-c")
        .arg("tee /etc/munin/munin-node.conf.new ; diff /etc/munin/munin-node.conf.new /etc/munin/munin-node.conf || (mv /etc/munin/munin-node.conf.new /etc/munin/munin-node.conf ; systemctl restart munin-node)")
        .stdin(openssh::Stdio::piped())
        .stdout(openssh::Stdio::piped())
        .stderr(openssh::Stdio::piped())
        .spawn().await.expect("ssh tee");
    cmd.stdin().take().expect("stdin").write_all(conf.as_bytes()).await.expect("ssh write");
    let cmd = cmd.wait_with_output().await.expect("ssh");
    if !cmd.status.success() {
        io::stdout().write_all(&cmd.stdout).unwrap();
        io::stderr().write_all(&cmd.stderr).unwrap();
        panic!("writing /etc/munin/munin-node.conf failed!");
    }
}
