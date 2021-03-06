pub mod adapter;
mod constants;
pub mod manager;
mod protocol;
mod util;

mod ioctl {
    use super::adapter;
    use super::manager;

    // #define HCIDEVUP	_IOW('H', 201, int)
    ioctl_write_int!(hci_dev_up, b'H', 201);
    // #define HCIDEVDOWN	_IOW('H', 202, int)
    ioctl_write_int!(hci_dev_down, b'H', 202);

    // #define HCIGETDEVLIST _IOR('H', 210, int)
    const HCI_GET_DEV_LIST_MAGIC: usize = (2u32 << 0i32 + 8i32 + 8i32 + 14i32
        | ((b'H' as i32) << 0i32 + 8i32) as u32
        | (210i32 << 0i32) as u32) as usize
        | 4 /* (sizeof(i32)) */ << 0i32 + 8i32 + 8i32;
    ioctl_read_bad!(
        hci_get_dev_list,
        HCI_GET_DEV_LIST_MAGIC,
        manager::HCIDevListReq
    );

    // #define HCIGETDEVINFO	_IOR('H', 211, int)
    const HCI_GET_DEV_MAGIC: usize = (2u32 << 0i32 + 8i32 + 8i32 + 14i32
        | ((b'H' as i32) << 0i32 + 8i32) as u32
        | (211i32 << 0i32) as u32) as usize
        | 4 /* (sizeof(i32)) */ << 0i32 + 8i32 + 8i32;
    ioctl_read_bad!(hci_get_dev_info, HCI_GET_DEV_MAGIC, adapter::HCIDevInfo);
}
