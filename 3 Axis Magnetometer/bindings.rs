/* automatically generated by rust-bindgen 0.71.1 */

pub const DRIVER_VERSION: f64 = 3.51;
unsafe extern "C" {
    pub fn Close();
}
unsafe extern "C" {
    pub fn ListAll(
        DeviceType: ::std::os::raw::c_long,
        ConnectionType: ::std::os::raw::c_long,
        pNumFound: *mut ::std::os::raw::c_long,
        pSerialNumbers: *mut ::std::os::raw::c_long,
        pIDs: *mut ::std::os::raw::c_long,
        pAddresses: *mut f64,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn ListAllS(
        pDeviceType: *const ::std::os::raw::c_char,
        pConnectionType: *const ::std::os::raw::c_char,
        pNumFound: *mut ::std::os::raw::c_long,
        pSerialNumbers: *mut ::std::os::raw::c_long,
        pIDs: *mut ::std::os::raw::c_long,
        pAddresses: *mut f64,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn OpenLabJack(
        DeviceType: ::std::os::raw::c_long,
        ConnectionType: ::std::os::raw::c_long,
        pAddress: *const ::std::os::raw::c_char,
        FirstFound: ::std::os::raw::c_long,
        pHandle: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn OpenLabJackS(
        pDeviceType: *const ::std::os::raw::c_char,
        pConnectionType: *const ::std::os::raw::c_char,
        pAddress: *const ::std::os::raw::c_char,
        FirstFound: ::std::os::raw::c_long,
        pHandle: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn AddRequest(
        Handle: ::std::os::raw::c_long,
        IOType: ::std::os::raw::c_long,
        Channel: ::std::os::raw::c_long,
        Value: f64,
        x1: ::std::os::raw::c_long,
        UserData: f64,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn AddRequestS(
        Handle: ::std::os::raw::c_long,
        pIOType: *const ::std::os::raw::c_char,
        Channel: ::std::os::raw::c_long,
        Value: f64,
        x1: ::std::os::raw::c_long,
        UserData: f64,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn AddRequestSS(
        Handle: ::std::os::raw::c_long,
        pIOType: *const ::std::os::raw::c_char,
        pChannel: *const ::std::os::raw::c_char,
        Value: f64,
        x1: ::std::os::raw::c_long,
        UserData: f64,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn AddRequestPtr(
        Handle: ::std::os::raw::c_long,
        IOType: ::std::os::raw::c_long,
        Channel: ::std::os::raw::c_long,
        Value: f64,
        x1: *mut ::std::os::raw::c_void,
        UserData: f64,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn Go() -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn GoOne(Handle: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn eGet(
        Handle: ::std::os::raw::c_long,
        IOType: ::std::os::raw::c_long,
        Channel: ::std::os::raw::c_long,
        pValue: *mut f64,
        x1: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn eGetPtr(
        Handle: ::std::os::raw::c_long,
        IOType: ::std::os::raw::c_long,
        Channel: ::std::os::raw::c_long,
        pValue: *mut f64,
        x1: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn eGetS(
        Handle: ::std::os::raw::c_long,
        pIOType: *const ::std::os::raw::c_char,
        Channel: ::std::os::raw::c_long,
        pValue: *mut f64,
        x1: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn eGetSS(
        Handle: ::std::os::raw::c_long,
        pIOType: *const ::std::os::raw::c_char,
        pChannel: *const ::std::os::raw::c_char,
        pValue: *mut f64,
        x1: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn ePut(
        Handle: ::std::os::raw::c_long,
        IOType: ::std::os::raw::c_long,
        Channel: ::std::os::raw::c_long,
        Value: f64,
        x1: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn ePutS(
        Handle: ::std::os::raw::c_long,
        pIOType: *const ::std::os::raw::c_char,
        Channel: ::std::os::raw::c_long,
        Value: f64,
        x1: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn ePutSS(
        Handle: ::std::os::raw::c_long,
        pIOType: *const ::std::os::raw::c_char,
        pChannel: *const ::std::os::raw::c_char,
        Value: f64,
        x1: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn eGet_DblArray(
        Handle: ::std::os::raw::c_long,
        IOType: ::std::os::raw::c_long,
        Channel: ::std::os::raw::c_long,
        pValue: *mut f64,
        x1: *mut f64,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn eGet_U8Array(
        Handle: ::std::os::raw::c_long,
        IOType: ::std::os::raw::c_long,
        Channel: ::std::os::raw::c_long,
        pValue: *mut f64,
        x1: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn eGetS_DblArray(
        Handle: ::std::os::raw::c_long,
        pIOType: *const ::std::os::raw::c_char,
        Channel: ::std::os::raw::c_long,
        pValue: *mut f64,
        x1: *mut f64,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn eGetS_U8Array(
        Handle: ::std::os::raw::c_long,
        pIOType: *const ::std::os::raw::c_char,
        Channel: ::std::os::raw::c_long,
        pValue: *mut f64,
        x1: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn eGetSS_DblArray(
        Handle: ::std::os::raw::c_long,
        pIOType: *const ::std::os::raw::c_char,
        pChannel: *const ::std::os::raw::c_char,
        pValue: *mut f64,
        x1: *mut f64,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn eGetSS_U8Array(
        Handle: ::std::os::raw::c_long,
        pIOType: *const ::std::os::raw::c_char,
        pChannel: *const ::std::os::raw::c_char,
        pValue: *mut f64,
        x1: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn GetResult(
        Handle: ::std::os::raw::c_long,
        IOType: ::std::os::raw::c_long,
        Channel: ::std::os::raw::c_long,
        pValue: *mut f64,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn GetResultS(
        Handle: ::std::os::raw::c_long,
        pIOType: *const ::std::os::raw::c_char,
        Channel: ::std::os::raw::c_long,
        pValue: *mut f64,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn GetResultSS(
        Handle: ::std::os::raw::c_long,
        pIOType: *const ::std::os::raw::c_char,
        pChannel: *const ::std::os::raw::c_char,
        pValue: *mut f64,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn GetFirstResult(
        Handle: ::std::os::raw::c_long,
        pIOType: *mut ::std::os::raw::c_long,
        pChannel: *mut ::std::os::raw::c_long,
        pValue: *mut f64,
        px1: *mut ::std::os::raw::c_long,
        pUserData: *mut f64,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn GetNextResult(
        Handle: ::std::os::raw::c_long,
        pIOType: *mut ::std::os::raw::c_long,
        pChannel: *mut ::std::os::raw::c_long,
        pValue: *mut f64,
        px1: *mut ::std::os::raw::c_long,
        pUserData: *mut f64,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn eAIN(
        Handle: ::std::os::raw::c_long,
        ChannelP: ::std::os::raw::c_long,
        ChannelN: ::std::os::raw::c_long,
        Voltage: *mut f64,
        Range: ::std::os::raw::c_long,
        Resolution: ::std::os::raw::c_long,
        Settling: ::std::os::raw::c_long,
        Binary: ::std::os::raw::c_long,
        Reserved1: ::std::os::raw::c_long,
        Reserved2: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn eDAC(
        Handle: ::std::os::raw::c_long,
        Channel: ::std::os::raw::c_long,
        Voltage: f64,
        Binary: ::std::os::raw::c_long,
        Reserved1: ::std::os::raw::c_long,
        Reserved2: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn eDI(
        Handle: ::std::os::raw::c_long,
        Channel: ::std::os::raw::c_long,
        State: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn eDO(
        Handle: ::std::os::raw::c_long,
        Channel: ::std::os::raw::c_long,
        State: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn eAddGoGet(
        Handle: ::std::os::raw::c_long,
        NumRequests: ::std::os::raw::c_long,
        aIOTypes: *mut ::std::os::raw::c_long,
        aChannels: *mut ::std::os::raw::c_long,
        aValues: *mut f64,
        ax1s: *mut ::std::os::raw::c_long,
        aRequestErrors: *mut ::std::os::raw::c_long,
        GoError: *mut ::std::os::raw::c_long,
        aResultErrors: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn eTCConfig(
        Handle: ::std::os::raw::c_long,
        aEnableTimers: *mut ::std::os::raw::c_long,
        aEnableCounters: *mut ::std::os::raw::c_long,
        TCPinOffset: ::std::os::raw::c_long,
        TimerClockBaseIndex: ::std::os::raw::c_long,
        TimerClockDivisor: ::std::os::raw::c_long,
        aTimerModes: *mut ::std::os::raw::c_long,
        aTimerValues: *mut f64,
        Reserved1: ::std::os::raw::c_long,
        Reserved2: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn eTCValues(
        Handle: ::std::os::raw::c_long,
        aReadTimers: *mut ::std::os::raw::c_long,
        aUpdateResetTimers: *mut ::std::os::raw::c_long,
        aReadCounters: *mut ::std::os::raw::c_long,
        aResetCounters: *mut ::std::os::raw::c_long,
        aTimerValues: *mut f64,
        aCounterValues: *mut f64,
        Reserved1: ::std::os::raw::c_long,
        Reserved2: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn eModbus(
        Handle: ::std::os::raw::c_long,
        readwrite: ::std::os::raw::c_long,
        addr: ::std::os::raw::c_long,
        size: ::std::os::raw::c_long,
        value: *mut ::std::os::raw::c_uchar,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn ResetLabJack(Handle: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn GetNextError(
        Handle: ::std::os::raw::c_long,
        pIOType: *mut ::std::os::raw::c_long,
        pChannel: *mut ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn GetStreamError(Handle: ::std::os::raw::c_long) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn DoubleToStringAddress(
        Number: f64,
        pString: *mut ::std::os::raw::c_char,
        HexDot: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn StringToDoubleAddress(
        pString: *const ::std::os::raw::c_char,
        pNumber: *mut f64,
        HexDot: ::std::os::raw::c_long,
    ) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn StringToConstant(pString: *const ::std::os::raw::c_char) -> ::std::os::raw::c_long;
}
unsafe extern "C" {
    pub fn ErrorToString(ErrorCode: ::std::os::raw::c_long, pString: *mut ::std::os::raw::c_char);
}
unsafe extern "C" {
    pub fn GetDriverVersion() -> f64;
}
unsafe extern "C" {
    pub fn GetThreadID() -> ::std::os::raw::c_ulong;
}
unsafe extern "C" {
    pub fn TCVoltsToTemp(
        TCType: ::std::os::raw::c_long,
        TCVolts: f64,
        CJTempK: f64,
        pTCTempK: *mut f64,
    ) -> ::std::os::raw::c_long;
}
pub const LJ_dtUE9: ::std::os::raw::c_long = 9;
pub const LJ_dtU3: ::std::os::raw::c_long = 3;
pub const LJ_dtU6: ::std::os::raw::c_long = 6;
pub const LJ_dtSMB: ::std::os::raw::c_long = 1000;
pub const LJ_ctUSB: ::std::os::raw::c_long = 1;
pub const LJ_ctETHERNET: ::std::os::raw::c_long = 2;
pub const LJ_ctETHERNET_MB: ::std::os::raw::c_long = 3;
pub const LJ_ctETHERNET_DATA_ONLY: ::std::os::raw::c_long = 4;
pub const LJ_ctUSB_RAW: ::std::os::raw::c_long = 101;
pub const LJ_ctETHERNET_RAW: ::std::os::raw::c_long = 102;
pub const LJ_ioGET_AIN: ::std::os::raw::c_long = 10;
pub const LJ_ioGET_AIN_DIFF: ::std::os::raw::c_long = 15;
pub const LJ_ioGET_AIN_ADVANCED: ::std::os::raw::c_long = 16;
pub const LJ_ioPUT_AIN_RANGE: ::std::os::raw::c_long = 2000;
pub const LJ_ioGET_AIN_RANGE: ::std::os::raw::c_long = 2001;
pub const LJ_ioPUT_ANALOG_ENABLE_BIT: ::std::os::raw::c_long = 2013;
pub const LJ_ioGET_ANALOG_ENABLE_BIT: ::std::os::raw::c_long = 2014;
pub const LJ_ioPUT_ANALOG_ENABLE_PORT: ::std::os::raw::c_long = 2015;
pub const LJ_ioGET_ANALOG_ENABLE_PORT: ::std::os::raw::c_long = 2016;
pub const LJ_ioPUT_DAC: ::std::os::raw::c_long = 20;
pub const LJ_ioPUT_DAC_ENABLE: ::std::os::raw::c_long = 2002;
pub const LJ_ioGET_DAC_ENABLE: ::std::os::raw::c_long = 2003;
pub const LJ_ioGET_DIGITAL_BIT: ::std::os::raw::c_long = 30;
pub const LJ_ioGET_DIGITAL_BIT_DIR: ::std::os::raw::c_long = 31;
pub const LJ_ioGET_DIGITAL_BIT_STATE: ::std::os::raw::c_long = 32;
pub const LJ_ioGET_DIGITAL_PORT: ::std::os::raw::c_long = 35;
pub const LJ_ioGET_DIGITAL_PORT_DIR: ::std::os::raw::c_long = 36;
pub const LJ_ioGET_DIGITAL_PORT_STATE: ::std::os::raw::c_long = 37;
pub const LJ_ioPUT_DIGITAL_BIT: ::std::os::raw::c_long = 40;
pub const LJ_ioPUT_DIGITAL_PORT: ::std::os::raw::c_long = 45;
pub const LJ_ioPUT_WAIT: ::std::os::raw::c_long = 70;
pub const LJ_ioGET_COUNTER: ::std::os::raw::c_long = 50;
pub const LJ_ioPUT_COUNTER_ENABLE: ::std::os::raw::c_long = 2008;
pub const LJ_ioGET_COUNTER_ENABLE: ::std::os::raw::c_long = 2009;
pub const LJ_ioPUT_COUNTER_RESET: ::std::os::raw::c_long = 2012;
pub const LJ_ioGET_TIMER: ::std::os::raw::c_long = 60;
pub const LJ_ioPUT_TIMER_VALUE: ::std::os::raw::c_long = 2006;
pub const LJ_ioPUT_TIMER_MODE: ::std::os::raw::c_long = 2004;
pub const LJ_ioGET_TIMER_MODE: ::std::os::raw::c_long = 2005;
pub const LJ_ioSHT_GET_READING: ::std::os::raw::c_long = 500;
pub const LJ_ioSPI_COMMUNICATION: ::std::os::raw::c_long = 503;
pub const LJ_ioI2C_COMMUNICATION: ::std::os::raw::c_long = 504;
pub const LJ_ioASYNCH_COMMUNICATION: ::std::os::raw::c_long = 505;
pub const LJ_ioTDAC_COMMUNICATION: ::std::os::raw::c_long = 506;
pub const LJ_ioPIN_CONFIGURATION_RESET: ::std::os::raw::c_long = 2017;
pub const LJ_ioRAW_OUT: ::std::os::raw::c_long = 100;
pub const LJ_ioRAW_IN: ::std::os::raw::c_long = 101;
pub const LJ_ioRAWMB_OUT: ::std::os::raw::c_long = 104;
pub const LJ_ioRAWMB_IN: ::std::os::raw::c_long = 105;
pub const LJ_ioSET_DEFAULTS: ::std::os::raw::c_long = 103;
pub const LJ_ioADD_STREAM_CHANNEL: ::std::os::raw::c_long = 200;
pub const LJ_ioADD_STREAM_CHANNEL_DIFF: ::std::os::raw::c_long = 206;
pub const LJ_ioCLEAR_STREAM_CHANNELS: ::std::os::raw::c_long = 201;
pub const LJ_ioSTART_STREAM: ::std::os::raw::c_long = 202;
pub const LJ_ioSTOP_STREAM: ::std::os::raw::c_long = 203;
pub const LJ_ioADD_STREAM_DAC: ::std::os::raw::c_long = 207;
pub const LJ_ioGET_STREAM_DATA: ::std::os::raw::c_long = 204;
pub type tStreamCallback = ::std::option::Option<
    unsafe extern "C" fn(ScansAvailable: ::std::os::raw::c_long, UserValue: ::std::os::raw::c_long),
>;
pub const LJ_ioSET_STREAM_CALLBACK: ::std::os::raw::c_long = 205;
pub const LJ_ioSET_STREAM_CALLBACK_PTR: ::std::os::raw::c_long = 260;
pub const LJ_ioBUZZER: ::std::os::raw::c_long = 300;
pub type tEventCallback = ::std::option::Option<
    unsafe extern "C" fn(
        EventCode: ::std::os::raw::c_long,
        Data1: ::std::os::raw::c_long,
        Data2: ::std::os::raw::c_long,
        Data3: ::std::os::raw::c_long,
        UserValue: ::std::os::raw::c_long,
    ),
>;
pub const LJ_ioSET_EVENT_CALLBACK: ::std::os::raw::c_long = 400;
pub const LJ_ecDISCONNECT: ::std::os::raw::c_long = 1;
pub const LJ_ecRECONNECT: ::std::os::raw::c_long = 2;
pub const LJ_ecSTREAMERROR: ::std::os::raw::c_long = 4;
pub const LJ_ioPUT_CONFIG: ::std::os::raw::c_long = 1000;
pub const LJ_ioGET_CONFIG: ::std::os::raw::c_long = 1001;
pub const LJ_chLOCALID: ::std::os::raw::c_long = 0;
pub const LJ_chHARDWARE_VERSION: ::std::os::raw::c_long = 10;
pub const LJ_chSERIAL_NUMBER: ::std::os::raw::c_long = 12;
pub const LJ_chFIRMWARE_VERSION: ::std::os::raw::c_long = 11;
pub const LJ_chBOOTLOADER_VERSION: ::std::os::raw::c_long = 15;
pub const LJ_chPRODUCTID: ::std::os::raw::c_long = 8;
pub const LJ_chCOMM_POWER_LEVEL: ::std::os::raw::c_long = 1;
pub const LJ_chIP_ADDRESS: ::std::os::raw::c_long = 2;
pub const LJ_chGATEWAY: ::std::os::raw::c_long = 3;
pub const LJ_chSUBNET: ::std::os::raw::c_long = 4;
pub const LJ_chPORTA: ::std::os::raw::c_long = 5;
pub const LJ_chPORTB: ::std::os::raw::c_long = 6;
pub const LJ_chDHCP: ::std::os::raw::c_long = 7;
pub const LJ_chMACADDRESS: ::std::os::raw::c_long = 9;
pub const LJ_chCOMM_FIRMWARE_VERSION: ::std::os::raw::c_long = 11;
pub const LJ_chCONTROL_POWER_LEVEL: ::std::os::raw::c_long = 13;
pub const LJ_chCONTROL_FIRMWARE_VERSION: ::std::os::raw::c_long = 14;
pub const LJ_chCONTROL_BOOTLOADER_VERSION: ::std::os::raw::c_long = 15;
pub const LJ_chCONTROL_RESET_SOURCE: ::std::os::raw::c_long = 16;
pub const LJ_chUE9_PRO: ::std::os::raw::c_long = 19;
pub const LJ_chLED_STATE: ::std::os::raw::c_long = 17;
pub const LJ_chSDA_SCL: ::std::os::raw::c_long = 18;
pub const LJ_chU3HV: ::std::os::raw::c_long = 22;
pub const LJ_chU6_PRO: ::std::os::raw::c_long = 23;
pub const LJ_chCOMMUNICATION_TIMEOUT: ::std::os::raw::c_long = 20;
pub const LJ_chSTREAM_COMMUNICATION_TIMEOUT: ::std::os::raw::c_long = 21;
pub const LJ_chCAL_CONSTANTS: ::std::os::raw::c_long = 400;
pub const LJ_chUSER_MEM: ::std::os::raw::c_long = 402;
pub const LJ_chUSB_STRINGS: ::std::os::raw::c_long = 404;
pub const LJ_chNUMBER_TIMERS_ENABLED: ::std::os::raw::c_long = 1000;
pub const LJ_chTIMER_CLOCK_BASE: ::std::os::raw::c_long = 1001;
pub const LJ_chTIMER_CLOCK_DIVISOR: ::std::os::raw::c_long = 1002;
pub const LJ_chTIMER_COUNTER_PIN_OFFSET: ::std::os::raw::c_long = 1003;
pub const LJ_chAIN_RESOLUTION: ::std::os::raw::c_long = 2000;
pub const LJ_chAIN_SETTLING_TIME: ::std::os::raw::c_long = 2001;
pub const LJ_chAIN_BINARY: ::std::os::raw::c_long = 2002;
pub const LJ_chDAC_BINARY: ::std::os::raw::c_long = 3000;
pub const LJ_chSHT_TEMP: ::std::os::raw::c_long = 5000;
pub const LJ_chSHT_RH: ::std::os::raw::c_long = 5001;
pub const LJ_chSHT_DATA_CHANNEL: ::std::os::raw::c_long = 5002;
pub const LJ_chSHT_CLOCK_CHANNEL: ::std::os::raw::c_long = 5003;
pub const LJ_chSPI_AUTO_CS: ::std::os::raw::c_long = 5100;
pub const LJ_chSPI_DISABLE_DIR_CONFIG: ::std::os::raw::c_long = 5101;
pub const LJ_chSPI_MODE: ::std::os::raw::c_long = 5102;
pub const LJ_chSPI_CLOCK_FACTOR: ::std::os::raw::c_long = 5103;
pub const LJ_chSPI_MOSI_PIN_NUM: ::std::os::raw::c_long = 5104;
pub const LJ_chSPI_MISO_PIN_NUM: ::std::os::raw::c_long = 5105;
pub const LJ_chSPI_CLK_PIN_NUM: ::std::os::raw::c_long = 5106;
pub const LJ_chSPI_CS_PIN_NUM: ::std::os::raw::c_long = 5107;
pub const LJ_chI2C_ADDRESS_BYTE: ::std::os::raw::c_long = 5108;
pub const LJ_chI2C_SCL_PIN_NUM: ::std::os::raw::c_long = 5109;
pub const LJ_chI2C_SDA_PIN_NUM: ::std::os::raw::c_long = 5110;
pub const LJ_chI2C_OPTIONS: ::std::os::raw::c_long = 5111;
pub const LJ_chI2C_SPEED_ADJUST: ::std::os::raw::c_long = 5112;
pub const LJ_chI2C_READ: ::std::os::raw::c_long = 5113;
pub const LJ_chI2C_WRITE: ::std::os::raw::c_long = 5114;
pub const LJ_chI2C_GET_ACKS: ::std::os::raw::c_long = 5115;
pub const LJ_chI2C_WRITE_READ: ::std::os::raw::c_long = 5130;
pub const LJ_chASYNCH_RX: ::std::os::raw::c_long = 5117;
pub const LJ_chASYNCH_TX: ::std::os::raw::c_long = 5118;
pub const LJ_chASYNCH_FLUSH: ::std::os::raw::c_long = 5128;
pub const LJ_chASYNCH_ENABLE: ::std::os::raw::c_long = 5129;
pub const LJ_chASYNCH_BAUDFACTOR: ::std::os::raw::c_long = 5127;
pub const LJ_chTDAC_SCL_PIN_NUM: ::std::os::raw::c_long = 5119;
pub const LJ_chTDAC_SERIAL_NUMBER: ::std::os::raw::c_long = 5120;
pub const LJ_chTDAC_READ_USER_MEM: ::std::os::raw::c_long = 5121;
pub const LJ_chTDAC_WRITE_USER_MEM: ::std::os::raw::c_long = 5122;
pub const LJ_chTDAC_READ_CAL_CONSTANTS: ::std::os::raw::c_long = 5123;
pub const LJ_chTDAC_WRITE_CAL_CONSTANTS: ::std::os::raw::c_long = 5124;
pub const LJ_chTDAC_UPDATE_DACA: ::std::os::raw::c_long = 5125;
pub const LJ_chTDAC_UPDATE_DACB: ::std::os::raw::c_long = 5126;
pub const LJ_chSTREAM_SCAN_FREQUENCY: ::std::os::raw::c_long = 4000;
pub const LJ_chSTREAM_BUFFER_SIZE: ::std::os::raw::c_long = 4001;
pub const LJ_chSTREAM_CLOCK_OUTPUT: ::std::os::raw::c_long = 4002;
pub const LJ_chSTREAM_EXTERNAL_TRIGGER: ::std::os::raw::c_long = 4003;
pub const LJ_chSTREAM_WAIT_MODE: ::std::os::raw::c_long = 4004;
pub const LJ_chSTREAM_DISABLE_AUTORECOVERY: ::std::os::raw::c_long = 4005;
pub const LJ_chSTREAM_SAMPLES_PER_PACKET: ::std::os::raw::c_long = 4108;
pub const LJ_chSTREAM_READS_PER_SECOND: ::std::os::raw::c_long = 4109;
pub const LJ_chAIN_STREAM_SETTLING_TIME: ::std::os::raw::c_long = 4110;
pub const LJ_chSTREAM_BACKLOG_COMM: ::std::os::raw::c_long = 4105;
pub const LJ_chSTREAM_BACKLOG_CONTROL: ::std::os::raw::c_long = 4106;
pub const LJ_chSTREAM_BACKLOG_UD: ::std::os::raw::c_long = 4107;
pub const LJ_chALL_CHANNELS: ::std::os::raw::c_long = -1;
pub const LJ_INVALID_CONSTANT: ::std::os::raw::c_long = -999;
pub const LJ_ttB: ::std::os::raw::c_long = 6001;
pub const LJ_ttE: ::std::os::raw::c_long = 6002;
pub const LJ_ttJ: ::std::os::raw::c_long = 6003;
pub const LJ_ttK: ::std::os::raw::c_long = 6004;
pub const LJ_ttN: ::std::os::raw::c_long = 6005;
pub const LJ_ttR: ::std::os::raw::c_long = 6006;
pub const LJ_ttS: ::std::os::raw::c_long = 6007;
pub const LJ_ttT: ::std::os::raw::c_long = 6008;
pub const LJ_rgBIP20V: ::std::os::raw::c_long = 1;
pub const LJ_rgBIP10V: ::std::os::raw::c_long = 2;
pub const LJ_rgBIP5V: ::std::os::raw::c_long = 3;
pub const LJ_rgBIP4V: ::std::os::raw::c_long = 4;
pub const LJ_rgBIP2P5V: ::std::os::raw::c_long = 5;
pub const LJ_rgBIP2V: ::std::os::raw::c_long = 6;
pub const LJ_rgBIP1P25V: ::std::os::raw::c_long = 7;
pub const LJ_rgBIP1V: ::std::os::raw::c_long = 8;
pub const LJ_rgBIPP625V: ::std::os::raw::c_long = 9;
pub const LJ_rgBIPP1V: ::std::os::raw::c_long = 10;
pub const LJ_rgBIPP01V: ::std::os::raw::c_long = 11;
pub const LJ_rgUNI20V: ::std::os::raw::c_long = 101;
pub const LJ_rgUNI10V: ::std::os::raw::c_long = 102;
pub const LJ_rgUNI5V: ::std::os::raw::c_long = 103;
pub const LJ_rgUNI4V: ::std::os::raw::c_long = 104;
pub const LJ_rgUNI2P5V: ::std::os::raw::c_long = 105;
pub const LJ_rgUNI2V: ::std::os::raw::c_long = 106;
pub const LJ_rgUNI1P25V: ::std::os::raw::c_long = 107;
pub const LJ_rgUNI1V: ::std::os::raw::c_long = 108;
pub const LJ_rgUNIP625V: ::std::os::raw::c_long = 109;
pub const LJ_rgUNIP5V: ::std::os::raw::c_long = 110;
pub const LJ_rgUNIP25V: ::std::os::raw::c_long = 112;
pub const LJ_rgUNIP3125V: ::std::os::raw::c_long = 111;
pub const LJ_rgUNIP025V: ::std::os::raw::c_long = 113;
pub const LJ_rgUNIP0025V: ::std::os::raw::c_long = 114;
pub const LJ_tmPWM16: ::std::os::raw::c_long = 0;
pub const LJ_tmPWM8: ::std::os::raw::c_long = 1;
pub const LJ_tmRISINGEDGES32: ::std::os::raw::c_long = 2;
pub const LJ_tmFALLINGEDGES32: ::std::os::raw::c_long = 3;
pub const LJ_tmDUTYCYCLE: ::std::os::raw::c_long = 4;
pub const LJ_tmFIRMCOUNTER: ::std::os::raw::c_long = 5;
pub const LJ_tmFIRMCOUNTERDEBOUNCE: ::std::os::raw::c_long = 6;
pub const LJ_tmFREQOUT: ::std::os::raw::c_long = 7;
pub const LJ_tmQUAD: ::std::os::raw::c_long = 8;
pub const LJ_tmTIMERSTOP: ::std::os::raw::c_long = 9;
pub const LJ_tmSYSTIMERLOW: ::std::os::raw::c_long = 10;
pub const LJ_tmSYSTIMERHIGH: ::std::os::raw::c_long = 11;
pub const LJ_tmRISINGEDGES16: ::std::os::raw::c_long = 12;
pub const LJ_tmFALLINGEDGES16: ::std::os::raw::c_long = 13;
pub const LJ_tmLINETOLINE: ::std::os::raw::c_long = 14;
pub const LJ_tc750KHZ: ::std::os::raw::c_long = 0;
pub const LJ_tcSYS: ::std::os::raw::c_long = 1;
pub const LJ_tc2MHZ: ::std::os::raw::c_long = 10;
pub const LJ_tc6MHZ: ::std::os::raw::c_long = 11;
pub const LJ_tc24MHZ: ::std::os::raw::c_long = 12;
pub const LJ_tc500KHZ_DIV: ::std::os::raw::c_long = 13;
pub const LJ_tc2MHZ_DIV: ::std::os::raw::c_long = 14;
pub const LJ_tc6MHZ_DIV: ::std::os::raw::c_long = 15;
pub const LJ_tc24MHZ_DIV: ::std::os::raw::c_long = 16;
pub const LJ_tc4MHZ: ::std::os::raw::c_long = 20;
pub const LJ_tc12MHZ: ::std::os::raw::c_long = 21;
pub const LJ_tc48MHZ: ::std::os::raw::c_long = 22;
pub const LJ_tc1MHZ_DIV: ::std::os::raw::c_long = 23;
pub const LJ_tc4MHZ_DIV: ::std::os::raw::c_long = 24;
pub const LJ_tc12MHZ_DIV: ::std::os::raw::c_long = 25;
pub const LJ_tc48MHZ_DIV: ::std::os::raw::c_long = 26;
pub const LJ_swNONE: ::std::os::raw::c_long = 1;
pub const LJ_swALL_OR_NONE: ::std::os::raw::c_long = 2;
pub const LJ_swPUMP: ::std::os::raw::c_long = 11;
pub const LJ_swSLEEP: ::std::os::raw::c_long = 12;
pub const LJ_ioSWDT_CONFIG: ::std::os::raw::c_long = 507;
pub const LJ_ioSWDT_STROKE: ::std::os::raw::c_long = 508;
pub const LJ_chSWDT_ENABLE: ::std::os::raw::c_long = 5200;
pub const LJ_chSWDT_DISABLE: ::std::os::raw::c_long = 5201;
pub const LJ_chSWDT_RESET_DEVICE: ::std::os::raw::c_long = 5202;
pub const LJ_chSWDT_RESET_COMM: ::std::os::raw::c_long = 5203;
pub const LJ_chSWDT_RESET_CONTROL: ::std::os::raw::c_long = 5204;
pub const LJ_chSWDT_UPDATE_DIOA: ::std::os::raw::c_long = 5205;
pub const LJ_chSWDT_UPDATE_DIOB: ::std::os::raw::c_long = 5206;
pub const LJ_chSWDT_DIOA_CHANNEL: ::std::os::raw::c_long = 5207;
pub const LJ_chSWDT_DIOA_STATE: ::std::os::raw::c_long = 5208;
pub const LJ_chSWDT_DIOB_CHANNEL: ::std::os::raw::c_long = 5209;
pub const LJ_chSWDT_DIOB_STATE: ::std::os::raw::c_long = 5210;
pub const LJ_chSWDT_UPDATE_DAC0: ::std::os::raw::c_long = 5211;
pub const LJ_chSWDT_UPDATE_DAC1: ::std::os::raw::c_long = 5212;
pub const LJ_chSWDT_DAC0: ::std::os::raw::c_long = 5213;
pub const LJ_chSWDT_DAC1: ::std::os::raw::c_long = 5214;
pub const LJ_chSWDT_DAC_ENABLE: ::std::os::raw::c_long = 5215;
pub const LJ_chSWDT_STRICT_ENABLE: ::std::os::raw::c_long = 5216;
pub const LJ_chSWDT_INITIAL_ROLL_TIME: ::std::os::raw::c_long = 5217;
pub const LJE_NOERROR: ::std::os::raw::c_long = 0;
pub const LJE_COMMAND_LIST_ERROR: ::std::os::raw::c_long = 1;
pub const LJE_INVALID_CHANNEL_NUMBER: ::std::os::raw::c_long = 2;
pub const LJE_INVALID_RAW_INOUT_PARAMETER: ::std::os::raw::c_long = 3;
pub const LJE_UNABLE_TO_START_STREAM: ::std::os::raw::c_long = 4;
pub const LJE_UNABLE_TO_STOP_STREAM: ::std::os::raw::c_long = 5;
pub const LJE_NOTHING_TO_STREAM: ::std::os::raw::c_long = 6;
pub const LJE_UNABLE_TO_CONFIG_STREAM: ::std::os::raw::c_long = 7;
pub const LJE_BUFFER_OVERRUN: ::std::os::raw::c_long = 8;
pub const LJE_STREAM_NOT_RUNNING: ::std::os::raw::c_long = 9;
pub const LJE_INVALID_PARAMETER: ::std::os::raw::c_long = 10;
pub const LJE_INVALID_STREAM_FREQUENCY: ::std::os::raw::c_long = 11;
pub const LJE_INVALID_AIN_RANGE: ::std::os::raw::c_long = 12;
pub const LJE_STREAM_CHECKSUM_ERROR: ::std::os::raw::c_long = 13;
pub const LJE_STREAM_COMMAND_ERROR: ::std::os::raw::c_long = 14;
pub const LJE_STREAM_ORDER_ERROR: ::std::os::raw::c_long = 15;
pub const LJE_AD_PIN_CONFIGURATION_ERROR: ::std::os::raw::c_long = 16;
pub const LJE_REQUEST_NOT_PROCESSED: ::std::os::raw::c_long = 17;
pub const LJE_SCRATCH_ERROR: ::std::os::raw::c_long = 19;
pub const LJE_DATA_BUFFER_OVERFLOW: ::std::os::raw::c_long = 20;
pub const LJE_ADC0_BUFFER_OVERFLOW: ::std::os::raw::c_long = 21;
pub const LJE_FUNCTION_INVALID: ::std::os::raw::c_long = 22;
pub const LJE_SWDT_TIME_INVALID: ::std::os::raw::c_long = 23;
pub const LJE_FLASH_ERROR: ::std::os::raw::c_long = 24;
pub const LJE_STREAM_IS_ACTIVE: ::std::os::raw::c_long = 25;
pub const LJE_STREAM_TABLE_INVALID: ::std::os::raw::c_long = 26;
pub const LJE_STREAM_CONFIG_INVALID: ::std::os::raw::c_long = 27;
pub const LJE_STREAM_BAD_TRIGGER_SOURCE: ::std::os::raw::c_long = 28;
pub const LJE_STREAM_INVALID_TRIGGER: ::std::os::raw::c_long = 30;
pub const LJE_STREAM_ADC0_BUFFER_OVERFLOW: ::std::os::raw::c_long = 31;
pub const LJE_STREAM_SAMPLE_NUM_INVALID: ::std::os::raw::c_long = 33;
pub const LJE_STREAM_BIPOLAR_GAIN_INVALID: ::std::os::raw::c_long = 34;
pub const LJE_STREAM_SCAN_RATE_INVALID: ::std::os::raw::c_long = 35;
pub const LJE_TIMER_INVALID_MODE: ::std::os::raw::c_long = 36;
pub const LJE_TIMER_QUADRATURE_AB_ERROR: ::std::os::raw::c_long = 37;
pub const LJE_TIMER_QUAD_PULSE_SEQUENCE: ::std::os::raw::c_long = 38;
pub const LJE_TIMER_BAD_CLOCK_SOURCE: ::std::os::raw::c_long = 39;
pub const LJE_TIMER_STREAM_ACTIVE: ::std::os::raw::c_long = 40;
pub const LJE_TIMER_PWMSTOP_MODULE_ERROR: ::std::os::raw::c_long = 41;
pub const LJE_TIMER_SEQUENCE_ERROR: ::std::os::raw::c_long = 42;
pub const LJE_TIMER_SHARING_ERROR: ::std::os::raw::c_long = 43;
pub const LJE_TIMER_LINE_SEQUENCE_ERROR: ::std::os::raw::c_long = 44;
pub const LJE_EXT_OSC_NOT_STABLE: ::std::os::raw::c_long = 45;
pub const LJE_INVALID_POWER_SETTING: ::std::os::raw::c_long = 46;
pub const LJE_PLL_NOT_LOCKED: ::std::os::raw::c_long = 47;
pub const LJE_INVALID_PIN: ::std::os::raw::c_long = 48;
pub const LJE_IOTYPE_SYNCH_ERROR: ::std::os::raw::c_long = 49;
pub const LJE_INVALID_OFFSET: ::std::os::raw::c_long = 50;
pub const LJE_FEEDBACK_IOTYPE_NOT_VALID: ::std::os::raw::c_long = 51;
pub const LJE_CANT_CONFIGURE_PIN_FOR_ANALOG: ::std::os::raw::c_long = 67;
pub const LJE_CANT_CONFIGURE_PIN_FOR_DIGITAL: ::std::os::raw::c_long = 68;
pub const LJE_TC_PIN_OFFSET_MUST_BE_4_TO_8: ::std::os::raw::c_long = 70;
pub const LJE_INVALID_DIFFERENTIAL_CHANNEL: ::std::os::raw::c_long = 71;
pub const LJE_DSP_SIGNAL_OUT_OF_RANGE: ::std::os::raw::c_long = 72;
pub const LJE_SHT_CRC: ::std::os::raw::c_long = 52;
pub const LJE_SHT_MEASREADY: ::std::os::raw::c_long = 53;
pub const LJE_SHT_ACK: ::std::os::raw::c_long = 54;
pub const LJE_SHT_SERIAL_RESET: ::std::os::raw::c_long = 55;
pub const LJE_SHT_COMMUNICATION: ::std::os::raw::c_long = 56;
pub const LJE_AIN_WHILE_STREAMING: ::std::os::raw::c_long = 57;
pub const LJE_STREAM_TIMEOUT: ::std::os::raw::c_long = 58;
pub const LJE_STREAM_CONTROL_BUFFER_OVERFLOW: ::std::os::raw::c_long = 59;
pub const LJE_STREAM_SCAN_OVERLAP: ::std::os::raw::c_long = 60;
pub const LJE_FIRMWARE_VERSION_IOTYPE: ::std::os::raw::c_long = 61;
pub const LJE_FIRMWARE_VERSION_CHANNEL: ::std::os::raw::c_long = 62;
pub const LJE_FIRMWARE_VERSION_VALUE: ::std::os::raw::c_long = 63;
pub const LJE_HARDWARE_VERSION_IOTYPE: ::std::os::raw::c_long = 64;
pub const LJE_HARDWARE_VERSION_CHANNEL: ::std::os::raw::c_long = 65;
pub const LJE_HARDWARE_VERSION_VALUE: ::std::os::raw::c_long = 66;
pub const LJE_LJTDAC_ACK_ERROR: ::std::os::raw::c_long = 69;
pub const LJE_STREAM_INVALID_CONNECTION: ::std::os::raw::c_long = 73;
pub const LJE_MIN_GROUP_ERROR: ::std::os::raw::c_long = 1000;
pub const LJE_UNKNOWN_ERROR: ::std::os::raw::c_long = 1001;
pub const LJE_INVALID_DEVICE_TYPE: ::std::os::raw::c_long = 1002;
pub const LJE_INVALID_HANDLE: ::std::os::raw::c_long = 1003;
pub const LJE_DEVICE_NOT_OPEN: ::std::os::raw::c_long = 1004;
pub const LJE_NO_DATA_AVAILABLE: ::std::os::raw::c_long = 1005;
pub const LJE_NO_MORE_DATA_AVAILABLE: ::std::os::raw::c_long = 1006;
pub const LJE_LABJACK_NOT_FOUND: ::std::os::raw::c_long = 1007;
pub const LJE_COMM_FAILURE: ::std::os::raw::c_long = 1008;
pub const LJE_CHECKSUM_ERROR: ::std::os::raw::c_long = 1009;
pub const LJE_DEVICE_ALREADY_OPEN: ::std::os::raw::c_long = 1010;
pub const LJE_COMM_TIMEOUT: ::std::os::raw::c_long = 1011;
pub const LJE_USB_DRIVER_NOT_FOUND: ::std::os::raw::c_long = 1012;
pub const LJE_INVALID_CONNECTION_TYPE: ::std::os::raw::c_long = 1013;
pub const LJE_INVALID_MODE: ::std::os::raw::c_long = 1014;
pub const LJE_DEVICE_NOT_CONNECTED: ::std::os::raw::c_long = 1015;
pub const LJE_DISCONNECT: ::std::os::raw::c_long = 2000;
pub const LJE_RECONNECT: ::std::os::raw::c_long = 2001;
pub const LJE_MIN_USER_ERROR: ::std::os::raw::c_long = 3000;
pub const LJE_MAX_USER_ERROR: ::std::os::raw::c_long = 3999;
pub const LJE_DEVICE_NOT_CALIBRATED: ::std::os::raw::c_long = -1;
pub const LJE_UNABLE_TO_READ_CALDATA: ::std::os::raw::c_long = -2;
pub const LJ_ioANALOG_INPUT: ::std::os::raw::c_long = 10;
pub const LJ_ioANALOG_OUTPUT: ::std::os::raw::c_long = 20;
pub const LJ_ioDIGITAL_BIT_IN: ::std::os::raw::c_long = 30;
pub const LJ_ioDIGITAL_PORT_IN: ::std::os::raw::c_long = 35;
pub const LJ_ioDIGITAL_BIT_OUT: ::std::os::raw::c_long = 40;
pub const LJ_ioDIGITAL_PORT_OUT: ::std::os::raw::c_long = 45;
pub const LJ_ioCOUNTER: ::std::os::raw::c_long = 50;
pub const LJ_ioTIMER: ::std::os::raw::c_long = 60;
pub const LJ_ioPUT_COUNTER_MODE: ::std::os::raw::c_long = 2010;
pub const LJ_ioGET_COUNTER_MODE: ::std::os::raw::c_long = 2011;
pub const LJ_ioGET_TIMER_VALUE: ::std::os::raw::c_long = 2007;
pub const LJ_ioCYCLE_PORT: ::std::os::raw::c_long = 102;
pub const LJ_chTIMER_CLOCK_CONFIG: ::std::os::raw::c_long = 1001;
pub const LJ_ioPUT_CAL_CONSTANTS: ::std::os::raw::c_long = 400;
pub const LJ_ioGET_CAL_CONSTANTS: ::std::os::raw::c_long = 401;
pub const LJ_ioPUT_USER_MEM: ::std::os::raw::c_long = 402;
pub const LJ_ioGET_USER_MEM: ::std::os::raw::c_long = 403;
pub const LJ_ioPUT_USB_STRINGS: ::std::os::raw::c_long = 404;
pub const LJ_ioGET_USB_STRINGS: ::std::os::raw::c_long = 405;
pub const LJ_ioSHT_DATA_CHANNEL: ::std::os::raw::c_long = 501;
pub const LJ_ioSHT_CLOCK_CHANNEL: ::std::os::raw::c_long = 502;
pub const LJ_chI2C_ADDRESS: ::std::os::raw::c_long = 5108;
pub const LJ_chASYNCH_CONFIG: ::std::os::raw::c_long = 5116;
pub const LJ_rgUNIP500V: ::std::os::raw::c_long = 110;
pub const LJ_ioENABLE_POS_PULLDOWN: ::std::os::raw::c_long = 2018;
pub const LJ_ioENABLE_NEG_PULLDOWN: ::std::os::raw::c_long = 2019;
pub const LJ_rgAUTO: ::std::os::raw::c_long = 0;
pub const LJ_chSWDT_UDPATE_DIOA: ::std::os::raw::c_long = 5205;
