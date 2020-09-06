use super::context::Context;
use super::timer;
use crate::fs::STDIN;
use crate::kernel::syscall_handler;
use crate::memory::*;
use crate::process::PROCESSOR;
use crate::sbi::console_getchar;
use riscv::register::{
    scause::{Exception, Interrupt, Scause, Trap},
    sie, stvec,
};
use crate::board::interrupt::{
    breakpoint,
    supervisor_timer,
    supervisor_soft,
    supervisor_external,
};

global_asm!(include_str!("./interrupt.asm"));

/// 初始化中断处理
///
/// 把中断入口 `__interrupt` 写入 `stvec` 中，并且开启中断使能
pub fn init() {
    unsafe {
        extern "C" {
            /// `interrupt.asm` 中的中断入口
            fn __interrupt();
        }
        // 使用 Direct 模式，将中断入口设置为 `__interrupt`
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);

        // 开启外部中断使能
        sie::set_sext();
        sie::set_ssoft();
    }
}

/// 中断的处理入口
///
/// `interrupt.asm` 首先保存寄存器至 Context，其作为参数和 scause 以及 stval 一并传入此函数
/// 具体的中断类型需要根据 scause 来推断，然后分别处理
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) -> *mut Context {
    // 首先检查线程是否已经结束（内核线程会自己设置标记来结束自己）
    {
        let mut processor = PROCESSOR.lock();
        let current_thread = processor.current_thread();
        if current_thread.as_ref().inner().dead {
            println!("thread {} exit", current_thread.id);
            processor.kill_current_thread();
            return processor.prepare_next_thread();
        }
    }
    // 根据中断类型来处理，返回的 Context 必须位于放在内核栈顶
    match scause.cause() {
        // 断点中断（ebreak）
        Trap::Exception(Exception::Breakpoint) => breakpoint(context),
        // 系统调用
        Trap::Exception(Exception::UserEnvCall) => syscall_handler(context),
        // 缺页异常
        Trap::Exception(Exception::LoadPageFault)
        | Trap::Exception(Exception::StorePageFault)
        | Trap::Exception(Exception::InstructionPageFault) => page_fault(context, scause, stval),
        // 时钟中断
        Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
        // 外部中断（键盘输入）
        Trap::Interrupt(Interrupt::SupervisorExternal) => supervisor_external(context),
        Trap::Interrupt(Interrupt::SupervisorSoft) => supervisor_soft(context),
        // 其他情况，无法处理
        _ => fault("unimplemented interrupt type", scause, stval),
    }
}

/// 处理缺页异常
///
/// todo: 理论上这里需要判断访问类型，并与页表中的标志位进行比对
fn page_fault(context: &mut Context, scause: Scause, stval: usize) -> *mut Context {
    static mut COUNT: usize = 0;
    println!("page_fault {}", unsafe {
        COUNT += 1;
        COUNT
    });
    let current_thread = PROCESSOR.lock().current_thread();
    let memory_set = &mut current_thread.process.inner().memory_set;

    match memory_set.mapping.handle_page_fault(stval) {
        Ok(_) => {
            memory_set.activate();
            context
        }
        Err(msg) => fault(msg, scause, stval),
    }
}

/// 出现未能解决的异常，终止当前线程
fn fault(msg: &str, scause: Scause, stval: usize) -> *mut Context {
    println!(
        "{:#x?} terminated: {}",
        PROCESSOR.lock().current_thread(),
        msg
    );
    println!("cause: {:?}, stval: {:x}", scause.cause(), stval);

    PROCESSOR.lock().kill_current_thread();
    // 跳转到 PROCESSOR 调度的下一个线程
    PROCESSOR.lock().prepare_next_thread()
}
