//! H3C vendor specific functions
//!
//! Reference:
//! https://networkgeekstuff.com/networking/hp-networking-comware-netconf-interface-quick-tutorial-using-pythons-ncclient-and-pyhpecw7/
//! https://github.com/HPENetworking/pyhpecw7
//! http://www.h3c.com/cn/d_201412/847691_30005_0.htm#_Toc404878998
//! https://github.com/hongfeioo/H3C_netconf_lib/blob/master/doc/Comware%20V7%20StaticRoute%20NETCONF%20XML%20API%20Configuration%20Reference.docx

mod get_vlan_config;
mod interface;
mod l2;
mod netconf;
mod reply;
mod vlan;

pub use get_vlan_config::*;
pub use interface::*;
pub use l2::*;
pub use netconf::*;
pub use reply::*;
pub use vlan::*;
