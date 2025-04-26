//!Implementation of [`TaskManager`]

use super::TaskControlBlock;
use crate::sync::UPSafeCell;
use alloc::collections::{BinaryHeap};
use alloc::sync::Arc;
use core::cmp::Reverse;
use lazy_static::*;
///A array of `TaskControlBlock` that is thread-safe
pub struct TaskManager {
    ready_queue: BinaryHeap<Reverse<Arc<TaskControlBlock>>>,
}
/// A simple FIFO scheduler.
impl TaskManager {
    ///Creat an empty TaskManager
    pub fn new() -> Self {
        Self {
            ready_queue: BinaryHeap::new(),
        }
    }
    /// Add process back to ready queue
    pub fn add(&mut self, task: Arc<TaskControlBlock>) {
        self.ready_queue.push(Reverse(task));
    }
    /// Take a process out of the ready queue
    pub fn fetch(&mut self) -> Option<Arc<TaskControlBlock>> {
        match self.ready_queue.pop(){
            Some(Reverse(task)) => {let mut inner=task.inner_exclusive_access();inner.stride+=inner.task_priority;Some(task.clone())},
            None => None
        }
    }
}

lazy_static! {
    /// TASK_MANAGER instance through lazy_static!
    pub static ref TASK_MANAGER: UPSafeCell<TaskManager> =
        unsafe { UPSafeCell::new(TaskManager::new()) };
}

/// Add process to ready queue
pub fn add_task(task: Arc<TaskControlBlock>) {
    //trace!("kernel: TaskManager::add_task");
    let mut task_manager=TASK_MANAGER.exclusive_access();
    if task_manager.ready_queue.iter().find_map(|a|{
        if a.0.pid.0==task.pid.0{
            Some(a.clone())
        }
        else { None }

    }).is_none(){
        task_manager.add(task);
    }
}
/// Take a process out of the ready queue
#[deny(dead_code)]
pub fn fetch_task() -> Option<Arc<TaskControlBlock>> {
    //trace!("kernel: TaskManager::fetch_task");
    TASK_MANAGER.exclusive_access().fetch()
}