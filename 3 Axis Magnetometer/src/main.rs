use rust_bindings::eGet;
use rust_bindings::GetResult;
use rust_bindings::GoOne;
use rust_bindings::AddRequest;
use rust_bindings::OpenLabJack;
use rust_bindings::LJ_dtU6;
use rust_bindings::LJ_ctUSB;
use rust_bindings::ePut;
use rust_bindings::LJ_ioPUT_CONFIG;
use rust_bindings::LJ_chAIN_RESOLUTION;
use rust_bindings::LJ_ioPUT_AIN_RANGE;
use rust_bindings::LJ_rgBIP10V;
use rust_bindings::LJ_ioGET_DIGITAL_BIT;
use rust_bindings::LJ_chSTREAM_SCAN_FREQUENCY;
use rust_bindings::{LJ_ioSTOP_STREAM, LJ_chSTREAM_BUFFER_SIZE, LJ_chSTREAM_WAIT_MODE, LJ_swNONE, LJ_ioCLEAR_STREAM_CHANNELS, LJ_ioADD_STREAM_CHANNEL, LJ_ioSTART_STREAM, LJ_swSLEEP, LJ_ioGET_STREAM_DATA, eGetPtr, LJ_chALL_CHANNELS};
use tokio::sync::mpsc;
use core::time;
use std::ffi::CString;
use std::ffi::c_void;
use std::{array, io::{self, Write}, thread::current, u16::MAX};
use std::time::SystemTime;
use std::time::Instant;
use rand::random;
use tokio::{stream, task};
use tokio::time::{sleep, Duration};
use zmq::{self, Socket};
extern crate chrono;
use chrono::prelude::*;
use std::fs::OpenOptions;
use csv::Writer;
use serde_json::json;
use std::fs;
use rustfft::{FftPlanner, num_complex::Complex};



//async fn main() -> io::Result<()> {
#[tokio::main]
async fn main(){
    const SCAN_RATE:f64 = 2000.0;
    let trace_duration: f64 = 0.1;
    //let (tx1, mut rx1) = mpsc::channel(32);
    //let (tx2, mut rx2) = mpsc::channel(32);

    // Set up ZeroMQ context and socket1
    let context = zmq::Context::new();
    let socket1 = context.socket(zmq::PUSH).expect("Failed to create socket");
    let socket2 = context.socket(zmq::PUSH).expect("Failed to create socket");
    let socket3 = context.socket(zmq::PUSH).expect("Failed to create socket");
    let socket4 = context.socket(zmq::PUSH).expect("Failed to create socket");
    //Socket 1: send slow data to python gui
    socket1.connect("tcp://localhost:5555").expect("Failed to connect to Python GUI");
    //Socket 2: send XYZ mean from scope traces to camera
    socket2.connect("tcp://localhost:49154").expect("Failed to connect to Camera");
    //Socket 3: send raw triggered traces to python gui
    socket3.connect("tcp://localhost:49156").expect("Failed to connect to Python GUI");
    //Socket 4: send fft data to python gui
    socket4.connect("tcp://localhost:49158").expect("Failed to connect to Python GUI");

//The below commented block used to implement tokio to do all data processing on a seperate thread as data aquisition.
// Long story short is it was a little buggy, so I gave up and did it all on a single thread. If you feel up to figuring it out, uncomment the below.

    //fs::create_dir(r"C:\b_field_traces")?; CHANGE THIS TO LOAD PATH FROM CARMERA!
/*
    task::spawn(async move {
        println!("Ran publication Thread!");
        let mut save_buffer: Vec<f64> = Vec::new();
        let mut data_buffer: Vec<f64> = Vec::new();
        loop {
            println!("Ran publication Thread!");
            let mut fresh_data: Vec<f64> = rx1.recv().await.unwrap();  
            for i in 0..fresh_data.len(){
                if i%10 ==0{
                    let db_string = format!("{:?}", [fresh_data[i]].to_vec());
                    socket1.send(&db_string, 0).expect("Failed to send message");
                }
            }
            /*
            if data_buffer.len() as u32 == 10{
                let db_string = format!("{:?}", data_buffer);
                socket1.send(&db_string, 0).expect("Failed to send message");
                println!("Published 100 points");
                //println!("Sent to Python GUI: {:?}", data_buffer);
                save_buffer.append(&mut data_buffer);
                data_buffer.clear();
            }
            */
            //println!("Sent to Python GUI: {:?}", data_buffer);

    
            if save_buffer.len() > 500000 {
                let _result = save_to_file(&save_buffer, &save_buffer, "test_log.csv").await;
                save_buffer.clear()
            }
        }
    });
    */
    /* 
    task::spawn(async move {
        loop {
            let current_time: DateTime<Local> = Local::now();
    
            // Format it as "Year-Month-Day HH:MM:SS"
            let formatted_time = current_time.format("%Y-%m-%d %H:%M:%S").to_string();
            socket2.send(&formatted_time, 0).expect("Failed to send message");

        }
    });
     */
/*
    task::spawn(async move{
        let mut timestamps:Vec<f64> = [].to_vec();
        for i in (0 as u32)..(200 as u32){
            timestamps.push(i as f64);
        }
        loop{
                //let mut fresh_trace = rx2.recv().await.unwrap();
                let t_time: DateTime<Local> = Local::now();
                
                let package = aquire_data(t_time.format("%Y-%m-%d %H:%M:%S").to_string(), fresh_trace, [0.0].to_vec()).to_string();
                socket3.send(&package, 0).expect("Failed to send message");
                println!("Sent trace!");
                //println!("Trigger line asleep for {:?} miliseconds during aquistion and transmission", timing.elapsed().as_millis());
        }
    });
*/
//Spawn a tokio thread to do our bidding; there is only one thread, so this is techically redundnat, but architecutre allows for seamless addition of more processing threads in the future if we want
//task::spawn(async move{
        let device_handle = connect_lj(); //Connect to a labjack U6 and get the device handle for future calls
        config_analog_chs_to_pm10(&device_handle, 3, [2,1, 3].to_vec()); //Configure specified device channels to p/m 10 output gain. See datasheet for resolution index meaning
        config_for_streaming(&device_handle, [2,3,1, 193].to_vec(), SCAN_RATE); //Configure specified device channels for data streeaming at 2kHz. Channel 193 corresponds to digital line for triggering
        let real_scan_rate = start_stream(&device_handle); //start steam
        let mut data:Vec<f64> = [].to_vec(); //container to hold the raw data
        let mut ch1:Vec<f64> = [].to_vec(); //container to hold ch1 (y) channel data
        let mut ch2:Vec<f64> = [].to_vec(); // cointainer to hold ch2 (x) channel data
        let mut ch3:Vec<f64> = [].to_vec(); // cointainer to hold ch 3(z) channel data
        let mut time_ax:Vec<f64> = [].to_vec(); //container to log the time elapsed
        let mut ch1_trace:Vec<f64> = [].to_vec(); //container to hold the high resolution traces
        let mut ch2_trace:Vec<f64> = [].to_vec();
        let mut ch3_trace:Vec<f64> = [].to_vec();
        let mut fft_buffer:Vec<Complex<f32>> = [Complex{re:0.0, im:0.0};512].to_vec(); //container to hold 512 samples for FFT
        let expected_samples: u32 = (real_scan_rate*trace_duration) as u32; //How many samples do we expect in a trace for a given trace duration?
        let mut planner = FftPlanner::<f32>::new(); // a scheduler for the FFT
        let fft = planner.plan_fft_forward(512);
        let mut time_counter: u32 = 0; //A time counter to reset things every 15 minutes or so
        loop{
            println!("1");
            time_counter +=1;
            if time_counter > 30000{
                //Resets every ~15 minutes
                start_stream(&device_handle);
                time_counter = 0;
            }
            let timing = Instant::now();
            data = read_streamed_data(&device_handle);
            println!("2");
            (ch1, ch2, ch3, ch1_trace, ch2_trace, ch3_trace) = parse_data(data, ch1_trace, ch2_trace, ch3_trace, expected_samples);
            if ch1_trace.len() as u32 == expected_samples{
                println!("2.5");
                //tx2.send(ch1_trace).await.unwrap();
                let t_time: DateTime<Local> = Local::now(); //time at trace complete
                let package = aquire_data(t_time.format("%Y-%m-%d %H:%M:%S").to_string(), ch1_trace, [0.0].to_vec()); //processed trace
                let x_mean = mean(&ch2_trace); //mean of x channel
                let z_mean = mean(&ch3_trace); // mean of z channel
                let for_camera = json!({"y":&package["mean"], "x":&x_mean, "z":&z_mean}); //compute  mean field value over trace to camera for all 3 axes
                //socket2.send(&for_camera.to_string(), 0).expect("Failed to send message"); //transmit
                println!("{:?}", &for_camera.to_string());
                socket3.send(&package.to_string(), 0).expect("Failed to send message"); //send the complete trace for the y axis to the python gui
                let t_string = t_time.format("%Y-%m-%d_%H%M%S").to_string(); //timestamp for filename
                let mut data_directory:String = r"C:\b_field_traces\".to_string(); //save the trace to a file with the timestamped file name
                save_trace(package.to_string(), [data_directory,t_string, ".txt".to_string()].join("")).await;
                println!("Sent trace!");
                ch1_trace = [].to_vec();
                ch2_trace = [].to_vec();
                ch3_trace = [].to_vec();
                println!("Filled trace");
            }
            println!("3");
            //tx1.send(ch1).await.unwrap();
            if 0 ==0{ //stupid; should remove, but can be usfeul for debugging
                // takes the mean over each buffer pull to send to the python gui so we don't overwhelm it
                let mut sum:f64 = 0.0;
                let mut mean:f64=0.0;
                for i in &ch1{
                    sum = sum+*i;
                }
                mean = sum/ch1.len() as f64;
                let db_string = format!("{:?}", [100.0*mean].to_vec());
                socket1.send(&db_string, 0).expect("Failed to send message"); //sends the mean of the 100 buffered samples to python gui
                println!("4");
                //feeds values to FFT buffer
                for i in ch1{
                    fft_buffer.push(Complex{re:i as f32, im:0.0});
                    fft_buffer.remove(0);
                }
                println!("5");
                //compute the FFT
                let mut result = fft_buffer.clone();
                fft.process(&mut result);
                let proc_result: Vec<f32> = result
                    .iter()
                    .take(257)
                    .map(|&x| x.norm_sqr())
                    .collect();
                println!("6");
                socket4.send(&format!("{:?}",proc_result), 0).expect("Failed to send message"); //transmit FFT to python gui
                println!("7");
            }
            
            println!("{:?}", timing.elapsed().as_millis()); //stop the clock; prints the number of miliseconds each loop took to the command line.
        }
//    });

//    loop {
//        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
  //  }
}

//Defines how many samples are allowed to accumulate in the labjack buffer before being read from board. Max is 950
const SCANS_PER_READ: u32 = 100;

//async function save_to_file
//  Description: saves vec64 data and corresponding timestamps to a file of your choosing
//  Inputs:
//      - data: &Vec<f64>: a pointer to a vector of f64s that you would like to save
//      - timestamps &Vec<f64>: a pointer to a vector of f64s that timestamp the data you would like to save
//      - filename: the path to a text file where the data will be saved
//  Outputs:
//      -   io::Result: Ok: returns the thread to the pool when complete
async fn save_to_file(data: &Vec<f64>, timestamps: &Vec<f64>,file_path: &str) -> io::Result<()>{
    let mut wtr = Writer::from_path(file_path)?;
    for (value, timestamp) in data.iter().zip(timestamps.iter()) {
        wtr.write_record(&[value.to_string(), timestamp.to_string()])?;
    }
    wtr.flush()?;
    Ok(())
}

// function aquire_data (probably poorly named; should change if brave)
//  Description: takes a data vector from the board, computes stats on it, and returns it as a JSON block for transmission
//  Inputs:
//      - time_trig: String: the timestamp of the trace trigger time as a string
//      - streamed_data: Vec<f64>: vector of f64s that you would like to save
//      - timestamps Vec<f64>: vector of f64s that timestamp the data you would like to save
// 
//  Outputs:
//      - json: returns a json object with fields corresponding to the triger time, the raw data and timestamsp, and mean, min, max of data
fn aquire_data(trig_time: String, streamed_data:Vec<f64>, timestamps:Vec<f64>) -> serde_json::Value<>{

    let mean_val = mean(&streamed_data);
    let min_val = min(&streamed_data);
    let max_val=max(&streamed_data);

    
    json!({"start_time":trig_time, "values":streamed_data, "timestamps":timestamps, "mean":mean_val, "min":min_val, "max":max_val})
}


//fn mean: &Vec<f64> -> f64: computes the mean of a vector
fn mean(data:&Vec<f64>) -> f64{
    let n_vals = data.len() as f64;
    let mut sum:f64 = 0.0;
    for n in data{
        sum += *n as f64;
    };
    sum/n_vals
}

//fn mean: &Vec<f64> -> f64: computes the max of a vector
fn max(data:&Vec<f64>) ->f64{
    let mut max:&f64 = &-10.0;
    for n in data{
        if n > max{
            max = n;
        };
    };
    *max
}

//fn mean: &Vec<f64> -> f64: computes the min of a vector
fn min(data:&Vec<f64>) ->f64{
    let mut min:&f64 = &11.0;
    for n in data{
        if n < min{
            min = n;
        };
    };
    *min
}

/*
fn rms(data:&Vec<f64>) -> f64{
    let n_vals = data.len() as f32;
    let mut sum:f64 = 0.0;
    for n in data{
        sum += f64::pow(*n as u32, 2) as f32;
    };
    (sum/n_vals).sqrt()
}
*/

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

fn parse_data(data:Vec<f64>, mut ch1_trace:Vec<f64>, mut ch2_trace:Vec<f64>, mut ch3_trace:Vec<f64>, expected_samples:u32) -> (Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>, Vec<f64>,Vec<f64>){
    let ch1:Vec<f64> = data.iter().skip(0).step_by(4).copied().collect();
    let ch2:Vec<f64> = data.iter().skip(1).step_by(4).copied().collect();
    let ch3:Vec<f64> = data.iter().skip(2).step_by(4).copied().collect();
    let trigger_line: Vec<f64> = data.iter().skip(3).step_by(4).copied().collect();
    let trig_holder = trigger_line.iter().position(|&x| x != 65533.0 as f64);
    if ch1_trace.len() as u32 == 0{
            if trig_holder != None{
                let trigger_position = trig_holder.unwrap() as u32;
                if trigger_position + expected_samples <= ch1.len() as u32{
                    for i in trigger_position..(trigger_position+expected_samples){
                        ch1_trace.push(100.0*ch1[i as usize]);
                        ch2_trace.push(100.0*ch2[i as usize]);
                        ch3_trace.push(100.0*ch3[i as usize]);
                    }
                }else{
                    for i in trigger_position as usize..ch1.len(){
                        ch1_trace.push(100.0*ch1[i]);
                        ch2_trace.push(100.0*ch2[i]);
                        ch3_trace.push(100.0*ch3[i]);
                    }
                };
            }
        }else {
            if expected_samples as usize - ch1_trace.len() <= ch1.len(){
                for i in 0 as usize..expected_samples as usize - ch1_trace.len(){
                    ch1_trace.push(100.0*ch1[i]);
                    ch2_trace.push(100.0*ch2[i]);
                    ch3_trace.push(100.0*ch3[i]);
                }
            }else{
                for i in 0 as usize..ch1.len(){
                    ch1_trace.push(100.0*ch1[i]);
                    ch2_trace.push(100.0*ch2[i]);
                    ch3_trace.push(100.0*ch3[i]);
                }
            }
        }
        return (ch1, ch2, ch3, ch1_trace, ch2_trace, ch3_trace)
}

async fn save_trace(pack:String, file_name:String){
    fs::write(file_name, pack).expect("Unable to write file!!");
}