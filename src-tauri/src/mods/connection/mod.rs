#![allow(dead_code)]
pub mod types;

use std::{ffi, mem, ptr, slice};

use types::wlan::Wlan;
use windows::Win32::{
  Foundation::{ERROR_SUCCESS, HANDLE, WIN32_ERROR},
  NetworkManagement::{
    IpHelper::{
      GetAdaptersAddresses, GAA_FLAG_SKIP_ANYCAST, GAA_FLAG_SKIP_DNS_SERVER, GAA_FLAG_SKIP_MULTICAST,
      GAA_FLAG_SKIP_UNICAST, IP_ADAPTER_ADDRESSES_LH,
    },
    Ndis::IfOperStatusUp,
    WiFi::{
      dot11_radio_state_off, dot11_radio_state_on, wlan_intf_opcode_radio_state, WlanCloseHandle, WlanEnumInterfaces,
      WlanFreeMemory, WlanGetAvailableNetworkList, WlanGetNetworkBssList, WlanOpenHandle, WlanSetInterface,
      WLAN_AVAILABLE_NETWORK, WLAN_BSS_ENTRY, WLAN_INTERFACE_INFO, WLAN_PHY_RADIO_STATE,
    },
  },
};

fn open_handle() -> Option<HANDLE> {
  let mut handle = HANDLE::default();
  let mut current_version = 0;

  let open_handle_result = WIN32_ERROR(unsafe { WlanOpenHandle(2, None, &mut current_version, &mut handle) });
  if open_handle_result != ERROR_SUCCESS {
    unsafe { WlanCloseHandle(handle, None) };
    return None;
  }

  Some(handle)
}

fn enum_interfaces(handle: &HANDLE) -> Vec<WLAN_INTERFACE_INFO> {
  let mut interface_info_list = ptr::null_mut();

  unsafe {
    let enum_interfaces_result = WIN32_ERROR(WlanEnumInterfaces(*handle, None, &mut interface_info_list));
    if enum_interfaces_result != ERROR_SUCCESS {
      WlanCloseHandle(*handle, None);
      return vec![];
    }

    // https://stackoverflow.com/a/78779478/9879620
    let interface_info_ptr = ptr::addr_of!((*interface_info_list).InterfaceInfo);
    let interface_info_len = (*interface_info_list).dwNumberOfItems as usize;
    let interface_info =
      slice::from_raw_parts(interface_info_ptr.cast::<WLAN_INTERFACE_INFO>(), interface_info_len).to_vec();

    WlanFreeMemory(interface_info_list.cast());

    interface_info
  }
}

fn get_available_network_list(handle: &HANDLE, interface: &WLAN_INTERFACE_INFO) -> Vec<WLAN_AVAILABLE_NETWORK> {
  let mut available_network_list = ptr::null_mut();

  unsafe {
    let result = WIN32_ERROR(WlanGetAvailableNetworkList(
      *handle,
      &interface.InterfaceGuid,
      0,
      None,
      &mut available_network_list,
    ));
    if result != ERROR_SUCCESS {
      return vec![];
    }

    // https://stackoverflow.com/a/78779478/9879620
    let networks_ptr = ptr::addr_of!((*available_network_list).Network);
    let networks_len = (*available_network_list).dwNumberOfItems as usize;
    let networks = slice::from_raw_parts(networks_ptr.cast::<WLAN_AVAILABLE_NETWORK>(), networks_len).to_vec();

    WlanFreeMemory(available_network_list.cast());

    networks
  }
}

fn get_network_bss_list(
  handle: &HANDLE,
  interface: &WLAN_INTERFACE_INFO,
  network: &WLAN_AVAILABLE_NETWORK,
) -> Vec<WLAN_BSS_ENTRY> {
  unsafe {
    let mut bssid_list = ptr::null_mut();
    let result = WIN32_ERROR(WlanGetNetworkBssList(
      *handle,
      &interface.InterfaceGuid,
      Some(ptr::addr_of!(network.dot11Ssid)),
      network.dot11BssType,
      network.bSecurityEnabled,
      None,
      &mut bssid_list,
    ));
    if result != ERROR_SUCCESS {
      return vec![];
    }

    // https://stackoverflow.com/a/78779478/9879620
    let bss_entries_ptr = ptr::addr_of!((*bssid_list).wlanBssEntries);
    let bss_entries_len = (*bssid_list).dwNumberOfItems as usize;
    let bss_entries = slice::from_raw_parts(bss_entries_ptr.cast::<WLAN_BSS_ENTRY>(), bss_entries_len).to_vec();

    WlanFreeMemory(bssid_list.cast());

    bss_entries
  }
}

pub fn get_available_networks() -> Vec<Wlan> {
  let mut network_list: Vec<Wlan> = Vec::new();

  if let Some(handle) = open_handle() {
    for interface in enum_interfaces(&handle) {
      let available_network_list = get_available_network_list(&handle, &interface);
      if available_network_list.is_empty() {
        drop(available_network_list);
        continue;
      }

      let available_network_list = get_available_network_list(&handle, &interface);
      for network in &available_network_list {
        network_list.push(Wlan::new(network, get_network_bss_list(&handle, &interface, network)));
      }
      drop(available_network_list);
    }
    unsafe { WlanCloseHandle(handle, None) };
  }

  vec![]
}

pub fn is_ethernet_plugged_in() -> bool {
  let mut is_plugged_in = false;

  // https://docs.microsoft.com/en-us/windows/desktop/api/iphlpapi/nf-iphlpapi-getadaptersaddresses
  let mut buf_len: core::ffi::c_ulong = 16384;
  let mut adapters_addresses_buffer = vec![0; buf_len as usize];
  let result = WIN32_ERROR(unsafe {
    GetAdaptersAddresses(
      0, // AF_UNSPEC
      GAA_FLAG_SKIP_UNICAST | GAA_FLAG_SKIP_ANYCAST | GAA_FLAG_SKIP_MULTICAST | GAA_FLAG_SKIP_DNS_SERVER,
      None,
      Some(adapters_addresses_buffer.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH),
      &mut buf_len,
    )
  });

  if result == ERROR_SUCCESS {
    let mut adapter_addresses_ptr = adapters_addresses_buffer.as_mut_ptr() as *mut IP_ADAPTER_ADDRESSES_LH;

    while !adapter_addresses_ptr.is_null() {
      let adapter = unsafe { adapter_addresses_ptr.read_unaligned() };

      if adapter.IfType == 6 && unsafe { adapter.FriendlyName.to_string().unwrap() } == "Ethernet" {
        is_plugged_in = adapter.OperStatus == IfOperStatusUp;
        break;
      }
      adapter_addresses_ptr = adapter.Next;
    }

    drop(adapters_addresses_buffer);
    is_plugged_in
  } else {
    drop(adapters_addresses_buffer);
    false
  }
}

pub fn set_wifi_state(on: bool) {
  if let Some(handle) = open_handle() {
    let enum_interfaces = enum_interfaces(&handle);

    let state = WLAN_PHY_RADIO_STATE {
      dwPhyIndex: 0,
      dot11SoftwareRadioState: if on {
        dot11_radio_state_on
      } else {
        dot11_radio_state_off
      },
      ..Default::default()
    };

    for interface in enum_interfaces {
      unsafe {
        WlanSetInterface(
          handle,
          ptr::addr_of!(interface.InterfaceGuid),
          wlan_intf_opcode_radio_state,
          mem::size_of::<WLAN_PHY_RADIO_STATE>() as u32,
          ptr::addr_of!(state) as *const ffi::c_void,
          None,
        );
      }
    }

    unsafe { WlanCloseHandle(handle, None) };
  }
}
