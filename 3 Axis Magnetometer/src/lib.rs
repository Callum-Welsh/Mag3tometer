#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!("../bindings.rs");


use std::array;
use std::ffi::CString;
use std::ffi::c_void;
use std::thread::sleep;


const SCANS_PER_READ: u32 = 1600;

//Fn connect_lj
//  Description: connects to first found labjack device
//  Inputs: None
//  Outputs:
//      handle: i32: device handle that will be passed to all future labjack calls; note that it is not the returned number that is important, but it's memory address!!!!!
fn connect_lj() ->i32{
    let address_c_string = CString::new("1").unwrap();
    let mut handle :i32 = 0;
    let ptr: *mut i32 = &mut handle;
    unsafe {
        let e_code = OpenLabJack(LJ_dtU6, LJ_ctUSB, address_c_string.as_ptr() as *const i8, 1, ptr);
        if e_code == 0{
            println!("Labjack connected successfully!");
        }else if e_code == 1007{
            println!("No Labjack device found");
        }else{
            println!("Labjack returned error code {:?}", e_code);
        }
    };
    handle
}

//Fn config_analog_chs_to_pm10
//  Description: Confiures the labjack resolution for specified channels and sets all  analog channels to +/- 10 V gain range
//  Inputs:
 //         handle: &i32: pointer to device handle
        // resolution: i8: resolution index on device, between 0 and 9; see LabJack U6 datasheet for corresponding resolution details
        // channels: Vec<u8>: a vector containing the channels desired for configuration
fn config_analog_chs_to_pm10 (handle:&i32, resolution:i8, channels:Vec<u8>){
    unsafe{
    let mut e_code = ePut (*handle, LJ_ioPUT_CONFIG, LJ_chAIN_RESOLUTION,  resolution as f64, 0);
    if e_code == 0{
        println!("Labjack analog resolution set to index {:?}!", resolution);
    }else if e_code == 1007{
        println!("");
    }else{
        println!("Labjack returned error code {:?}", e_code);
    }
    for i in channels{
        e_code = ePut (*handle,  LJ_ioPUT_AIN_RANGE, i as i32, LJ_rgBIP10V as f64, 0);
        if e_code == 0{
            println!("Labjack analog ch. {:?} configured to +/-10 V", i);
        }else if e_code == 1007{
            println!("");
        }else{
            println!("Labjack returned error code {:?}", e_code);
        }
    };
}
}

//Fn wait_for_trigger
//  Description: Reads a Labjack Digital channel continously; blocks thread until digital high is read
//  Inputs:
        // handle: &i32: pointer to device handle
        // digital channel: i32: digital channel to number to read
        // noisy: bool: Boolean that determines whether or not status updates are printed to console
    //Returns: none
fn wait_for_trigger(handle:&i32, digital_channel:i32, noisy:bool){
    let mut dbl_value: f64 = 0.0;
    let result_ptr: *mut f64 = &mut dbl_value;
    let mut prev_state: f64 = 0.0;
    if noisy{
        println!("Listening for trigger on on digital ch FIO{:?}", digital_channel);
    }
    unsafe{
        let mut counter :u64 = 0;
        AddRequest(*handle, LJ_ioGET_DIGITAL_BIT, digital_channel, 0 as f64, 0,0 as f64);
        GoOne(*handle);
        GetResult(*handle, LJ_ioGET_DIGITAL_BIT, 1, result_ptr);
    

        loop {
            counter += 1;
            prev_state = *result_ptr;
            AddRequest(*handle, LJ_ioGET_DIGITAL_BIT, digital_channel, 0 as f64, 0,0 as f64);
            GoOne(*handle);
            GetResult(*handle, LJ_ioGET_DIGITAL_BIT, 1, result_ptr);
            
            if prev_state-*result_ptr < 0.0{
                if noisy{
                    println!("Triggered on digital ch FIO{:?}", digital_channel);
                };
                break;
            }
            if counter > 10000000000{
                if noisy{
                    println!("Timed out with no trigger");
                };
                break;
            }
            
        }
    };
}

//Fn config_for streaming
//  Description: Configures a Labjack device for streaming mode with "LJ_swSLEEP" by default; blocks thread until desired number of samples are ready for collection from device, which is convinient in that each pull from labjack is of a predictable and uniform size
//  Inputs:
        // handle: &i32: pointer to device handle
        // channel: Vec<u8>: a vector containing the channel numbers desired for streaming
        // scan rate: f64: the scan rate in Hz; smapling rate = scan_rate * number of channels
fn config_for_streaming(handle:&i32, channels:Vec<u8>, scan_rate:f64){
    unsafe{
        //Set the scan rate.
        AddRequest(*handle, LJ_ioPUT_CONFIG, LJ_chSTREAM_SCAN_FREQUENCY, scan_rate, 0, 0 as f64);
        //Give the driver a 2 second buffer (scanRate * 4 channels * 2 seconds); assumes 4 channel scanning by default due to the original application
        AddRequest(*handle, LJ_ioPUT_CONFIG, LJ_chSTREAM_BUFFER_SIZE, scan_rate*4.0*2.0, 0, 0 as f64);
        //Configure reads to retrieve whatever data is available without waiting (wait mode LJ_swNONE).
	    //See comments below to change this program to use LJ_swSLEEP mode.
        AddRequest(*handle, LJ_ioPUT_CONFIG, LJ_chSTREAM_WAIT_MODE, LJ_swSLEEP as f64, 0, 0 as f64);
        //Reset list of channels locked to stream on device
        AddRequest(*handle, LJ_ioCLEAR_STREAM_CHANNELS, 0, 0 as f64, 0, 0 as f64);
        for i in &channels{
            AddRequest(*handle, LJ_ioADD_STREAM_CHANNEL, *i as i32, 0 as f64, 0, 0 as f64);
        }
        let config_result = GoOne(*handle);
        if config_result == 0{
            println!("Successfully configured channels {:?} for streaming at {:?} Hz", channels, scan_rate);
        }else {
            println!("LabJack threw error code {:?}", config_result);
        };
    }
}

//Fn start_stream
//  Description: starts streaming data from configured labjack device
//  Inputs:
        // handle: &i32: pointer to device handle
// Outputs:
        // *result_ptr: f64: the measured scan rate in Hz
fn start_stream(handle:&i32) ->f64{
    stop_stream(handle);
    let mut dbl_value: f64 = 0.0;
    let result_ptr: *mut f64 = &mut dbl_value;
    unsafe{
        eGet(*handle, LJ_ioSTART_STREAM, 0, result_ptr, 0);
        *result_ptr
    }
}

//Fn read_streamed_data
//  Description: pulls data off streaming labjack device
//  Inputs:
        // handle: &i32: pointer to device handle
// Outputs:
        // data_array.to_vec(): Vec<f64>: results from stream in vector format
        // note that the vector's format is [first channel result 1, second channel result 1 ... nth channel result 1, first channel result 2, ....]
fn read_streamed_data(handle:&i32)-> Vec<f64> {
    //SCANS_PER_READ is a constant set in the header as a way of doing a bit of array magic; this is probably bad practice.
    // data_array should be number of channels * SCANS_PER_READ in size to accomodate all of the data coming off of the LabJack at each pull
    let mut data_array: [f64; 4*SCANS_PER_READ as usize] = [0.0; 4*SCANS_PER_READ as usize];
    let mut num_scans_requested = SCANS_PER_READ as f64;
    let num_scans_requested_ptr: *mut f64 = &mut num_scans_requested;

    let d_arr_val_ptr: *mut f64  = &data_array[0] as *const f64 as *mut f64;
    let d_arr_val_ptr: *mut c_void  = d_arr_val_ptr as *mut c_void;

    unsafe{
        eGetPtr(*handle, LJ_ioGET_STREAM_DATA, LJ_chALL_CHANNELS, num_scans_requested_ptr, d_arr_val_ptr);
        return data_array.to_vec()
    }
}

//Fn stop_stream
//  Description: pulls data off streaming labjack device
//  Inputs:
        // handle: &i32: pointer to device handle
// Outputs:
fn stop_stream(handle:&i32){
    unsafe{
        eGet(*handle, LJ_ioSTOP_STREAM, 0,  0 as *mut f64, 0 as i32);
    }
}