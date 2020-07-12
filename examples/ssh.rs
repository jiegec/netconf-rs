use log::*;
use netconf_rs;
use netconf_rs::Connection;

fn main() {
    env_logger::init();
    let mut args = std::env::args();
    args.next();
    let addr = args.next().unwrap();
    info!("connecting to {}", addr);
    let ssh = netconf_rs::transport::ssh::SSHTransport::connect(&addr, "admin", "admin").unwrap();
    let mut conn = Connection::new(ssh).unwrap();
    conn.get_config().unwrap();
    info!("connected to {}", addr);
}
