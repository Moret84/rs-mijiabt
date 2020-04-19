// This code was autogenerated with `dbus-codegen-rust -s -g -m None -d org.bluez -p /org/bluez/hci0`, see https://github.com/diwic/dbus-rs
use dbus as dbus;
use dbus::arg;
use dbus::blocking;

/// ADAPTER
pub trait OrgBluezAdapter1 {
    fn start_discovery(&self) -> Result<(), dbus::Error>;
    fn set_discovery_filter(&self, properties: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(), dbus::Error>;
    fn stop_discovery(&self) -> Result<(), dbus::Error>;
    fn remove_device(&self, device: dbus::Path) -> Result<(), dbus::Error>;
    fn get_discovery_filters(&self) -> Result<Vec<String>, dbus::Error>;
    fn address(&self) -> Result<String, dbus::Error>;
    fn address_type(&self) -> Result<String, dbus::Error>;
    fn name(&self) -> Result<String, dbus::Error>;
    fn alias(&self) -> Result<String, dbus::Error>;
    fn set_alias(&self, value: String) -> Result<(), dbus::Error>;
    fn class(&self) -> Result<u32, dbus::Error>;
    fn powered(&self) -> Result<bool, dbus::Error>;
    fn set_powered(&self, value: bool) -> Result<(), dbus::Error>;
    fn discoverable(&self) -> Result<bool, dbus::Error>;
    fn set_discoverable(&self, value: bool) -> Result<(), dbus::Error>;
    fn discoverable_timeout(&self) -> Result<u32, dbus::Error>;
    fn set_discoverable_timeout(&self, value: u32) -> Result<(), dbus::Error>;
    fn pairable(&self) -> Result<bool, dbus::Error>;
    fn set_pairable(&self, value: bool) -> Result<(), dbus::Error>;
    fn pairable_timeout(&self) -> Result<u32, dbus::Error>;
    fn set_pairable_timeout(&self, value: u32) -> Result<(), dbus::Error>;
    fn discovering(&self) -> Result<bool, dbus::Error>;
    fn uuids(&self) -> Result<Vec<String>, dbus::Error>;
    fn modalias(&self) -> Result<String, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> OrgBluezAdapter1 for blocking::Proxy<'a, C> {

    fn start_discovery(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Adapter1", "StartDiscovery", ())
    }

    fn set_discovery_filter(&self, properties: ::std::collections::HashMap<&str, arg::Variant<Box<dyn arg::RefArg>>>) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Adapter1", "SetDiscoveryFilter", (properties, ))
    }

    fn stop_discovery(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Adapter1", "StopDiscovery", ())
    }

    fn remove_device(&self, device: dbus::Path) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Adapter1", "RemoveDevice", (device, ))
    }

    fn get_discovery_filters(&self) -> Result<Vec<String>, dbus::Error> {
        self.method_call("org.bluez.Adapter1", "GetDiscoveryFilters", ())
            .and_then(|r: (Vec<String>, )| Ok(r.0, ))
    }

    fn address(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Address")
    }

    fn address_type(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "AddressType")
    }

    fn name(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Name")
    }

    fn alias(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Alias")
    }

    fn class(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Class")
    }

    fn powered(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Powered")
    }

    fn discoverable(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Discoverable")
    }

    fn discoverable_timeout(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "DiscoverableTimeout")
    }

    fn pairable(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Pairable")
    }

    fn pairable_timeout(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "PairableTimeout")
    }

    fn discovering(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Discovering")
    }

    fn uuids(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "UUIDs")
    }

    fn modalias(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Adapter1", "Modalias")
    }

    fn set_alias(&self, value: String) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "Alias", value)
    }

    fn set_powered(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "Powered", value)
    }

    fn set_discoverable(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "Discoverable", value)
    }

    fn set_discoverable_timeout(&self, value: u32) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "DiscoverableTimeout", value)
    }

    fn set_pairable(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "Pairable", value)
    }

    fn set_pairable_timeout(&self, value: u32) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Adapter1", "PairableTimeout", value)
    }
}

/// DEVICE
pub trait OrgBluezDevice1 {
    fn disconnect(&self) -> Result<(), dbus::Error>;
    fn connect(&self) -> Result<(), dbus::Error>;
    fn connect_profile(&self, uuid: &str) -> Result<(), dbus::Error>;
    fn disconnect_profile(&self, uuid: &str) -> Result<(), dbus::Error>;
    fn pair(&self) -> Result<(), dbus::Error>;
    fn cancel_pairing(&self) -> Result<(), dbus::Error>;
    fn address(&self) -> Result<String, dbus::Error>;
    fn address_type(&self) -> Result<String, dbus::Error>;
    fn name(&self) -> Result<String, dbus::Error>;
    fn alias(&self) -> Result<String, dbus::Error>;
    fn set_alias(&self, value: String) -> Result<(), dbus::Error>;
    fn class(&self) -> Result<u32, dbus::Error>;
    fn appearance(&self) -> Result<u16, dbus::Error>;
    fn icon(&self) -> Result<String, dbus::Error>;
    fn paired(&self) -> Result<bool, dbus::Error>;
    fn trusted(&self) -> Result<bool, dbus::Error>;
    fn set_trusted(&self, value: bool) -> Result<(), dbus::Error>;
    fn blocked(&self) -> Result<bool, dbus::Error>;
    fn set_blocked(&self, value: bool) -> Result<(), dbus::Error>;
    fn legacy_pairing(&self) -> Result<bool, dbus::Error>;
    fn rssi(&self) -> Result<i16, dbus::Error>;
    fn connected(&self) -> Result<bool, dbus::Error>;
    fn uuids(&self) -> Result<Vec<String>, dbus::Error>;
    fn modalias(&self) -> Result<String, dbus::Error>;
    fn adapter(&self) -> Result<dbus::Path<'static>, dbus::Error>;
    fn manufacturer_data(&self) -> Result<::std::collections::HashMap<u16, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error>;
    fn service_data(&self) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error>;
    fn tx_power(&self) -> Result<i16, dbus::Error>;
    fn services_resolved(&self) -> Result<bool, dbus::Error>;
}

impl<'a, C: ::std::ops::Deref<Target=blocking::Connection>> OrgBluezDevice1 for blocking::Proxy<'a, C> {

    fn disconnect(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Device1", "Disconnect", ())
    }

    fn connect(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Device1", "Connect", ())
    }

    fn connect_profile(&self, uuid: &str) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Device1", "ConnectProfile", (uuid, ))
    }

    fn disconnect_profile(&self, uuid: &str) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Device1", "DisconnectProfile", (uuid, ))
    }

    fn pair(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Device1", "Pair", ())
    }

    fn cancel_pairing(&self) -> Result<(), dbus::Error> {
        self.method_call("org.bluez.Device1", "CancelPairing", ())
    }

    fn address(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Address")
    }

    fn address_type(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "AddressType")
    }

    fn name(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Name")
    }

    fn alias(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Alias")
    }

    fn class(&self) -> Result<u32, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Class")
    }

    fn appearance(&self) -> Result<u16, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Appearance")
    }

    fn icon(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Icon")
    }

    fn paired(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Paired")
    }

    fn trusted(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Trusted")
    }

    fn blocked(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Blocked")
    }

    fn legacy_pairing(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "LegacyPairing")
    }

    fn rssi(&self) -> Result<i16, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "RSSI")
    }

    fn connected(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Connected")
    }

    fn uuids(&self) -> Result<Vec<String>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "UUIDs")
    }

    fn modalias(&self) -> Result<String, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Modalias")
    }

    fn adapter(&self) -> Result<dbus::Path<'static>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "Adapter")
    }

    fn manufacturer_data(&self) -> Result<::std::collections::HashMap<u16, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "ManufacturerData")
    }

    fn service_data(&self) -> Result<::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "ServiceData")
    }

    fn tx_power(&self) -> Result<i16, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "TxPower")
    }

    fn services_resolved(&self) -> Result<bool, dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::get(&self, "org.bluez.Device1", "ServicesResolved")
    }

    fn set_alias(&self, value: String) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Device1", "Alias", value)
    }

    fn set_trusted(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Device1", "Trusted", value)
    }

    fn set_blocked(&self, value: bool) -> Result<(), dbus::Error> {
        <Self as blocking::stdintf::org_freedesktop_dbus::Properties>::set(&self, "org.bluez.Device1", "Blocked", value)
    }
}

/// OBJECT MANAGER
#[derive(Debug)]
pub struct OrgFreedesktopDBusObjectManagerInterfacesAdded {
    pub object: dbus::Path<'static>,
    pub interfaces: ::std::collections::HashMap<String, ::std::collections::HashMap<String, arg::Variant<Box<dyn arg::RefArg + 'static>>>>,
}

impl arg::AppendAll for OrgFreedesktopDBusObjectManagerInterfacesAdded {
    fn append(&self, i: &mut arg::IterAppend) {
        arg::RefArg::append(&self.object, i);
        arg::RefArg::append(&self.interfaces, i);
    }
}

impl arg::ReadAll for OrgFreedesktopDBusObjectManagerInterfacesAdded {
    fn read(i: &mut arg::Iter) -> Result<Self, arg::TypeMismatchError> {
        Ok(OrgFreedesktopDBusObjectManagerInterfacesAdded {
            object: i.read()?,
            interfaces: i.read()?,
        })
    }
}

impl dbus::message::SignalArgs for OrgFreedesktopDBusObjectManagerInterfacesAdded {
    const NAME: &'static str = "InterfacesAdded";
    const INTERFACE: &'static str = "org.freedesktop.DBus.ObjectManager";
}
