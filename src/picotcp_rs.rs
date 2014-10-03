extern crate libc;
//use libc::size_t;
use libc::c_int;
use libc::c_char;
use std::mem::to_be32;
use std::fmt;

#[packed]
pub struct pico_ip4 {
    addr: u32,
}

impl fmt::Show for pico_ip4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let addr = self.addr.to_be();
        let a = (addr >> 24) & 0xFF;
        let b = (addr >> 16) & 0xFF;
        let c = (addr >> 8) & 0xFF;
        let d = (addr) & 0xFF;

        write!(f, " {}.{}.{}.{}", a,b,c,d)
    }
}

/*
pub struct pico_device {
    char name[MAX_DEVICE_NAME];
    hash: u32;
    overhead: u32;
    struct pico_ethdev *eth; // Null if non-ethernet
    struct pico_queue *q_in;
    struct pico_queue *q_out;
    int (*send)(struct pico_device *self, void *buf, int len); // Send function. Return 0 if busy
    int (*poll)(struct pico_device *self, int loop_score);
    void (*destroy)(struct pico_device *self);
    int (*dsr)(struct pico_device *self, int loop_score);
    __serving_interrupt: u32;
    // used to signal the upper layer the number of events arrived since the last processing
    eventCnt: u32;
  #ifdef PICO_SUPPORT_IPV6
    struct pico_nd_hostvars hostvars;
  #endif
*/


#[link(name = "devtun", kind="static")]
extern {
    fn pico_tun_create(name: *const c_char) -> *mut u32;
}

#[link(name = "picotcp", kind="static")]
extern {
    //fn snappy_max_compressed_length(source_length: size_t) -> size_t;
    fn pico_stack_init() -> c_int;
    fn pico_stack_tick(); 
    fn pico_string_to_ipv4(ipstr: *const c_char, ip: *mut pico_ip4);
    fn pico_ipv4_link_add(dev: *mut u32, address: pico_ip4, netmask: pico_ip4);
}

fn main() {
    let ipaddr = "192.168.2.150";
    let ipaddr_cstr = ipaddr.to_c_str();
    let netmask = "255.255.255.0";
    let netmask_cstr = netmask.to_c_str();

    let mut my_ip_addr = pico_ip4 { addr: 0 };
    let mut my_netmask = pico_ip4 { addr: 0 };

    unsafe { pico_stack_init(); }

    let tun_name = "tun0";
    let tun_name_cstr = tun_name.to_c_str();
    let pico_dev_eth = unsafe { pico_tun_create(tun_name_cstr.as_ptr()) };
    unsafe {
        //if pico_dev_eth == Nil { return; }
    }

    unsafe {
        pico_string_to_ipv4(ipaddr_cstr.as_ptr(), &mut my_ip_addr);
        pico_string_to_ipv4(netmask_cstr.as_ptr(), &mut my_netmask);
        pico_ipv4_link_add(pico_dev_eth, my_ip_addr, my_netmask);
    }

    println!("ip addr is {}", my_ip_addr);

    loop {
        unsafe { pico_stack_tick() };
    }

}
