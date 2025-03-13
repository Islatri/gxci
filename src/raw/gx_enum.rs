//! Enumeration definition of the GxISDK
#![allow(dead_code)]
#![allow(non_camel_case_types)]
use std::ops::BitOr;

impl BitOr for GX_FEATURE_TYPE {
    type Output = u32;

    fn bitor(self, rhs: Self) -> Self::Output {
        self as u32 | rhs as u32
    }
}

impl BitOr<GX_FEATURE_LEVEL> for GX_FEATURE_TYPE {
    type Output = u32;

    fn bitor(self, rhs: GX_FEATURE_LEVEL) -> Self::Output {
        self as u32 | rhs as u32
    }
}

impl BitOr<GX_FEATURE_TYPE> for u32 {
    type Output = u32;

    fn bitor(self, rhs: GX_FEATURE_TYPE) -> Self::Output {
        self | rhs as u32
    }
}

impl BitOr<GX_FEATURE_LEVEL> for u32 {
    type Output = u32;

    fn bitor(self, rhs: GX_FEATURE_LEVEL) -> Self::Output {
        self | rhs as u32
    }
}

//------------------------------------------------------------------------------
//  Frame Status Definition
//------------------------------------------------------------------------------
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_FRAME_STATUS {
    GX_FRAME_STATUS_SUCCESS = 0,     // Normal frame
    GX_FRAME_STATUS_INCOMPLETE = -1, // Incomplete frame
}

//------------------------------------------------------------------------------
//  Device Type Definition
//------------------------------------------------------------------------------
// 历史原因保留下面的写法
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_DEVICE_CLASS {
    GX_DEVICE_CLASS_UNKNOWN = 0, // Unknown device type
    GX_DEVICE_CLASS_USB2 = 1,    // USB2.0 Vision device
    GX_DEVICE_CLASS_GEV = 2,     // Gige Vision device
    GX_DEVICE_CLASS_U3V = 3,     // USB3 Vision device
}

//------------------------------------------------------------------------------
//  Feature Type Definition
//------------------------------------------------------------------------------
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_FEATURE_TYPE {
    GX_FEATURE_INT = 0x10000000,     // Integer type
    GX_FEATURE_FLOAT = 0x20000000,   // Floating point type
    GX_FEATURE_ENUM = 0x30000000,    // Enum type
    GX_FEATURE_BOOL = 0x40000000,    // Boolean type
    GX_FEATURE_STRING = 0x50000000,  // String type
    GX_FEATURE_BUFFER = 0x60000000,  // Block data type
    GX_FEATURE_COMMAND = 0x70000000, // Command type
}

//------------------------------------------------------------------------------
//  Feature Level Definition
//------------------------------------------------------------------------------
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_FEATURE_LEVEL {
    GX_FEATURE_LEVEL_REMOTE_DEV = 0x00000000, //< Remote device layer
    GX_FEATURE_LEVEL_TL = 0x01000000,         //< TL layer
    GX_FEATURE_LEVEL_IF = 0x02000000,         //< Interface layer
    GX_FEATURE_LEVEL_DEV = 0x03000000,        //< Device layer
    GX_FEATURE_LEVEL_DS = 0x04000000,         //< DataStream layer
}

//------------------------------------------------------------------------------
//  Access Mode of Device
//------------------------------------------------------------------------------
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_ACCESS_MODE {
    GX_ACCESS_READONLY = 2,  //< Open the device in read-only mode
    GX_ACCESS_CONTROL = 3,   //< Open the device in controlled mode
    GX_ACCESS_EXCLUSIVE = 4, //< Open the device in exclusive mode
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_FEATURE_ID {
    //////////////////////////////////////////////////////////////////////////
    /// Remote device layer(Remote Device Feature)
    //////////////////////////////////////////////////////////////////////////

    //---------------DeviceInfomation Section--------------------------
    GX_STRING_DEVICE_VENDOR_NAME =
        (0 | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Name of the manufacturer of the device.
    GX_STRING_DEVICE_MODEL_NAME =
        (1 | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Model of the device.
    GX_STRING_DEVICE_FIRMWARE_VERSION =
        (2 | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Version of the firmware in the device.
    GX_STRING_DEVICE_VERSION =
        (3 | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Version of the device.
    GX_STRING_DEVICE_SERIAL_NUMBER =
        (4 | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Device serial number.
    GX_STRING_FACTORY_SETTING_VERSION =
        (6 | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Factory parameter version
    GX_STRING_DEVICE_USERID =
        (7 | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< User-programmable device identifier.
    GX_INT_DEVICE_LINK_SELECTOR =
        (8 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Selects which Link of the device to control.
    GX_ENUM_DEVICE_LINK_THROUGHPUT_LIMIT_MODE =
        (9 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Controls if the DeviceLinkThroughputLimit is active.
    GX_INT_DEVICE_LINK_THROUGHPUT_LIMIT =
        (10 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Limits the maximum bandwidth of the data that will be streamed out by the device on the selected Link.
    GX_INT_DEVICE_LINK_CURRENT_THROUGHPUT =
        (11 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< The bandwidth of current device acquisition
    GX_COMMAND_DEVICE_RESET =
        (12 | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< reset device
    GX_INT_TIMESTAMP_TICK_FREQUENCY =
        (13 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Time stamp clock frequency
    GX_COMMAND_TIMESTAMP_LATCH =
        (14 | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Timestamp latch
    GX_COMMAND_TIMESTAMP_RESET =
        (15 | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< reset Timestamp
    GX_COMMAND_TIMESTAMP_LATCH_RESET =
        (16 | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< reset Timestamp latch
    GX_INT_TIMESTAMP_LATCH_VALUE =
        (17 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Timestamp Latch value
    GX_STRING_DEVICE_PHY_VERSION =
        (18 | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Device network chip version
    GX_ENUM_DEVICE_TEMPERATURE_SELECTOR =
        (19 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Device temperature selection
    GX_FLOAT_DEVICE_TEMPERATURE =
        (20 | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Device temperature
    GX_STRING_DEVICE_ISP_FIRMWARE_VERSION =
        (21 | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Device ISP firmware version
    GX_ENUM_LOWPOWER_MODE =
        (22 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Low power consumption mode,refer to GX_LOWPOWER_MODE_ENTRY
    GX_ENUM_CLOSE_CCD = (23
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Close CCD, refer to GX_CLOSE_CCD_ENTRY

    //---------------ImageFormat Section--------------------------------
    GX_INT_SENSOR_WIDTH = (1000
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Effective width of the sensor in pixels.
    GX_INT_SENSOR_HEIGHT = (1001
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Effective height of the sensor in pixels.
    GX_INT_WIDTH_MAX = (1002
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Maximum width of the image (in pixels).
    GX_INT_HEIGHT_MAX = (1003
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Maximum height of the image (in pixels).
    GX_INT_OFFSET_X = (1004
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Horizontal offset from the origin to the region of interest (in pixels).
    GX_INT_OFFSET_Y = (1005
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Vertical offset from the origin to the region of interest (in pixels).
    GX_INT_WIDTH = (1006
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Width of the image provided by the device (in pixels).
    GX_INT_HEIGHT = (1007
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Height of the image provided by the device (in pixels).
    GX_INT_BINNING_HORIZONTAL = (1008
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Number of horizontal photo-sensitive cells to combine together.
    GX_INT_BINNING_VERTICAL = (1009
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Number of vertical photo-sensitive cells to combine together.
    GX_INT_DECIMATION_HORIZONTAL = (1010
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Horizontal sub-sampling of the image.
    GX_INT_DECIMATION_VERTICAL = (1011
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Vertical sub-sampling of the image.
    GX_ENUM_PIXEL_SIZE = (1012
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Total size in bits of a pixel of the image.
    GX_ENUM_PIXEL_COLOR_FILTER = (1013
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Type of color filter that is applied to the image.
    GX_ENUM_PIXEL_FORMAT = (1014
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Format of the pixels provided by the device.
    GX_BOOL_REVERSE_X = (1015
        | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Flip horizontally the image sent by the device.
    GX_BOOL_REVERSE_Y = (1016
        | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Flip vertically the image sent by the device.
    GX_ENUM_TEST_PATTERN = (1017
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Selects the type of test pattern that is generated by the device as image source.
    GX_ENUM_TEST_PATTERN_GENERATOR_SELECTOR = (1018
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Selects which test pattern generator is controlled by the TestPattern feature.
    GX_ENUM_REGION_SEND_MODE = (1019
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< ROI output mode, see also GX_REGION_SEND_MODE_ENTRY
    GX_ENUM_REGION_MODE = (1020
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< zone switch, see also GX_REGION_MODE_ENTRY
    GX_ENUM_REGION_SELECTOR = (1021
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Selects the Region of interest to control.
    GX_INT_CENTER_WIDTH = (1022
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< width of window
    GX_INT_CENTER_HEIGHT = (1023
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< height of window
    GX_ENUM_BINNING_HORIZONTAL_MODE = (1024
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Binning Horizontal mode, see also GX_BINNING_HORIZONTAL_MODE_ENTRY
    GX_ENUM_BINNING_VERTICAL_MODE = (1025
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Binning vertical mode, see also GX_BINNING_VERTICAL_MODE_ENTRY
    GX_ENUM_SENSOR_SHUTTER_MODE = (1026
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Sensor shutter mode, refer to GX_SENSOR_SHUTTER_MODE_ENTRY
    GX_INT_DECIMATION_LINENUMBER = (1027
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< decimation line number
    GX_INT_SENSOR_DECIMATION_HORIZONTAL = (1028
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Sensor horizontal decimation
    GX_INT_SENSOR_DECIMATION_VERTICAL = (1029
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Sensor vertical decimation
    GX_ENUM_SENSOR_SELECTOR = (1030
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< selector current sonsor, refer to GX_SENSOR_SELECTOR_ENTRY
    GX_INT_CURRENT_SENSOR_WIDTH = (1031
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< current sonsor width
    GX_INT_CURRENT_SENSOR_HEIGHT = (1032
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< current sonsor height
    GX_INT_CURRENT_SENSOR_OFFSETX = (1033
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< current sonsor offset X
    GX_INT_CURRENT_SENSOR_OFFSETY = (1034
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< current sonsor offset Y
    GX_INT_CURRENT_SENSOR_WIDTHMAX = (1035
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< current sonsor width max
    GX_INT_CURRENT_SENSOR_HEIGHTMAX = (1036
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< current sonsor height max
    GX_ENUM_SENSOR_BIT_DEPTH = (1037
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Sensor Bit Depth, refer to GX_SENSOR_BIT_DEPTH_ENTRY
    GX_BOOL_WATERMARK_ENABLE = (1038
        | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Watermark

    //---------------TransportLayer Section-------------------------------
    GX_INT_PAYLOAD_SIZE = (2000
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Provides the number of bytes transferred for each image or chunk on the stream channel.
    GX_BOOL_GEV_CURRENT_IPCONFIGURATION_LLA = (2001
        | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Controls whether the Link Local Address IP configuration scheme is activated on the given logical link.
    GX_BOOL_GEV_CURRENT_IPCONFIGURATION_DHCP = (2002
        | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Controls whether the DHCP IP configuration scheme is activated on the given logical link.
    GX_BOOL_GEV_CURRENT_IPCONFIGURATION_PERSISTENTIP = (2003
        | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Controls whether the PersistentIP configuration scheme is activated on the given logical link.
    GX_INT_ESTIMATED_BANDWIDTH = (2004
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< EstimatedBandwidth, Unit: Bps(Bytes per second)
    GX_INT_GEV_HEARTBEAT_TIMEOUT = (2005
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Controls the current heartbeat timeout in milliseconds.
    GX_INT_GEV_PACKETSIZE = (2006
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Specifies the stream packet size, in bytes, to send on the selected channel for a GVSP transmitter or specifies the maximum packet size supported by a GVSP receiver.
    GX_INT_GEV_PACKETDELAY = (2007
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Controls the delay (in timestamp counter unit) to insert between each packet for this stream channel.
    GX_INT_GEV_LINK_SPEED = (2008
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< It indicates the connection speed in Mbps for the selected network interface.

    //---------------AcquisitionTrigger Section---------------------------
    GX_ENUM_ACQUISITION_MODE = (3000
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Sets the acquisition mode of the device.
    GX_COMMAND_ACQUISITION_START = (3001
        | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Starts the Acquisition of the device.
    GX_COMMAND_ACQUISITION_STOP = (3002
        | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Stops the Acquisition of the device at the end of the current Frame.
    GX_INT_ACQUISITION_SPEED_LEVEL = (3003
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Setting the speed level of acquiring image.
    GX_INT_ACQUISITION_FRAME_COUNT = (3004
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Number of frames to acquire in MultiFrame Acquisition mode.
    GX_ENUM_TRIGGER_MODE = (3005
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Controls if the selected trigger is active.
    GX_COMMAND_TRIGGER_SOFTWARE = (3006
        | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Generates an internal trigger.
    GX_ENUM_TRIGGER_ACTIVATION = (3007
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Specifies the activation mode of the trigger.
    GX_ENUM_TRIGGER_SWITCH = (3008
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Control external trigger signal is valid, see also GX_TRIGGER_SWITCH_ENTRY
    GX_FLOAT_EXPOSURE_TIME = (3009
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Sets the Exposure time when ExposureMode is Timed and ExposureAuto is Off.
    GX_ENUM_EXPOSURE_AUTO = (3010
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Sets the automatic exposure mode when ExposureMode is Timed.
    GX_FLOAT_TRIGGER_FILTER_RAISING = (3011
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Raising edge signal pulse width is smaller than this value is invalid.
    GX_FLOAT_TRIGGER_FILTER_FALLING = (3012
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Falling edge signal pulse width is smaller than this value is invalid.
    GX_ENUM_TRIGGER_SOURCE = (3013
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Specifies the internal signal or physical input Line to use as the trigger source.
    GX_ENUM_EXPOSURE_MODE = (3014
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Sets the operation mode of the Exposure (or shutter).
    GX_ENUM_TRIGGER_SELECTOR = (3015
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Selects the type of trigger to configure.
    GX_FLOAT_TRIGGER_DELAY = (3016
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Specifies the delay in microseconds (us) to apply after the trigger reception before activating it.
    GX_ENUM_TRANSFER_CONTROL_MODE = (3017
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Selects the control method for the transfers.
    GX_ENUM_TRANSFER_OPERATION_MODE = (3018
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Selects the operation mode of the transfer.
    GX_COMMAND_TRANSFER_START = (3019
        | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Starts the streaming of data blocks out of the device.
    GX_INT_TRANSFER_BLOCK_COUNT = (3020
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< frame number of transmission. when set GX_ENUM_TRANSFER_OPERATION_MODE as GX_ENUM_TRANSFER_OPERATION_MODE_MULTIBLOCK, this function is actived
    GX_BOOL_FRAMESTORE_COVER_ACTIVE = (3021
        | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< FrameBufferOverwriteActive
    GX_ENUM_ACQUISITION_FRAME_RATE_MODE = (3022
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Controls if the acquisitionFrameRate is active, see also GX_ACQUISITION_FRAME_RATE_MODE_ENTRY
    GX_FLOAT_ACQUISITION_FRAME_RATE = (3023
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Controls the acquisition rate (in Hertz) at which the frames are captured.
    GX_FLOAT_CURRENT_ACQUISITION_FRAME_RATE = (3024
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Indicates the maximum allowed frame acquisition rate.
    GX_ENUM_FIXED_PATTERN_NOISE_CORRECT_MODE = (3025
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Controls if the FixedPatternNoise is active, see also GX_FIXED_PATTERN_NOISE_CORRECT_MODE
    GX_INT_ACQUISITION_BURST_FRAME_COUNT = (3030
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< frame number of transmission.
    GX_ENUM_ACQUISITION_STATUS_SELECTOR = (3031
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Acquisition status selection, see also GX_ACQUISITION_STATUS_SELECTOR_ENTRY
    GX_BOOL_ACQUISITION_STATUS = (3032
        | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Acquisition status
    GX_FLOAT_EXPOSURE_DELAY = (30300
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Delay of exposure
    GX_FLOAT_EXPOSURE_OVERLAP_TIME_MAX = (30301
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Maximum overlapping exposure time
    GX_ENUM_EXPOSURE_TIME_MODE = (30302
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Exposure time mode, refer to GX_EXPOSURE_TIME_MODE_ENTRY
    GX_ENUM_ACQUISITION_BURST_MODE = (30303
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Burst acquisition mode
    GX_ENUM_OVERLAP_MODE = (30304
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< overlap mode,refer to GX_OVERLAP_MODE_ENTRY
    GX_ENUM_MULTISOURCE_SELECTOR = (30305
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< MultiSourceSelector to GX_MULTISOURCE_SELECTOR_ENTRY
    GX_BOOL_MULTISOURCE_ENABLE = (30306
        | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< MultiSource Trigger Enable
    GX_BOOL_TRIGGER_CACHE_ENABLE = (30307
        | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Trigger Cache Enable

    //----------------AnalogControls Section----------------------------
    GX_ENUM_GAIN_AUTO = (5000
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Sets the automatic gain control (AGC) mode.
    GX_ENUM_GAIN_SELECTOR = (5001
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Selects which Gain is controlled by the various Gain features.
    GX_ENUM_BLACKLEVEL_AUTO = (5003
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Controls the mode for automatic black level adjustment.
    GX_ENUM_BLACKLEVEL_SELECTOR = (5004
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Selects which Black Level is controlled by the various Black Level features.
    GX_ENUM_BALANCE_WHITE_AUTO = (5006
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Controls the mode for automatic white balancing between the color channels.
    GX_ENUM_BALANCE_RATIO_SELECTOR = (5007
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Selects which Balance ratio to control.
    GX_FLOAT_BALANCE_RATIO = (5008
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Controls ratio of the selected color component to a reference color component.
    GX_ENUM_COLOR_CORRECT = (5009
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Color correction, see also GX_COLOR_CORRECT_ENTRY
    GX_ENUM_DEAD_PIXEL_CORRECT = (5010
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< The dead pixel correct function can eliminate dead pixels in the image, see also GX_DEAD_PIXEL_CORRECT_ENTRY
    GX_FLOAT_GAIN = (5011
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< The value is an float value that sets the selected gain control in units specific to the camera.
    GX_FLOAT_BLACKLEVEL = (5012
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Controls the analog black level as an absolute physical value.
    GX_BOOL_GAMMA_ENABLE = (5013
        | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Enable bit of Gamma
    GX_ENUM_GAMMA_MODE = (5014
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Gamma select, see also GX_GAMMA_MODE_ENTRY
    GX_FLOAT_GAMMA = (5015
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Gamma
    GX_INT_DIGITAL_SHIFT = (5016
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< bit select
    GX_ENUM_LIGHT_SOURCE_PRESET = (5017
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Ambient Light Presets, refer to GX_LIGHT_SOURCE_PRESET_ENTRY
    GX_BOOL_BLACKLEVEL_CALIB_STATUS = (5018
        | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< BlackLevelCalibStatus
    GX_INT_BLACKLEVEL_CALIB_VALUE = (5019
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< BlackLevelCalibValue

    //---------------CustomFeature Section-------------------------
    GX_INT_ADC_LEVEL = (6000
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< When the pixel size is not 8bits, this function can be used to choose 8bits form 10bits or 12bit for show image.
    GX_INT_H_BLANKING = (6001
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Horizontal blanking
    GX_INT_V_BLANKING = (6002
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Vertical blanking
    GX_STRING_USER_PASSWORD = (6003
        | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< user password
    GX_STRING_VERIFY_PASSWORD = (6004
        | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< verify password
    GX_BUFFER_USER_DATA = (6005
        | GX_FEATURE_TYPE::GX_FEATURE_BUFFER as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< user data
    GX_INT_GRAY_VALUE = (6006
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< ExpectedGrayValue_InqIsImplemented
    GX_ENUM_AA_LIGHT_ENVIRONMENT = (6007
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Automatic function according to the external light conditions better for accommodation, see also GX_AA_LIGHT_ENVIRMENT_ENTRY
    GX_INT_AAROI_OFFSETX = (6008
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< This value sets the X offset (left offset) for the rect of interest in pixels for 2A, i.e., the distance in pixels between the left side of the image area and the left side of the AAROI.
    GX_INT_AAROI_OFFSETY = (6009
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< This value sets the Y offset (top offset) for the rect of interest for 2A, i.e., the distance in pixels between the top of the image area and the top of the AAROI.
    GX_INT_AAROI_WIDTH = (6010
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< This value sets the width of the rect of interest in pixels for 2A.
    GX_INT_AAROI_HEIGHT = (6011
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< This value sets the height of the rect of interest in pixels for 2A.
    GX_FLOAT_AUTO_GAIN_MIN = (6012
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Setting up automatic gain range of minimum. When the gain is set to auto mode, this function works.
    GX_FLOAT_AUTO_GAIN_MAX = (6013
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Setting up automatic gain range of maximum. When the gain is set to auto mode, this function works.
    GX_FLOAT_AUTO_EXPOSURE_TIME_MIN = (6014
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Setting up automatic shutter range of minimum. When the shutter is set to auto mode, this function works.
    GX_FLOAT_AUTO_EXPOSURE_TIME_MAX = (6015
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Setting up automatic shutter range of maximum. When the shutter is set to auto mode, this function works.
    GX_BUFFER_FRAME_INFORMATION = (6016
        | GX_FEATURE_TYPE::GX_FEATURE_BUFFER as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< FrameInformation
    GX_INT_CONTRAST_PARAM = (6017
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Contrast parameter
    GX_FLOAT_GAMMA_PARAM = (6018
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Gamma parameter
    GX_INT_COLOR_CORRECTION_PARAM = (6019
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Color correction coefficient
    GX_ENUM_IMAGE_GRAY_RAISE_SWITCH = (6020
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Control ImageGrayRaise is valid, see also GX_IMAGE_GRAY_RAISE_SWITCH_ENTRY
    GX_ENUM_AWB_LAMP_HOUSE = (6021
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Refers to the AWB working environment, see also GX_AWB_LAMP_HOUSE_ENTRY
    GX_INT_AWBROI_OFFSETX = (6022
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< This value sets the X offset (left offset) for the rect of interest in pixels for Auto WhiteBalance
    GX_INT_AWBROI_OFFSETY = (6023
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< This value sets the Y offset (top offset) for the rect of interest for Auto WhiteBalance
    GX_INT_AWBROI_WIDTH = (6024
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< This value sets the width of the rect of interest in pixels for Auto WhiteBalance
    GX_INT_AWBROI_HEIGHT = (6025
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< This value sets the height of the rect of interest in pixels for Auto WhiteBalance
    GX_ENUM_SHARPNESS_MODE = (6026
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Sharpening mode, see also GX_SHARPNESS_MODE_ENTRY
    GX_FLOAT_SHARPNESS = (6027
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Sharpness
    GX_ENUM_USER_DATA_FILED_SELECTOR = (6028
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< User selects Flash data area refer to GX_USER_DATA_FILED_SELECTOR_ENTRY
    GX_BUFFER_USER_DATA_FILED_VALUE = (6029
        | GX_FEATURE_TYPE::GX_FEATURE_BUFFER as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< User Area Content
    GX_ENUM_FLAT_FIELD_CORRECTION = (6030
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Flat field correction switch, refer to GX_FLAT_FIELD_CORRECTION_ENTRY
    GX_ENUM_NOISE_REDUCTION_MODE = (6031
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Noise reduction switch, refer to GX_NOISE_REDUCTION_MODE_ENTRY
    GX_FLOAT_NOISE_REDUCTION = (6032
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Noise reduction
    GX_BUFFER_FFCLOAD = (6033
        | GX_FEATURE_TYPE::GX_FEATURE_BUFFER as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Obtain flat field correction parameters
    GX_BUFFER_FFCSAVE = (6034
        | GX_FEATURE_TYPE::GX_FEATURE_BUFFER as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Set flat field correction parameters
    GX_ENUM_STATIC_DEFECT_CORRECTION = (6035
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Static bad point correction refer to GX_ENUM_STATIC_DEFECT_CORRECTION_ENTRY
    GX_ENUM_2D_NOISE_REDUCTION_MODE = (6036
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Refer to GX_2D_NOISE_REDUCTION_MODE_ENTRY
    GX_ENUM_3D_NOISE_REDUCTION_MODE = (6037
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Refer to GX_3D_NOISE_REDUCTION_MODE_ENTRY
    GX_COMMAND_CLOSE_ISP = (6038
        | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Close ISP
    GX_BUFFER_STATIC_DEFECT_CORRECTION_VALUE_ALL = (6039
        | GX_FEATURE_TYPE::GX_FEATURE_BUFFER as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< static defect conrrection value Refer to GX_BUFFER_FFCSAVE
    GX_BUFFER_STATIC_DEFECT_CORRECTION_FLASH_VALUE = (6040
        | GX_FEATURE_TYPE::GX_FEATURE_BUFFER as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< static defect conrrection flash value Refer to GX_BUFFER_FFCSAVE
    GX_INT_STATIC_DEFECT_CORRECTION_FINISH = (6041
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< static defect conrrection finish Refer to GX_INT_AWBROI_HEIGHT
    GX_BUFFER_STATIC_DEFECT_CORRECTION_INFO = (6042
        | GX_FEATURE_TYPE::GX_FEATURE_BUFFER as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< static defect conrrection Info Refer to GX_BUFFER_FFCSAVE
    GX_COMMAND_STRIP_CALIBRATION_START = (6043
        | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Starts the strip calibration
    GX_COMMAND_STRIP_CALIBRATION_STOP = (6044
        | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Ready to stop the strip calibration

    //---------------UserSetControl Section-------------------------
    GX_ENUM_USER_SET_SELECTOR = (7000
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Selects the feature User Set to load, save or configure.
    GX_COMMAND_USER_SET_LOAD = (7001
        | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Loads the User Set specified by UserSetSelector to the device and makes it active.
    GX_COMMAND_USER_SET_SAVE = (7002
        | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Save the User Set specified by UserSetSelector to the non-volatile memory of the device.
    GX_ENUM_USER_SET_DEFAULT = (7003
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Selects the feature User Set to load and make active by default when the device is reset.

    //---------------Event Section-------------------------
    GX_ENUM_EVENT_SELECTOR = (8000
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Selects which Event to signal to the host application.
    GX_ENUM_EVENT_NOTIFICATION = (8001
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Activate or deactivate the notification to the host application of the occurrence of the selected Event.
    GX_INT_EVENT_EXPOSUREEND = (8002
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Returns the unique identifier of the ExposureEnd type of Event.
    GX_INT_EVENT_EXPOSUREEND_TIMESTAMP = (8003
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Returns the Timestamp of the ExposureEnd Event.
    GX_INT_EVENT_EXPOSUREEND_FRAMEID = (8004
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Returns the unique Identifier of the Frame (or image) that generated the ExposureEnd Event.
    GX_INT_EVENT_BLOCK_DISCARD = (8005
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< This enumeration value indicates the BlockDiscard event ID.
    GX_INT_EVENT_BLOCK_DISCARD_TIMESTAMP = (8006
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Indicates the time stamp for the BlockDiscard event
    GX_INT_EVENT_OVERRUN = (8007
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< This enumeration value indicates the EventOverrun event ID.
    GX_INT_EVENT_OVERRUN_TIMESTAMP = (8008
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Indicates the time stamp of the EventOverrun event
    GX_INT_EVENT_FRAMESTART_OVERTRIGGER = (8009
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< This enumeration value indicates the FrameStartOverTrigger event ID.
    GX_INT_EVENT_FRAMESTART_OVERTRIGGER_TIMESTAMP = (8010
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Indicates the time stamp of the FrameStartOverTrigger event
    GX_INT_EVENT_BLOCK_NOT_EMPTY = (8011
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< This enumeration value indicates the BlockNotEmpty event.
    GX_INT_EVENT_BLOCK_NOT_EMPTY_TIMESTAMP = (8012
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Indicates the time stamp of the BlockNotEmpty event
    GX_INT_EVENT_INTERNAL_ERROR = (8013
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< This enumeration value indicates the InternalError event.
    GX_INT_EVENT_INTERNAL_ERROR_TIMESTAMP = (8014
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Indicates the time stamp of the InternalError event
    GX_INT_EVENT_FRAMEBURSTSTART_OVERTRIGGER = (8015
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Multi frame trigger mask event ID
    GX_INT_EVENT_FRAMEBURSTSTART_OVERTRIGGER_FRAMEID = (8016
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Multi frame trigger mask event frame ID
    GX_INT_EVENT_FRAMEBURSTSTART_OVERTRIGGER_TIMESTAMP = (8017
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Multi frame trigger mask event timestamp
    GX_INT_EVENT_FRAMESTART_WAIT = (8018
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Frame Wait Event ID
    GX_INT_EVENT_FRAMESTART_WAIT_TIMESTAMP = (8019
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Frame Wait Event Timestamp
    GX_INT_EVENT_FRAMEBURSTSTART_WAIT = (8020
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Multi frame waiting event ID
    GX_INT_EVENT_FRAMEBURSTSTART_WAIT_TIMESTAMP = (8021
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Multi frame waiting event timestamp
    GX_INT_EVENT_BLOCK_DISCARD_FRAMEID = (8022
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Block Loss Event Frame ID
    GX_INT_EVENT_FRAMESTART_OVERTRIGGER_FRAMEID = (8023
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Trigger signal masked event frame ID
    GX_INT_EVENT_BLOCK_NOT_EMPTY_FRAMEID = (8024
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< The frame memory is not empty Event frame ID
    GX_INT_EVENT_FRAMESTART_WAIT_FRAMEID = (8025
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Frame Wait Event Frame ID
    GX_INT_EVENT_FRAMEBURSTSTART_WAIT_FRAMEID = (8026
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Multi frame waiting event frame ID
    GX_ENUM_EVENT_SIMPLE_MODE = (8027
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< event block ID enable,refer to GX_EVENT_SIMPLE_MODE_ENTRY

    //---------------LUT Section-------------------------
    GX_ENUM_LUT_SELECTOR = (9000
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Selects which LUT to control.
    GX_BUFFER_LUT_VALUEALL = (9001
        | GX_FEATURE_TYPE::GX_FEATURE_BUFFER as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Accesses all the LUT coefficients in a single access without using individual LUTIndex.
    GX_BOOL_LUT_ENABLE = (9002
        | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Activates the selected LUT.
    GX_INT_LUT_INDEX = (9003
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Control the index (offset) of the coefficient to access in the selected LUT.
    GX_INT_LUT_VALUE = (9004
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Returns the Value at entry LUTIndex of the LUT selected by LUTSelector.

    //---------------ChunkData Section-------------------------
    GX_BOOL_CHUNKMODE_ACTIVE = (10001
        | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Activates the inclusion of Chunk data in the payload of the image.
    GX_ENUM_CHUNK_SELECTOR = (10002
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Selects which Chunk to enable or control.
    GX_BOOL_CHUNK_ENABLE = (10003
        | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Enables the inclusion of the selected Chunk data in the payload of the image.

    //---------------Color Transformation Control-------------------------
    GX_ENUM_COLOR_TRANSFORMATION_MODE = (11000
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Color conversion selection, see also GX_COLOR_TRANSFORMATION_MODE_ENTRY
    GX_BOOL_COLOR_TRANSFORMATION_ENABLE = (11001
        | GX_FEATURE_TYPE::GX_FEATURE_BOOL as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Activates the selected Color Transformation module.
    GX_ENUM_COLOR_TRANSFORMATION_VALUE_SELECTOR = (11002
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Selects the Gain factor or Offset of the Transformation matrix to access in the selected Color Transformation module.
    GX_FLOAT_COLOR_TRANSFORMATION_VALUE = (11003
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Represents the value of the selected Gain factor or Offset inside the Transformation matrix.
    GX_ENUM_SATURATION_MODE = (11004
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Saturation Mode refer to GX_ENUM_SATURATION_MODE_ENTRY
    GX_INT_SATURATION = (11005
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Saturation

    //---------------CounterAndTimerControl Section-------------------------
    GX_ENUM_TIMER_SELECTOR = (12000
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Selects which Counter to configure, Refer to GX_TIMER_SELECTOR_ENTRY
    GX_FLOAT_TIMER_DURATION = (12001
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Sets the duration (in microseconds) of the Timer pulse.
    GX_FLOAT_TIMER_DELAY = (12002
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Sets the duration (in microseconds) of the delay to apply at the reception of a trigger before starting the Timer.
    GX_ENUM_TIMER_TRIGGER_SOURCE = (12003
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Selects the source of the trigger to start the Timer, Refer to GX_TIMER_TRIGGER_SOURCE_ENTRY
    GX_ENUM_COUNTER_SELECTOR = (12004
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Selects which Counter to configure, Refer to GX_COUNTER_SELECTOR_ENTRY
    GX_ENUM_COUNTER_EVENT_SOURCE = (12005
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Select the events that will be the source to increment the Counter, Refer to GX_COUNTER_EVENT_SOURCE_ENTRY
    GX_ENUM_COUNTER_RESET_SOURCE = (12006
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Selects the signals that will be the source to reset the Counter, Refer to GX_COUNTER_RESET_SOURCE_ENTRY
    GX_ENUM_COUNTER_RESET_ACTIVATION = (12007
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Selects the Activation mode of the Counter Reset Source signal, Refer to GX_COUNTER_RESET_ACTIVATION_ENTRY
    GX_COMMAND_COUNTER_RESET = (12008
        | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Does a software reset of the selected Counter and starts it.
    GX_ENUM_COUNTER_TRIGGER_SOURCE = (12009
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Counter trigger source refer to GX_COUNTER_TRIGGER_SOURCE_ENTRY
    GX_INT_COUNTER_DURATION = (12010
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Counter Duration
    GX_ENUM_TIMER_TRIGGER_ACTIVATION = (12011
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Timer Trigger Activation see also GX_TIMER_TRIGGER_ACTIVATION_ENTRY

    //---------------RemoveParameterLimitControl Section-------------------------
    GX_ENUM_REMOVE_PARAMETER_LIMIT = (13000
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Cancel parameter range restriction, refer to GX_REMOVE_PARAMETER_LIMIT_ENTRY

    //---------------HDRControl Section-------------------------
    GX_ENUM_HDR_MODE = (14000
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Refer to GX_HDR_MODE_ENTRY
    GX_INT_HDR_TARGET_LONG_VALUE = (14001
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //<
    GX_INT_HDR_TARGET_SHORT_VALUE = (14002
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //<
    GX_INT_HDR_TARGET_MAIN_VALUE = (14003
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //<

    //---------------MultiGrayControl Section-------------------------
    GX_ENUM_MGC_MODE = (15001
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Refer to GX_MGC_MODE_ENTRY
    GX_INT_MGC_SELECTOR = (15002
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //<
    GX_FLOAT_MGC_EXPOSURE_TIME = (15003
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //<
    GX_FLOAT_MGC_GAIN = (15004
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //<

    //---------------ImageQualityControl Section-------------------------
    GX_BUFFER_STRIPED_CALIBRATION_INFO = (16001
        | GX_FEATURE_TYPE::GX_FEATURE_BUFFER as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Fringe calibration information Refer to GX_BUFFER_STATIC_DEFECT_CORRECTION_INFO
    GX_FLOAT_CONTRAST = (16002
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Contrast

    //---------------GyroControl Section-------------------------
    GX_BUFFER_IMU_DATA = (17001
        | GX_FEATURE_TYPE::GX_FEATURE_BUFFER as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< IMU data
    GX_ENUM_IMU_CONFIG_ACC_RANGE = (17002
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< IMU config acc range, refer to GX_IMU_CONFIG_ACC_RANGE_ENTRY
    GX_ENUM_IMU_CONFIG_ACC_ODR_LOW_PASS_FILTER_SWITCH = (17003
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< IMU config acc odr low pass filter switch, refer to GX_IMU_CONFIG_ACC_ODR_LOW_PASS_FILTER_SWITCH_ENTRY
    GX_ENUM_IMU_CONFIG_ACC_ODR = (17004
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< IMU config acc odr, refer to GX_IMU_CONFIG_ACC_ODR_ENTRY
    GX_ENUM_IMU_CONFIG_ACC_ODR_LOW_PASS_FILTER_FREQUENCY = (17005
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< imu config acc odr low pass filter frequency, refer to GX_IMU_CONFIG_ACC_ODR_LOW_PASS_FILTER_FREQUENCY_ENTRY
    GX_ENUM_IMU_CONFIG_GYRO_XRANGE = (17006
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< imu config gyro Xrange, refer to GX_IMU_CONFIG_GYRO_RANGE_ENTRY
    GX_ENUM_IMU_CONFIG_GYRO_YRANGE = (17007
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< imu config gyro Yrange, refer to GX_IMU_CONFIG_GYRO_RANGE_ENTRY
    GX_ENUM_IMU_CONFIG_GYRO_ZRANGE = (17008
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< imu config gyro Zrange, refer to GX_IMU_CONFIG_GYRO_RANGE_ENTRY
    GX_ENUM_IMU_CONFIG_GYRO_ODR_LOW_PASS_FILTER_SWITCH = (17009
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< imu config gyro odr low pass filter switch, refer to GX_IMU_CONFIG_GYRO_ODR_LOW_PASS_FILTER_SWITCH_ENTRY
    GX_ENUM_IMU_CONFIG_GYRO_ODR = (17010
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< imu config gyro odr, refer to GX_IMU_CONFIG_GYRO_ODR_ENTRY
    GX_ENUM_IMU_CONFIG_GYRO_ODR_LOW_PASS_FILTER_FREQUENCY = (17011
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< imu config gyro odr low pass filter frequency, refer to GX_IMU_CONFIG_GYRO_ODR_LOW_PASS_FILTER_FREQUENCY_ENTRY
    GX_FLOAT_IMU_ROOM_TEMPERATURE = (17012
        | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< imu room temperature
    GX_ENUM_IMU_TEMPERATURE_ODR = (17013
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< imu temperature odr, refer to GX_IMU_TEMPERATURE_ODR_ENTRY

    //---------------FrameBufferControl Section-------------------------
    GX_INT_FRAME_BUFFER_COUNT = (18001
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Frame memory depth
    GX_COMMAND_FRAME_BUFFER_FLUSH = (18002
        | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Empty the frame save

    //----------------SerialPortControl Section----------------------------------
    GX_ENUM_SERIALPORT_SELECTOR = (19001
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Serial port selection
    GX_ENUM_SERIALPORT_SOURCE = (19002
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Serial port input source
    GX_ENUM_SERIALPORT_BAUDRATE = (19003
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Serial baud rate
    GX_INT_SERIALPORT_DATA_BITS = (19004
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Serial port data bit
    GX_ENUM_SERIALPORT_STOP_BITS = (19005
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Serial port stop bit
    GX_ENUM_SERIALPORT_PARITY = (19006
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Serial port parity
    GX_INT_TRANSMIT_QUEUE_MAX_CHARACTER_COUNT = (19007
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Maximum number of characters in transmission queue
    GX_INT_TRANSMIT_QUEUE_CURRENT_CHARACTER_COUNT = (19008
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Current number of characters in the transmission queue
    GX_INT_RECEIVE_QUEUE_MAX_CHARACTER_COUNT = (19009
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Maximum number of characters in receive queue
    GX_INT_RECEIVE_QUEUE_CURRENT_CHARACTER_COUNT = (19010
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Current number of characters in the receive queue
    GX_INT_RECEIVE_FRAMING_ERROR_COUNT = (19011
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Received frame error count
    GX_INT_RECEIVE_PARITY_ERROR_COUNT = (19012
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Receive parity error count
    GX_COMMAND_RECEIVE_QUEUE_CLEAR = (19013
        | GX_FEATURE_TYPE::GX_FEATURE_COMMAND as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Queue Clear
    GX_BUFFER_SERIALPORT_DATA = (19014
        | GX_FEATURE_TYPE::GX_FEATURE_BUFFER as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< serial data
    GX_INT_SERIALPORT_DATA_LENGTH = (19015
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Serial port data length

    //--------------EnoderControl Section-------------------------
    GX_ENUM_ENCODER_SELECTOR = (22001
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Encoder selector
    GX_ENUM_ENCODER_DIRECTION = (22002
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Encoder direction
    GX_INT_ENCODER_VALUE = (22003
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Decoder value
    GX_ENUM_ENCODER_SOURCEA = (22004
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Encoder phase A input
    GX_ENUM_ENCODER_SOURCEB = (22005
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)
        as isize, //< Encoder phase B input
    GX_ENUM_ENCODER_MODE = (22006
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32) as isize, //< Encoder Mode

    //////////////////////////////////////////////////////////////////////////
    /// Local device layer(Device Feature)
    //////////////////////////////////////////////////////////////////////////
    GX_DEV_INT_COMMAND_TIMEOUT = (0
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DEV as u32) as isize, //< Indicates the current command timeout of the specific Link.
    GX_DEV_INT_COMMAND_RETRY_COUNT = (1
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DEV as u32)
        as isize, //< Command retry times

    //////////////////////////////////////////////////////////////////////////
    /// Flow layer(DataStream Feature)
    //////////////////////////////////////////////////////////////////////////
    GX_DS_INT_ANNOUNCED_BUFFER_COUNT =
        (0 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32)
            as isize, //< Number of Buffers declared
    GX_DS_INT_DELIVERED_FRAME_COUNT =
        (1 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32)
            as isize, //< Number of received frames (including residual frames)
    GX_DS_INT_LOST_FRAME_COUNT = (2
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32) as isize, //< Number of frames lost due to insufficient buffers
    GX_DS_INT_INCOMPLETE_FRAME_COUNT =
        (3 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32)
            as isize, //< Number of residual frames received
    GX_DS_INT_DELIVERED_PACKET_COUNT =
        (4 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32)
            as isize, //< Number of packets received
    GX_DS_INT_RESEND_PACKET_COUNT =
        (5 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32)
            as isize, //< Number of retransmission packets
    GX_DS_INT_RESCUED_PACKED_COUNT =
        (6 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32)
            as isize, //< Number of successful retransmitted packets
    GX_DS_INT_RESEND_COMMAND_COUNT =
        (7 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32)
            as isize, //< Repeat command times
    GX_DS_INT_UNEXPECTED_PACKED_COUNT =
        (8 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32)
            as isize, //< Exception packet number
    GX_DS_INT_MAX_PACKET_COUNT_IN_ONE_BLOCK =
        (9 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32)
            as isize, //< Maximum number of retransmissions of data blocks
    GX_DS_INT_MAX_PACKET_COUNT_IN_ONE_COMMAND = (10
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32)
        as isize, //< Maximum number of packets contained in a retransmit command
    GX_DS_INT_RESEND_TIMEOUT = (11
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32) as isize, //< Retransmission timeout time
    GX_DS_INT_MAX_WAIT_PACKET_COUNT = (12
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32)
        as isize, //< Maximum waiting packet number
    GX_DS_ENUM_RESEND_MODE = (13
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32) as isize, //< Retransmission, see also GX_DS_RESEND_MODE_ENTRY
    GX_DS_INT_MISSING_BLOCKID_COUNT = (14
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32)
        as isize, //< Missing number of BlockID
    GX_DS_INT_BLOCK_TIMEOUT = (15
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32) as isize, //< Data block timeout
    GX_DS_INT_STREAM_TRANSFER_SIZE = (16
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32)
        as isize, //< size of transfer block
    GX_DS_INT_STREAM_TRANSFER_NUMBER_URB = (17
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32)
        as isize, //< Number of data blocks transmitted
    GX_DS_INT_PACKET_TIMEOUT = (19
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32) as isize, //< time of package timeout
    GX_DS_INT_SOCKET_BUFFER_SIZE = (20
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32) as isize, //< Socket buffer size in kilobytes
    GX_DS_ENUM_STOP_ACQUISITION_MODE =
        (21 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32) as isize, //< stop acquisition mode
    GX_DS_ENUM_STREAM_BUFFER_HANDLING_MODE =
        (22 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
            | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32) as isize, //< Buffer processing mode, refer to GX_DS_STREAM_BUFFER_HANDLING_MODE_ENTRY
    GX_DS_INT_ACQUISITION_BUFFER_CACHE_PREC = (23
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32)
        as isize, //< Number of buffer caches collected
    GX_DS_ENUM_MULTI_RESEND_MODE = (24
        | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32) as isize, //< Retransmission, see also GX_DS_MULTI_RESEND_MODE_ENTRY

    //////////////////////////////////////////////////////////////////////////
    /// Deprecated Section
    //////////////////////////////////////////////////////////////////////////
    // GX_STRING_DEVICE_ID               = (4    | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize, //< switch to GX_STRING_DEVICE_SERIAL_NUMBER
    // GX_STRING_DEVICE_HARDWARE_VERSION = (5    | GX_FEATURE_TYPE::GX_FEATURE_STRING as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize, //< Device hardware version
    // GX_INT_GAIN                       = (5002 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< switch to GX_FLOAT_GAIN
    // GX_INT_BLACKLEVEL                 = (5005 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< switch to GX_FLOAT_BLACKLEVEL
    // GX_FLOAT_BALANCE_RATIO_SELECTOR   = (5007 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,   //< switch to GX_ENUM_BALANCE_RATIO_SELECTOR
    // GX_ENUM_AA_LIGHT_ENVIRMENT        = (6007 | GX_FEATURE_TYPE::GX_FEATURE_ENUM as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,   //< switch to GX_ENUM_AA_LIGHT_ENVIRONMENT
    // GX_INT_ROI_X                      = (6008 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< switch to GX_INT_AAROI_OFFSETX
    // GX_INT_ROI_Y                      = (6009 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< switch to GX_INT_AAROI_OFFSETY
    // GX_INT_ROI_WIDTH                  = (6010 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< switch to GX_INT_AAROI_WIDTH
    // GX_INT_ROI_HEIGHT                 = (6011 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< switch to GX_INT_AAROI_HEIGHT
    // GX_INT_AUTO_GAIN_VALUEMIN         = (6012 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< switch to GX_FLOAT_AUTO_GAIN_MIN
    // GX_INT_AUTO_GAIN_VALUEMAX         = (6013 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< switch to GX_FLOAT_AUTO_GAIN_MAX
    // GX_INT_AUTO_SHUTTER_VALUEMIN      = (6014 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< switch to GX_FLOAT_AUTO_EXPOSURE_TIME_MIN
    // GX_INT_AUTO_SHUTTER_VALUEMAX      = (6015 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< switch to GX_FLOAT_AUTO_EXPOSURE_TIME_MAX
    // GX_INT_CONTRASTPARAM              = (6017 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< switch to GX_INT_CONTRAST_PARAM
    // GX_FLOAT_GAMMAPARAM               = (6018 | GX_FEATURE_TYPE::GX_FEATURE_FLOAT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,  //< switch to GX_FLOAT_GAMMA_PARAM
    // GX_INT_COLORCORRECTIONPARAM       = (6019 | GX_FEATURE_TYPE::GX_FEATURE_INT as u32 | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_REMOTE_DEV as u32)as isize,    //< switch to GX_INT_COLOR_CORRECTION_PARAM
    GX_DS_INT_MAX_NUM_QUEUE_BUFFER = (18
        | GX_FEATURE_TYPE::GX_FEATURE_INT as u32
        | GX_FEATURE_LEVEL::GX_FEATURE_LEVEL_DS as u32)
        as isize, //< the max number queue buffer
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GX_STATUS_LIST {
    GX_STATUS_SUCCESS = 0,
    GX_STATUS_ERROR = -1,
    GX_STATUS_NOT_FOUND_TL = -2,
    GX_STATUS_NOT_FOUND_DEVICE = -3,
    GX_STATUS_OFFLINE = -4,
    GX_STATUS_INVALID_PARAMETER = -5,
    GX_STATUS_INVALID_HANDLE = -6,
    GX_STATUS_INVALID_CALL = -7,
    GX_STATUS_INVALID_ACCESS = -8,
    GX_STATUS_NEED_MORE_BUFFER = -9,
    GX_STATUS_ERROR_TYPE = -10,
    GX_STATUS_OUT_OF_RANGE = -11,
    GX_STATUS_NOT_IMPLEMENTED = -12,
    GX_STATUS_NOT_INIT_API = -13,
    GX_STATUS_TIMEOUT = -14,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_FRAME_STATUS_LIST {
    GX_FRAME_STATUS_SUCCESS = 0,
    GX_FRAME_STATUS_INCOMPLETE = -1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_DEVICE_CLASS_LIST {
    GX_DEVICE_CLASS_UNKNOWN = 0,
    GX_DEVICE_CLASS_USB2 = 1,
    GX_DEVICE_CLASS_GEV = 2,
    GX_DEVICE_CLASS_U3V = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_ACCESS_STATUS {
    GX_ACCESS_STATUS_UNKNOWN = 0,
    GX_ACCESS_STATUS_READWRITE = 1,
    GX_ACCESS_STATUS_READONLY = 2,
    GX_ACCESS_STATUS_NOACCESS = 3,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_OPEN_MODE {
    GX_OPEN_SN = 0,
    GX_OPEN_IP = 1,
    GX_OPEN_MAC = 2,
    GX_OPEN_INDEX = 3,
    GX_OPEN_USERID = 4,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_IP_CONFIGURE_MODE_LIST {
    GX_IP_CONFIGURE_DHCP,
    GX_IP_CONFIGURE_LLA,
    GX_IP_CONFIGURE_STATIC_IP,
    GX_IP_CONFIGURE_DEFAULT,
}

pub type GX_IP_CONFIGURE_MODE = i32;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub enum GX_RESET_DEVICE_MODE {
    GX_MANUFACTURER_SPECIFIC_RECONNECT = 0x1,
    GX_MANUFACTURER_SPECIFIC_RESET = 0x2,
}

pub type GX_RESET_DEVICE_MODE_CMD = i32;
