use log::*;
use netconf_rs;
use netconf_rs::vendor::h3c::*;
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
    get_netconf_information(&mut conn).unwrap();
    /*
    get_vlan_config(&mut conn).unwrap();

    // create vlan 10 and 11
    create_vlan(&mut conn, 10, "Test VLAN 10").unwrap();
    create_vlan(&mut conn, 11, "Test VLAN 11").unwrap();

    // assign access ports
    // port 1 access vlan 10
    set_vlan_access_port(&mut conn, 1, 10).unwrap();
    // port 2 access vlan 11
    set_vlan_access_port(&mut conn, 2, 11).unwrap();

    // assign trunk ports
    // port 9 trunk permit 1025 pvid 1025
    set_vlan_trunk_port(&mut conn, 9, &[1025], Some(1025)).unwrap();
    // port 10 trunk permit 1025 pvid 1025
    set_vlan_trunk_port(&mut conn, 10, &[1025], Some(1025)).unwrap();

    get_interfaces(&mut conn).unwrap();
    */
    info!("connected to {}", addr);
}
