#![allow(non_snake_case)]
use std::ffi::c_char;

use crate::raw::{gx_const::*, gx_enum::*, gx_struct::*};

pub struct GXDeviceBaseInfoBuilder {
    szVendorName: [u8; GX_INFO_LENGTH_32_BYTE],
    szModelName: [u8; GX_INFO_LENGTH_32_BYTE],
    szSN: [u8; GX_INFO_LENGTH_32_BYTE],
    szDisplayName: [u8; GX_INFO_LENGTH_128_BYTE],
    szDeviceID: [u8; GX_INFO_LENGTH_64_BYTE],
    szUserID: [u8; GX_INFO_LENGTH_64_BYTE],
    accessStatus: GX_ACCESS_STATUS,
    deviceClass: GX_DEVICE_CLASS,
    reserved: [u8; 300],
}

impl GXDeviceBaseInfoBuilder {
    pub fn new() -> Self {
        Self {
            szVendorName: [0; GX_INFO_LENGTH_32_BYTE],
            szModelName: [0; GX_INFO_LENGTH_32_BYTE],
            szSN: [0; GX_INFO_LENGTH_32_BYTE],
            szDisplayName: [0; GX_INFO_LENGTH_128_BYTE],
            szDeviceID: [0; GX_INFO_LENGTH_64_BYTE],
            szUserID: [0; GX_INFO_LENGTH_64_BYTE],
            accessStatus: GX_ACCESS_STATUS::GX_ACCESS_STATUS_UNKNOWN,
            deviceClass: GX_DEVICE_CLASS::GX_DEVICE_CLASS_UNKNOWN,
            reserved: [0; 300],
        }
    }

    pub fn sz_vendor_name(mut self, value: [u8; GX_INFO_LENGTH_32_BYTE]) -> Self {
        self.szVendorName = value;
        self
    }

    pub fn sz_model_name(mut self, value: [u8; GX_INFO_LENGTH_32_BYTE]) -> Self {
        self.szModelName = value;
        self
    }

    pub fn sz_sn(mut self, value: [u8; GX_INFO_LENGTH_32_BYTE]) -> Self {
        self.szSN = value;
        self
    }

    pub fn sz_display_name(mut self, value: [u8; GX_INFO_LENGTH_128_BYTE]) -> Self {
        self.szDisplayName = value;
        self
    }

    pub fn sz_device_id(mut self, value: [u8; GX_INFO_LENGTH_64_BYTE]) -> Self {
        self.szDeviceID = value;
        self
    }

    pub fn sz_user_id(mut self, value: [u8; GX_INFO_LENGTH_64_BYTE]) -> Self {
        self.szUserID = value;
        self
    }

    pub fn access_status(mut self, value: GX_ACCESS_STATUS) -> Self {
        self.accessStatus = value;
        self
    }

    pub fn device_class(mut self, value: GX_DEVICE_CLASS) -> Self {
        self.deviceClass = value;
        self
    }

    pub fn reserved(mut self, value: [u8; 300]) -> Self {
        self.reserved = value;
        self
    }

    // 构建最终的 GX_DEVICE_BASE_INFO 实例
    pub fn build(self) -> GX_DEVICE_BASE_INFO {
        GX_DEVICE_BASE_INFO {
            szVendorName: self.szVendorName,
            szModelName: self.szModelName,
            szSN: self.szSN,
            szDisplayName: self.szDisplayName,
            szDeviceID: self.szDeviceID,
            szUserID: self.szUserID,
            accessStatus: self.accessStatus,
            deviceClass: self.deviceClass,
            reserved: self.reserved,
        }
    }
}

pub struct GXOpenParamBuilder {
    pub pszContent: *const c_char,
    pub openMode: GX_OPEN_MODE,
    pub accessMode: GX_ACCESS_MODE,
}

impl GXOpenParamBuilder {
    pub fn new() -> Self {
        Self {
            pszContent: std::ptr::null(),
            openMode: GX_OPEN_MODE::GX_OPEN_SN,
            accessMode: GX_ACCESS_MODE::GX_ACCESS_CONTROL,
        }
    }

    pub fn psz_content(mut self, value: *const c_char) -> Self {
        self.pszContent = value;
        self
    }

    pub fn open_mode(mut self, value: GX_OPEN_MODE) -> Self {
        self.openMode = value;
        self
    }

    pub fn access_mode(mut self, value: GX_ACCESS_MODE) -> Self {
        self.accessMode = value;
        self
    }

    pub fn build(self) -> GX_OPEN_PARAM {
        GX_OPEN_PARAM {
            pszContent: self.pszContent,
            openMode: self.openMode,
            accessMode: self.accessMode,
        }
    }
}
