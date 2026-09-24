#![no_std]
//! Mali CSF Cortex-M7 reconstruction — Stage 5.
pub mod volatile;
pub const VECTOR_SYSTICK:u32=0x0080_0015;pub const VECTOR_RESET:u32=0x0080_1FBD;
pub const DATA_SRC_START:usize=0x0100_0000;pub const DATA_DST_START:usize=0x0200_0000;pub const DATA_DST_END:usize=0x0202_B7E0;pub const BSS_END:usize=0x0202_DF18;pub const PSP_TOP:usize=0x0202_F000;
pub const DATA_BYTES:usize=DATA_DST_END-DATA_DST_START;pub const BSS_BYTES:usize=BSS_END-DATA_DST_END;
pub const SCB_ICSR:usize=0xE000_ED04;pub const SCB_CCR:usize=0xE000_ED14;pub const SCB_SHCSR:usize=0xE000_ED24;pub const SCB_CFSR:usize=0xE000_ED28;pub const SCB_MMFAR:usize=0xE000_ED34;pub const SCB_BFAR:usize=0xE000_ED38;pub const MPU_CTRL:usize=0xE000_ED94;pub const MPU_RBAR:usize=0xE000_ED9C;pub const MPU_RASR:usize=0xE000_EDA0;
#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]pub struct FaultFlags{pub iaccviol:bool,pub daccviol:bool,pub munstkerr:bool,pub mstkerr:bool,pub mlsp_err:bool,pub ibuserr:bool,pub preciserr:bool,pub unstkerr:bool,pub stkerr:bool,pub lsp_err:bool,pub undefinstr:bool,pub invstate:bool,pub invpc:bool,pub nocp:bool,pub unaligned:bool,pub divbyzero:bool}
pub const fn decode_cfsr(v:u32)->FaultFlags{FaultFlags{iaccviol:v&1!=0,daccviol:v&2!=0,munstkerr:v&8!=0,mstkerr:v&0x10!=0,mlsp_err:v&0x20!=0,ibuserr:v&0x100!=0,preciserr:v&0x200!=0,unstkerr:v&0x800!=0,stkerr:v&0x1000!=0,lsp_err:v&0x2000!=0,undefinstr:v&0x1_0000!=0,invstate:v&0x2_0000!=0,invpc:v&0x4_0000!=0,nocp:v&0x8_0000!=0,unaligned:v&0x100_0000!=0,divbyzero:v&0x200_0000!=0}}
pub unsafe fn enable_caches_bits(){use volatile::{read32,write32};let c=unsafe{read32(SCB_CCR)};unsafe{write32(SCB_CCR,c|0x0003_0000)}}
pub unsafe fn relocate_and_zero(src:*const u32,dst:*mut u32,init_words:usize,total_words:usize){for i in 0..init_words{unsafe{core::ptr::write_volatile(dst.add(i),core::ptr::read_volatile(src.add(i)))}}for i in init_words..total_words{unsafe{core::ptr::write_volatile(dst.add(i),0)}}}
#[cfg(test)] extern crate std;#[cfg(test)]mod tests{use super::*;#[test]fn boot_layout(){assert_eq!(DATA_BYTES,0x2b7e0);assert_eq!(BSS_BYTES,0x2738);assert_eq!(PSP_TOP-DATA_DST_START,0x2f000);}#[test]fn cfsr(){let f=decode_cfsr(0x0202_0202);assert!(f.daccviol&&f.preciserr&&f.divbyzero);}}

/// Stage 6 memory classification derived from the CSF container map and reset path.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum FirmwareAddressClass{Code,InitializedData,RuntimeSram,SystemControl,Other}
pub const fn classify_address(a:usize)->FirmwareAddressClass{
    if a>=0x0080_0000&&a<0x0081_5000 { FirmwareAddressClass::Code }
    else if a>=0x0100_0000&&a<0x0103_0000 { FirmwareAddressClass::InitializedData }
    else if a>=0x0200_0000&&a<PSP_TOP { FirmwareAddressClass::RuntimeSram }
    else if a>=0xE000_E000&&a<0xE001_0000 { FirmwareAddressClass::SystemControl }
    else { FirmwareAddressClass::Other }
}
#[repr(C)]#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]pub struct CortexMExceptionFrame{pub r0:u32,pub r1:u32,pub r2:u32,pub r3:u32,pub r12:u32,pub lr:u32,pub pc:u32,pub xpsr:u32}
#[cfg(test)]mod stage6_tests{use super::*;use core::mem::size_of;#[test]fn classify(){assert_eq!(classify_address(VECTOR_RESET as usize),FirmwareAddressClass::Code);assert_eq!(classify_address(DATA_DST_START),FirmwareAddressClass::RuntimeSram);assert_eq!(classify_address(SCB_CFSR),FirmwareAddressClass::SystemControl);assert_eq!(size_of::<CortexMExceptionFrame>(),32);}}

/// Stage 7: hottest runtime SRAM globals by static reference count; semantics deliberately unnamed.
pub const STAGE7_HOT_GLOBALS:&[(usize,u32)]=&[
(0x2000000,235),
(0x202BE7C,96),
(0x202B950,95),
(0x202B948,87),
(0x202B945,86),
(0x202B258,69),
(0x202B508,68),
(0x202BE80,67),
(0x2006B40,64),
(0x202B7EC,61),
(0x202DF04,50),
(0x202DECE,46),
(0x202DED8,38),
(0x202DF0E,36),
(0x202BE84,35),
(0x202BE64,35),
(0x202B7F8,34),
(0x202B7F0,32),
(0x202DEC8,30),
(0x202DEAC,27),
(0x202BE68,27),
(0x202B818,25),
(0x202B509,24),
(0x202BE60,24),
(0x202B718,24),
(0x202B7E8,23),
(0x202B7F4,23),
(0x202B810,22),
(0x202B668,21),
(0x202BFF0,21),
(0x202B228,19),
(0x202B188,18),
(0x202DF00,18),
(0x202B25C,17),
(0x202BE6C,17),
(0x202B5B8,17),
(0x202BE70,15),
(0x202DF0C,15),
(0x202B259,15),
(0x202DF06,15),
(0x202B7E0,14),
(0x202C084,14),
(0x202B25A,14),
(0x202DEDC,14),
(0x202DEC4,13),
(0x202DEC0,13),
(0x202B150,13),
(0x202BE78,13)];

/// Stage 10: typed view of the exact Cortex-M fault registers consumed by
/// `csffw_fault_dump_and_halt@0x802e44` and its three decoder helpers.
pub const SCB_HFSR:usize=0xE000_ED2C;
pub const CFSR_MMARVALID:u32=0x0000_0080;
pub const CFSR_BFARVALID:u32=0x0000_8000;
pub const HFSR_VECTTBL:u32=0x0000_0002;
pub const HFSR_FORCED:u32=0x4000_0000;
pub const HFSR_DEBUGEVT:u32=0x8000_0000;

#[derive(Clone,Copy,Debug,Default,PartialEq,Eq)]
pub struct FaultSnapshot{pub cfsr:u32,pub hfsr:u32,pub mmfar:u32,pub bfar:u32}
impl FaultSnapshot{
    pub const fn mmfar_valid(self)->bool{self.cfsr&CFSR_MMARVALID!=0}
    pub const fn bfar_valid(self)->bool{self.cfsr&CFSR_BFARVALID!=0}
    pub const fn forced_hardfault(self)->bool{self.hfsr&HFSR_FORCED!=0}
    pub const fn vector_table_fault(self)->bool{self.hfsr&HFSR_VECTTBL!=0}
    pub const fn debug_event(self)->bool{self.hfsr&HFSR_DEBUGEVT!=0}
    pub const fn flags(self)->FaultFlags{decode_cfsr(self.cfsr)}
}
#[cfg(test)]mod stage10_tests{use super::*;#[test]fn snapshot(){let s=FaultSnapshot{cfsr:CFSR_MMARVALID|CFSR_BFARVALID|2,hfsr:HFSR_FORCED,mmfar:1,bfar:2};assert!(s.mmfar_valid()&&s.bfar_valid()&&s.forced_hardfault());assert!(s.flags().daccviol);}}

/// Stage 11: exact MPU region-0 programming values from `configure_mpu_region0@0x811d12`.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct MpuRegion0Program{pub rbar:u32,pub rasr:u32,pub ctrl:u32}
pub const fn mpu_region0_program(alternate:bool)->MpuRegion0Program{
    if alternate { MpuRegion0Program{rbar:0x0300_001B,rasr:0x150B_0023,ctrl:3} }
    else { MpuRegion0Program{rbar:0xE000_0019,rasr:0x1110_0027,ctrl:3} }
}
pub trait MpuRegisterIo{fn write32(&mut self,addr:usize,value:u32);fn dsb(&mut self);fn isb(&mut self);}
pub fn apply_mpu_region0<I:MpuRegisterIo>(io:&mut I,alternate:bool){let p=mpu_region0_program(alternate);io.write32(MPU_RBAR,p.rbar);io.write32(MPU_RASR,p.rasr);io.write32(MPU_CTRL,p.ctrl);io.dsb();io.isb();}

/// Pure transition extracted from `systick_scheduler_tick@0x8130c2`.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum SchedulerTickAction{None,RunScheduler,PendContextSwitch{current:u32}}
pub const fn scheduler_tick(counter:u8,current:u32,last:u32)->(u8,SchedulerTickAction){
    let next=counter.wrapping_add(1);
    if next!=3{return(next,SchedulerTickAction::None)}
    if current==last{(3,SchedulerTickAction::RunScheduler)}else{(0,SchedulerTickAction::PendContextSwitch{current})}
}
#[cfg(test)]mod stage11_tests{use super::*;#[test]fn mpu(){assert_eq!(mpu_region0_program(false).rbar,0xE0000019);assert_eq!(mpu_region0_program(true).rasr,0x150B0023);}#[test]fn tick(){assert_eq!(scheduler_tick(1,9,8),(2,SchedulerTickAction::None));assert_eq!(scheduler_tick(2,9,8),(0,SchedulerTickAction::PendContextSwitch{current:9}));assert_eq!(scheduler_tick(2,8,8),(3,SchedulerTickAction::RunScheduler));}}

/// Stage 12: early Cortex-M7 reset register program, excluding the still-unresolved
/// auxiliary-control helper and the conditional data/BSS relocation.
pub const FW_TRACE_BOOT0:usize=0x4002_2000;pub const FW_TRACE_BOOT1:usize=0x4002_2004;pub const SCB_SCR:usize=0xE000_ED10;
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct CortexMResetPlan{pub trace0:u32,pub trace1:u32,pub shcsr:u32,pub scr:u32,pub psp:usize,pub control:u32,pub mpu:MpuRegion0Program,pub ccr_set_bits:u32}
pub const fn cortex_m_reset_plan(alternate:bool)->CortexMResetPlan{CortexMResetPlan{trace0:11,trace1:7,shcsr:0x0007_0000,scr:2,psp:PSP_TOP,control:2,mpu:mpu_region0_program(alternate),ccr_set_bits:0x0003_0000}}
pub trait CortexMResetIo{fn disable_irq(&mut self);fn read32(&mut self,addr:usize)->u32;fn write32(&mut self,addr:usize,value:u32);fn set_psp(&mut self,value:usize);fn set_control(&mut self,value:u32);fn dsb(&mut self);fn isb(&mut self);}
pub fn apply_cortex_m_reset_registers<I:CortexMResetIo>(io:&mut I,alternate:bool){let p=cortex_m_reset_plan(alternate);io.disable_irq();io.write32(FW_TRACE_BOOT0,p.trace0);io.write32(FW_TRACE_BOOT1,p.trace1);io.write32(SCB_SHCSR,p.shcsr);io.write32(SCB_SCR,p.scr);io.set_psp(p.psp);io.set_control(p.control);io.isb();io.write32(MPU_RBAR,p.mpu.rbar);io.write32(MPU_RASR,p.mpu.rasr);io.write32(MPU_CTRL,p.mpu.ctrl);io.dsb();io.isb();let c=io.read32(SCB_CCR);io.dsb();io.isb();io.write32(SCB_CCR,c|0x0002_0000);io.dsb();io.isb();io.dsb();io.write32(SCB_CCR,(c|0x0002_0000)|0x0001_0000);io.dsb();io.isb();}
#[cfg(test)]mod stage12_tests{use super::*;struct M{ccr:u32,psp:usize,ctl:u32,w:u32}impl CortexMResetIo for M{fn disable_irq(&mut self){}fn read32(&mut self,a:usize)->u32{if a==SCB_CCR{self.ccr}else{0}}fn write32(&mut self,a:usize,v:u32){self.w+=1;if a==SCB_CCR{self.ccr=v}}fn set_psp(&mut self,v:usize){self.psp=v}fn set_control(&mut self,v:u32){self.ctl=v}fn dsb(&mut self){}fn isb(&mut self){}}#[test]fn reset(){let mut m=M{ccr:0,psp:0,ctl:0,w:0};apply_cortex_m_reset_registers(&mut m,false);assert_eq!(m.psp,PSP_TOP);assert_eq!(m.ctl,2);assert_eq!(m.ccr&0x30000,0x30000);assert!(m.w>=8);}}

/// Stage 13: reset-handler initialized-data relocation and BSS clearing.
pub const RESET_DATA_SOURCE:usize=0x0100_0000;pub const RESET_DATA_DEST:usize=0x0200_0000;pub const RESET_DATA_END:usize=0x0202_B7E0;pub const RESET_BSS_END:usize=0x0202_DF18;pub const RESET_RELOCATE_FLAG:usize=0x0302_B250;pub const RESET_CONTINUATION:usize=0x0080_3718;
pub const RESET_DATA_WORDS:usize=(RESET_DATA_END-RESET_DATA_DEST)/4;pub const RESET_BSS_WORDS:usize=(RESET_BSS_END-RESET_DATA_END)/4;
pub trait ResetMemoryIo{fn read32_reset(&mut self,addr:usize)->u32;fn read8_reset(&mut self,addr:usize)->u8;fn copy_words_reset(&mut self,src:usize,dst:usize,words:usize);fn zero_words_reset(&mut self,dst:usize,words:usize);}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct ResetMemoryResult{pub alternate_mpu:bool,pub relocated:bool,pub continuation:usize}
pub fn apply_reset_memory_phase<I:ResetMemoryIo>(io:&mut I)->ResetMemoryResult{let alternate=io.read32_reset(0x4000_3034)&0x80!=0;let relocate=alternate&&io.read8_reset(RESET_RELOCATE_FLAG)!=0;if relocate{io.copy_words_reset(RESET_DATA_SOURCE,RESET_DATA_DEST,RESET_DATA_WORDS);io.zero_words_reset(RESET_DATA_END,RESET_BSS_WORDS);}ResetMemoryResult{alternate_mpu:alternate,relocated:relocate,continuation:RESET_CONTINUATION}}
pub const fn systick_dispatches_scheduler(exc_return:u32)->bool{exc_return&4==0}
#[cfg(test)]mod stage13_tests{use super::*;struct M{alt:bool,flag:u8,c:usize,z:usize}impl ResetMemoryIo for M{fn read32_reset(&mut self,_:usize)->u32{if self.alt{0x80}else{0}}fn read8_reset(&mut self,_:usize)->u8{self.flag}fn copy_words_reset(&mut self,s:usize,d:usize,w:usize){assert_eq!((s,d),(RESET_DATA_SOURCE,RESET_DATA_DEST));self.c=w}fn zero_words_reset(&mut self,d:usize,w:usize){assert_eq!(d,RESET_DATA_END);self.z=w}}#[test]fn relocate(){let mut m=M{alt:true,flag:1,c:0,z:0};let r=apply_reset_memory_phase(&mut m);assert!(r.relocated);assert_eq!(m.c,RESET_DATA_WORDS);assert_eq!(m.z,RESET_BSS_WORDS);assert_eq!(r.continuation,RESET_CONTINUATION);assert!(systick_dispatches_scheduler(0));assert!(!systick_dispatches_scheduler(4));}}

/// Stage 14: composition of the exact reset-register phase and the conditional
/// data/BSS phase observed in `reset_handler@0x801fbc`.
pub trait MaliResetRuntime:CortexMResetIo+ResetMemoryIo{}
impl<T:CortexMResetIo+ResetMemoryIo> MaliResetRuntime for T{}
pub fn run_reset_until_continuation<I:MaliResetRuntime>(io:&mut I)->ResetMemoryResult{
    let alternate=io.read32_reset(0x4000_3034)&0x80!=0;
    apply_cortex_m_reset_registers(io,alternate);
    apply_reset_memory_phase(io)
}
#[cfg(test)]mod stage14_tests{use super::*;struct M{ccr:u32,flag:u8,c:usize,z:usize}impl CortexMResetIo for M{fn disable_irq(&mut self){}fn read32(&mut self,a:usize)->u32{if a==SCB_CCR{self.ccr}else{0}}fn write32(&mut self,a:usize,v:u32){if a==SCB_CCR{self.ccr=v}}fn set_psp(&mut self,_:usize){}fn set_control(&mut self,_:u32){}fn dsb(&mut self){}fn isb(&mut self){}}impl ResetMemoryIo for M{fn read32_reset(&mut self,a:usize)->u32{if a==0x40003034{0x80}else{0}}fn read8_reset(&mut self,_:usize)->u8{self.flag}fn copy_words_reset(&mut self,_:usize,_:usize,w:usize){self.c=w}fn zero_words_reset(&mut self,_:usize,w:usize){self.z=w}}#[test]fn full_reset_prefix(){let mut m=M{ccr:0,flag:1,c:0,z:0};let r=run_reset_until_continuation(&mut m);assert!(r.alternate_mpu&&r.relocated);assert_eq!(r.continuation,RESET_CONTINUATION);assert_eq!((m.c,m.z),(RESET_DATA_WORDS,RESET_BSS_WORDS));assert_eq!(m.ccr&0x30000,0x30000);}}

/// Stage 15: reset-to-runtime handoff model. The reset prefix is binary-derived;
/// the scheduler decision is the previously recovered SysTick state transition.
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub struct MaliRuntimeHandoff{pub reset:ResetMemoryResult,pub next_tick_counter:u8,pub tick_action:SchedulerTickAction}
pub fn reset_and_prepare_runtime<I:MaliResetRuntime>(io:&mut I,tick_counter:u8,current_task:u32,last_task:u32)->MaliRuntimeHandoff{let reset=run_reset_until_continuation(io);let(next_tick_counter,tick_action)=scheduler_tick(tick_counter,current_task,last_task);MaliRuntimeHandoff{reset,next_tick_counter,tick_action}}
#[cfg(test)]mod stage15_tests{use super::*;struct M{ccr:u32,c:usize,z:usize}impl CortexMResetIo for M{fn disable_irq(&mut self){}fn read32(&mut self,a:usize)->u32{if a==SCB_CCR{self.ccr}else{0}}fn write32(&mut self,a:usize,v:u32){if a==SCB_CCR{self.ccr=v}}fn set_psp(&mut self,_:usize){}fn set_control(&mut self,_:u32){}fn dsb(&mut self){}fn isb(&mut self){}}impl ResetMemoryIo for M{fn read32_reset(&mut self,a:usize)->u32{if a==0x4000_3034{0x80}else{0}}fn read8_reset(&mut self,_:usize)->u8{1}fn copy_words_reset(&mut self,_:usize,_:usize,w:usize){self.c=w}fn zero_words_reset(&mut self,_:usize,w:usize){self.z=w}}#[test]fn handoff(){let mut m=M{ccr:0,c:0,z:0};let h=reset_and_prepare_runtime(&mut m,2,9,8);assert!(h.reset.relocated);assert_eq!(h.next_tick_counter,0);assert_eq!(h.tick_action,SchedulerTickAction::PendContextSwitch{current:9});}}

/// Stage 16: exact SysTick scheduler-side MMIO transition recovered from
/// `systick_scheduler_tick@0x8130c2`. The same-task scheduler body remains an
/// explicit callback until its callee chain is independently reconstructed.
pub const SCHED_LAST_TASK_ADDR:usize=0x0202_DED0;
pub const SCHED_CURRENT_TASK_PTR_ADDR:usize=0x0202_DED4;
pub const SCHED_TICK_COUNTER_ADDR:usize=0x0202_DED8;
pub const SCHED_ICSR_OR_MASK:u32=0x0200_0000;
pub trait SchedulerTickRuntimeIo{fn read32_sched(&mut self,addr:usize)->u32;fn write32_sched(&mut self,addr:usize,value:u32);fn run_same_task_scheduler(&mut self);}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum ExactSchedulerTickOutcome{NoDispatch{counter:u32},SameTaskScheduler{current:u32},ContextChange{current:u32}}
pub fn run_scheduler_tick_exact<I:SchedulerTickRuntimeIo>(io:&mut I)->ExactSchedulerTickOutcome{
    let next=io.read32_sched(SCHED_TICK_COUNTER_ADDR).wrapping_add(1);io.write32_sched(SCHED_TICK_COUNTER_ADDR,next);
    if next!=3{return ExactSchedulerTickOutcome::NoDispatch{counter:next}}
    let current_ptr=io.read32_sched(SCHED_CURRENT_TASK_PTR_ADDR) as usize;let current=io.read32_sched(current_ptr);let last=io.read32_sched(SCHED_LAST_TASK_ADDR);
    if current==last{io.run_same_task_scheduler();ExactSchedulerTickOutcome::SameTaskScheduler{current}}
    else{let icsr=io.read32_sched(SCB_ICSR);io.write32_sched(SCB_ICSR,icsr|SCHED_ICSR_OR_MASK);io.write32_sched(SCHED_LAST_TASK_ADDR,current);io.write32_sched(SCHED_TICK_COUNTER_ADDR,0);ExactSchedulerTickOutcome::ContextChange{current}}
}
#[cfg(test)]mod stage16_tests{use super::*;struct M{tick:u32,last:u32,current:u32,icsr:u32,same:u8}impl SchedulerTickRuntimeIo for M{fn read32_sched(&mut self,a:usize)->u32{match a{SCHED_TICK_COUNTER_ADDR=>self.tick,SCHED_LAST_TASK_ADDR=>self.last,SCHED_CURRENT_TASK_PTR_ADDR=>0x2000,0x2000=>self.current,SCB_ICSR=>self.icsr,_=>0}}fn write32_sched(&mut self,a:usize,v:u32){match a{SCHED_TICK_COUNTER_ADDR=>self.tick=v,SCHED_LAST_TASK_ADDR=>self.last=v,SCB_ICSR=>self.icsr=v,_=>{}}}fn run_same_task_scheduler(&mut self){self.same+=1}}#[test]fn context_change(){let mut m=M{tick:2,last:7,current:9,icsr:1,same:0};assert_eq!(run_scheduler_tick_exact(&mut m),ExactSchedulerTickOutcome::ContextChange{current:9});assert_eq!(m.tick,0);assert_eq!(m.last,9);assert_eq!(m.icsr,1|SCHED_ICSR_OR_MASK);}#[test]fn same_task(){let mut m=M{tick:2,last:9,current:9,icsr:0,same:0};assert_eq!(run_scheduler_tick_exact(&mut m),ExactSchedulerTickOutcome::SameTaskScheduler{current:9});assert_eq!(m.same,1);assert_eq!(m.tick,3);}}

/// Stage 17: correct the Stage-16 provisional same-task interpretation.
/// The branch at 0x8130c2 calls 0x8036ac, prints this binary-exact diagnostic,
/// then enters the recovered fault-dump/halt path rather than normal scheduling.
pub const SCHED_HANG_DIAGNOSTIC:&[u8]=b"Firmware hang detected at PC 0x%08x\n\0";
pub trait SchedulerTickMmio{fn read32_tick(&mut self,addr:usize)->u32;fn write32_tick(&mut self,addr:usize,value:u32);}
#[derive(Clone,Copy,Debug,PartialEq,Eq)]pub enum ReferenceSchedulerTickOutcome{NoDispatch{counter:u32},FirmwareHang{current:u32},ContextChange{current:u32}}
pub fn run_scheduler_tick_reference<I:SchedulerTickMmio>(io:&mut I)->ReferenceSchedulerTickOutcome{
    let next=io.read32_tick(SCHED_TICK_COUNTER_ADDR).wrapping_add(1);io.write32_tick(SCHED_TICK_COUNTER_ADDR,next);if next!=3{return ReferenceSchedulerTickOutcome::NoDispatch{counter:next}}
    let current_ptr=io.read32_tick(SCHED_CURRENT_TASK_PTR_ADDR)as usize;let current=io.read32_tick(current_ptr);let last=io.read32_tick(SCHED_LAST_TASK_ADDR);
    if current==last{return ReferenceSchedulerTickOutcome::FirmwareHang{current}}
    let icsr=io.read32_tick(SCB_ICSR);io.write32_tick(SCB_ICSR,icsr|SCHED_ICSR_OR_MASK);io.write32_tick(SCHED_LAST_TASK_ADDR,current);io.write32_tick(SCHED_TICK_COUNTER_ADDR,0);ReferenceSchedulerTickOutcome::ContextChange{current}
}
#[cfg(test)]mod stage17_tests{use super::*;struct M{tick:u32,last:u32,current:u32,icsr:u32}impl SchedulerTickMmio for M{fn read32_tick(&mut self,a:usize)->u32{match a{SCHED_TICK_COUNTER_ADDR=>self.tick,SCHED_LAST_TASK_ADDR=>self.last,SCHED_CURRENT_TASK_PTR_ADDR=>0x2000,0x2000=>self.current,SCB_ICSR=>self.icsr,_=>0}}fn write32_tick(&mut self,a:usize,v:u32){match a{SCHED_TICK_COUNTER_ADDR=>self.tick=v,SCHED_LAST_TASK_ADDR=>self.last=v,SCB_ICSR=>self.icsr=v,_=>{}}}}#[test]fn same_task_is_hang(){let mut m=M{tick:2,last:9,current:9,icsr:0};assert_eq!(run_scheduler_tick_reference(&mut m),ReferenceSchedulerTickOutcome::FirmwareHang{current:9});assert_eq!(m.tick,3);assert!(SCHED_HANG_DIAGNOSTIC.starts_with(b"Firmware hang"));}#[test]fn switch_is_preserved(){let mut m=M{tick:2,last:7,current:9,icsr:1};assert_eq!(run_scheduler_tick_reference(&mut m),ReferenceSchedulerTickOutcome::ContextChange{current:9});assert_eq!((m.tick,m.last),(0,9));assert_eq!(m.icsr,1|SCHED_ICSR_OR_MASK);}}
