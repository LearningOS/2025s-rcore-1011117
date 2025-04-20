//! Process management syscalls

use crate::mm::{translated_byte_buffer, try_translated_byte_buffer, MapPermission};
use crate::task::{add_mmap, change_program_brk, count_syscall, exit_current_and_run_next, get_current_page, sub_mmap, suspend_current_and_run_next};
use crate::timer::get_time;

#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    pub sec: usize,
    pub usec: usize,
}

/// task exits and submit an exit code
pub fn sys_exit(_exit_code: i32) -> ! {
    trace!("kernel: sys_exit");
    exit_current_and_run_next();
    panic!("Unreachable in sys_exit!");
}

/// current task gives up resources for other tasks
pub fn sys_yield() -> isize {
    trace!("kernel: sys_yield");
    suspend_current_and_run_next();
    0
}

/// YOUR JOB: get time with second and microsecond
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TimeVal`] is splitted by two pages ?
pub fn sys_get_time(_ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    //get system time
    let time=get_time();
    //get user page
    let page=get_current_page();
    //Mapping user memory
    let mut data=translated_byte_buffer(page,_ts as *const u8,16);
    //As a pointer
    let ts =data[0].as_mut_ptr() as *mut TimeVal;
    //Copy from CH3
    unsafe {
        *ts = TimeVal {
            sec: time / 1_000_000,
            usec: time % 1_000_000,
        };
    }
    0
}

/// TODO: Finish sys_trace to pass testcases
/// HINT: You might reimplement it with virtual memory management.
pub fn sys_trace(_trace_request: usize, _id: usize, _data: usize) -> isize {
    trace!("kernel: sys_trace");
    //get user page
    let page=get_current_page();
    //Mapping user memory
    let mut data1;
    match _trace_request {
        0=>{
            if let Some(data)=try_translated_byte_buffer(page,_id as *const u8,1,MapPermission::U|MapPermission::R){
                data1=data;
            }
            else{
                return -1;
            };
            let _id =data1[0].as_mut_ptr();
            unsafe{(*_id)  as isize}
        }
        1=>{
            if let Some(data)=try_translated_byte_buffer(page,_id as *const u8,1,MapPermission::R|MapPermission::W|MapPermission::U){
                data1=data;
            }
            else{
                return -1;
            };
            data1[0][0]=_data as u8;
            let _id =data1[0].as_mut_ptr();
            0
        }
        2=> {
            count_syscall(_id) as isize
        }
        _=>-1
    }
}
// YOUR JOB: Implement mmap.
pub fn sys_mmap(_start: usize, _len: usize, _port: usize) -> isize {
    trace!("kernel: sys_mmap NOT IMPLEMENTED YET!");
    if _port>7 || _port == 0 {
        return -1;
    }
    if (_start<<(usize::BITS-12))!=0 {
        return -1;
    }
    let _port=_port<<1;
    let mp=MapPermission::from_bits_truncate(_port as u8);
    let mp=mp.union(MapPermission::U);
    if add_mmap(_start,_len,mp){
        return 0;
    }
    -1
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(_start: usize, _len: usize) -> isize {
    trace!("kernel: sys_munmap NOT IMPLEMENTED YET!");
    if sub_mmap(_start,_len){
        return 0;
    }
    -1
}
/// change data segment size
pub fn sys_sbrk(size: i32) -> isize {
    trace!("kernel: sys_sbrk");
    if let Some(old_brk) = change_program_brk(size) {
        old_brk as isize
    } else {
        -1
    }
}
