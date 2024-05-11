//! Process management syscalls

use crate::{
    config::MAX_SYSCALL_NUM, mm::{translated_taskinfo, translated_timeval}, task::{
        change_program_brk, curren_task_start_time,current_user_token, exit_current_and_run_next, mmap, suspend_current_and_run_next, syscall_times, unmap, TaskStatus
    }, timer::{get_time_ms, get_time_us}
};

///
#[repr(C)]
#[derive(Debug)]
pub struct TimeVal {
    ///
    pub sec: usize,
    ///
    pub usec: usize,
}

/// Task information
#[allow(dead_code)]
pub struct TaskInfo {
    /// Task status in it's life cycle
    status: TaskStatus,
    /// The numbers of syscall called by task
    syscall_times: [u32; MAX_SYSCALL_NUM],
    /// Total running time of task
    time: usize,
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
pub fn sys_get_time(ts: *mut TimeVal, _tz: usize) -> isize {
    trace!("kernel: sys_get_time");
    let us = get_time_us();
    trace!("kernel: current_time is {}",us);
    let mut t = 0;
    trace!("time: {}",t);
    unsafe {*translated_timeval(current_user_token(), ts) = TimeVal{
        sec: us / 1_000_000,
        usec:us % 1_000_000,
    };
    t = (*translated_timeval(current_user_token(), ts)).usec;
    }
    trace!("time: {}",t);
    0
}

/// YOUR JOB: Finish sys_task_info to pass testcases
/// HINT: You might reimplement it with virtual memory management.
/// HINT: What if [`TaskInfo`] is splitted by two pages ?
pub fn sys_task_info(ti: *mut TaskInfo) -> isize {
    trace!("kernel: sys_task_info");
    unsafe{
        (*translated_taskinfo(current_user_token(), ti)).status = TaskStatus::Running;
        (*translated_taskinfo(current_user_token(), ti)).syscall_times = syscall_times();
        (*translated_taskinfo(current_user_token(), ti)).time = get_time_ms() - curren_task_start_time();
    }
    trace!("kernel: sys_task_info is ok!");
    0
}

// YOUR JOB: Implement mmap.
pub fn sys_mmap(start: usize, len: usize, port: usize) -> isize {
    //println!("sys_mmap");
    trace!("kernel: sys_mmap");
    mmap(start, len, port)
}

// YOUR JOB: Implement munmap.
pub fn sys_munmap(start: usize, len: usize) -> isize {
    //println!("sys ummap");
    trace!("kernel: sys_munmap");
    unmap(start, len)
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
