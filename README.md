# OGuard

> Work in progress 

**OGuard** is an open source alternative to the [NetGuard](https://powershield.com.au/support-menu/download-area/netguard-software-downloads/) software used by Dynamix UPSs. This is only intended to support 
the "Dynamix UPSD2000 Defender" as this is the only model I own so I won't be implementing the protocols 
for other UPS devices

So far I've implemented the basic USB HID protocol and can pull the battery and device state information from the device. There is a "Watcher" which polls for the devices various states every 3 seconds
and tracks the changes in those states (AC -> AC Lost) and at the moment it reports them with a desktop notification.