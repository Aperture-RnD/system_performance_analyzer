use sysinfo;
use sysinfo::CpuExt;
use nvml_wrapper::Nvml;
use std::{io, thread::{self}, time::Duration};
use std::io::prelude::*;

use sysinfo::{System, SystemExt};

fn main() {
    //STARTUP
    let mut sys = System::new_all();
    sys.refresh_all();
    sys.refresh_cpu();
    println!("Enter how long you want to monitor the system:");
    let mut input_line = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut input_line).ok();
    let duration: i64 = input_line.trim().parse().expect("Input not an integer");
    println!("Starting! Monitoring system for chosen duration: {}", input_line);
    let one_second = Duration::new(1, 0);
    //END STARTUP

    //USED LATER
    fn largest_in_array(input_array : &Vec<f64>) -> f64 { 
        let mut large : f64 = 0.0;
        let mut i : usize = 0;

        while i<input_array.len() {
            if input_array[i] > large {
                large = input_array[i]
            }
            i +=1
        }
        return large;
    }

    fn pause() {
        let mut pause_stdin = io::stdin();
        let mut pause_stdout = io::stdout();

        write!(pause_stdout, "Press enter to continue...").unwrap();
        pause_stdout.flush().unwrap();
        let _ = pause_stdin.read(&mut [0u8]).unwrap();
    }
    //USED LATER 

    sys.refresh_all();
    sys.refresh_cpu();
    thread::sleep(one_second);
    sys.refresh_cpu();

    //MONITOR START
    let mut mem_monitor_ref : Vec<f64> = Vec::new();
    sys.refresh_cpu();

    let memory_monitor = | _i64 | {
        let mut i : i64 = 0;
        let mut internal_cpu_ref = Vec::new();
        let mut internal_gpu_vram_ref = Vec::new();
        let mut internal_gpu_use_ref = Vec::new();
        let mut internal_gpu_encoder_ref = Vec::new();
        
        sys.refresh_cpu();

        //MONITOR LOOP START
        while duration > i {

        //CPU MONITOR
        sys.refresh_cpu();
        for cpu in sys.cpus() {
            internal_cpu_ref.push(cpu.cpu_usage() as f64);
        }
        //CPU MONITOR END

        //RAM MONITOR START 
        sys.refresh_all();
        let kbtotal_mem = sys.total_memory() / 1024;
        let kbused_mem = sys.used_memory() / 1024;
        let mbtotal_mem = kbtotal_mem / 1024; 
        let mbused_mem = kbused_mem / 1024;
        let used_mem_percent = (mbused_mem as f64 / mbtotal_mem as f64) * 100 as f64;
        mem_monitor_ref.push(used_mem_percent); 
        //RAM MONITOR END
        
        //GPU MONITOR START
        let nvml = Nvml::init();
        let binding = nvml.unwrap();
        let device = binding.device_by_index(0).unwrap();
        let encoder_util = device.encoder_utilization().unwrap().utilization; 
        let binded_use_rate = device.utilization_rates();
        let unwrap_me = binded_use_rate.unwrap();
        let gpu_use_rate = unwrap_me.gpu;
        let vram_use_rate = unwrap_me.memory;
    
        internal_gpu_encoder_ref.push(encoder_util as f64);
        internal_gpu_use_ref.push(gpu_use_rate as f64);
        internal_gpu_vram_ref.push(vram_use_rate as f64);
        //GPU MONITOR ENDS

        //management
        thread::sleep(one_second);
        println!("MONITORING: {:?}", i +1);
        i +=1;
        }
        //MONITOR LOOP ENDS
    return (mem_monitor_ref, internal_cpu_ref, internal_gpu_encoder_ref, internal_gpu_use_ref, internal_gpu_vram_ref)
    };

    let monitor_output = memory_monitor(duration);
    let cpu_monitor_output = monitor_output.1.clone();
    let cpu_monitor_output2 = monitor_output.1.clone();

    //MONITOR END

    //ANALYSIS START
    let cpu_analyzer = | input_cpu_vec : Vec<f64> | {
        let num_cpus = sys.cpus().len();

        let mut ii : usize = 0;
        let mut cpu_use_good_bool: bool = true; 

        for value in input_cpu_vec {
            let current_second = ii as i32 / num_cpus as i32;
            let cpu_core = if ii > 23 {
                ii as i32 - (num_cpus as i32 * current_second)
            } 
            else 
            { 
                ii as i32
            };

            if value > 90 as f64 {
            println!("ATTENTION: CPU{:?} is running at {:?}% on second {:?}!!!", cpu_core, value, current_second);
            cpu_use_good_bool = false;
            }

            ii +=1;
        }
        if cpu_use_good_bool {
            println!("CPU has stayed below 90%, not the issue.")
        }
    };

    let _test_array = [85.123411, 92.2331, 98.2231, 99.2212, 60.00123];
    let _cpu_analyzer_output = cpu_analyzer(cpu_monitor_output);
    let peak_cpu = largest_in_array(&cpu_monitor_output2);
    let peak_ram = largest_in_array(&monitor_output.0);
    let peak_encoder = largest_in_array(&monitor_output.2);
    let peak_gpu = largest_in_array(&monitor_output.3);
    let peak_vram = largest_in_array(&monitor_output.4);
    
    println!("Peak CPU use was {:?}%", peak_cpu);

    let mut iii : i32 = 0; 
    for value in &monitor_output.0 {
        let limit: &f64 = &90.0;
        if value > limit {
            println!("ATTTENTION: RAM usage is {:?} during second {:?}!!!", value, iii)
        }
        iii +=1;
    } 
    println!("Peak RAM use was {:?}%", peak_ram);

    iii = 0;
    for value in &monitor_output.3 {
        let limit: &f64 = &90.0;
        if value > limit {
            println!("ATTTENTION: GPU usage is {:?} during second {:?}!!!", value, iii)
        }
        iii +=1;
    } 
    println!("Peak GPU use was {:?}%", peak_gpu);

    iii = 0;
    for value in &monitor_output.4 {
        let limit: &f64 = &90.0;
        if value > limit {
            println!("ATTTENTION: GPU VRAM usage is {:?} during second {:?}!!!", value, iii)
        }
        iii +=1;
    } 
    println!("Peak GPU VRAM use was {:?}%", peak_vram);

    iii = 0;
    for value in &monitor_output.2 {
        let limit: &f64 = &90.0;
        if value > limit {
            println!("ATTTENTION: GPU Encoder usage is {:?} during second {:?}!!!", value, iii)
        }
        iii +=1;
    } 
    println!("Peak GPU Encoder use was {:?}%", peak_encoder);
    pause();
}