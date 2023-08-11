use super::context::TrapFrame;
use core::arch::asm;
//use libax::println;
use log::info;
use loongarch64::register::csr::Register;
use loongarch64::register::eentry::Eentry;
use loongarch64::register::estat::{self, Estat, Exception, Trap};
use loongarch64::register::prcfg3::Prcfg3;
use loongarch64::tlb::TLBELO;

use loongarch64::cpu::{
    get_mmu_support_page, get_palen, get_support_execution_protection, get_support_lspw,
    get_support_read_forbid, get_support_rplv, get_support_rva, get_support_rva_len, get_valen,
};
use loongarch64::register::crmd::Crmd;
use loongarch64::register::dmwn::{Dmw0, Dmw1};
use loongarch64::register::ecfg::Ecfg;
use loongarch64::register::prcfg1::Prcfg1;
use loongarch64::register::prcfg2::Prcfg2;
use loongarch64::register::tcfg::Tcfg;
use loongarch64::register::Misc;
use loongarch64::tlb::TlbREhi;
use loongarch64::tlb::{Pgd, Pgdh, Pgdl, Pwch, Pwcl, StlbPs};
use loongarch64::tlb::{TLBREntry, TlbRelo};

// 打印硬件的相关信息
pub fn print_machine_info() {
    info!("PALEN: {}", get_palen()); //支持的物理地址范围
    info!("VALEN: {}", get_valen()); //支持的虚拟地址范围
    info!("Support MMU-Page :{}", get_mmu_support_page());
    info!("Support Read-only :{}", get_support_read_forbid());
    info!(
        "Support Execution-Protect :{}",
        get_support_execution_protection()
    );
    info!("Support RPLV: {}", get_support_rplv()); //是否支持吃rplv页属性
    info!("Support RVA: {}", get_support_rva()); //是否支持虚拟地址缩减
    info!("Support RVAMAX :{}", get_support_rva_len()); //支持的虚拟地址缩减的长度
    info!("Support Page-Size: {:#x}", Prcfg2::read().get_val()); //支持的页大小,
    info!("Support LSPW: {}", get_support_lspw());
    match Prcfg3::read().get_tlb_type() {
        0 => {
            info!("No TLB");
        }
        1 => {
            info!("Have MTLB");
        }
        2 => {
            info!("Have STLB + MTLB");
        }
        _ => {
            info!("Unknown TLB");
        }
    }
    info!("MLTB Entry: {}", Prcfg3::read().get_mtlb_entries()); //MTLB的页数量
    info!("SLTB Ways :{}", Prcfg3::read().get_stlb_ways()); //STLB的路数量
    info!("SLTB Entry: {}", Prcfg3::read().get_sltb_sets()); //STLB每一路的项数
    info!("SLTB Page-size: {}", StlbPs::read().get_page_size()); //STLB的页大小
    info!("PTE-size: {}", Pwcl::read().get_pte_width()); //PTE的大小
    info!("TLB-RFill entry_point: {:#x}", TLBREntry::read().get_val()); //TLB重新加载的入口地址
    info!("TLB-RFill page-size :{}", TlbREhi::read().get_page_size()); //TLB重新加载的页大小
    let pwcl = Pwcl::read();
    info!(
        "PT-index-width: {},{}",
        pwcl.get_ptbase(),
        pwcl.get_ptwidth()
    ); //PT的索引宽度
    info!(
        "dir1-index-width: {},{}",
        pwcl.get_dir1_base(),
        pwcl.get_dir1_width()
    ); //dir1的索引宽度
    let pwch = Pwch::read();
    info!(
        "dir2-index-width: {},{}",
        pwcl.get_dir2_base(),
        pwcl.get_dir2_width()
    ); //dir2的索引宽度
    info!(
        "dir3-index-width: {},{}",
        pwch.get_dir3_base(),
        pwch.get_dir3_width()
    ); //dir3的索引宽度
    info!(
        "dir4-index-width: {},{}",
        pwch.get_dir4_base(),
        pwch.get_dir4_width()
    ); //dir4的索引宽度
    let crmd = Crmd::read();
    info!("DA: {}", crmd.get_da()); //是否支持DA模式
    info!("PG :{}", crmd.get_pg()); //是否支持PG模式
    info!("DATF: {}", crmd.get_datf()); //
    info!("DATM :{}", crmd.get_datm()); //
    info!("CRMD :{:#x}", crmd.get_val()); //
    let misc = Misc::read().get_enable_32_in_plv3();
    info!("MISC: enable_32_in_plv3 :{}", misc); //是否支持32位在PLV3模式下运行
    info!("dmwo: {:#x}", Dmw0::read().get_value());
    info!("dmw1: {:#x}", Dmw1::read().get_value());
    info!("PLV: {}", crmd.get_plv()); //
}

// 检查初始化后的硬件是否正确
pub fn checkout_after_init() {
    info!(
        "Direct address translation enabled: {}",
        Crmd::read().get_da()
    ); //是否开启直接地址转换
    info!("Map address translation enabled: {}", Crmd::read().get_pg()); //是否开启映射地址转换
    info!("TLBRENTRY: {:#x}", TLBREntry::read().get_val()); //打印TLB重填异常的处理程序地址
}
pub fn test_csr_register() {
    let estat = Estat::read();
    info!("estat = {:#x}", estat.get_val());
    // 打印当前的特权级
    let crmd = Crmd::read();
    let spp = crmd.get_plv();
    info!("Privilege level:{}", spp);
    // 打印是否开启全局中断
    let interrupt = crmd.get_ie();
    info!("global Interrupt:{}", interrupt);
    // 打印中断入口地址是否同一个
    let ecfg = Ecfg::read();
    let add = ecfg.get_vs();
    info!("vs = {}", add);
    // 打印中断入口地址
    let eentry = Eentry::read();
    let add = eentry.get_eentry();
    info!("eentry = {:#x}", add);
    // save 寄存器个数
    let prcfg1 = Prcfg1::read();
    let prc = prcfg1.get_save_num();
    let time_bits = prcfg1.get_timer_bits();
    info!("save register num:{}", prc);
    info!("timer bits:{}", time_bits);
    info!("{:?}", prcfg1);

    //查看页表相关
    let pgdh = Pgdh::read();
    let pgdh = pgdh.get_val();
    let pgdl = Pgdl::read();
    let pgdl = pgdl.get_val();
    let pgd = Pgd::read();
    let pgd = pgd.get_val();
    info!("Pgdh = {:#x}", pgdh);
    info!("Pgdl = {:#x}", pgdl);
    info!("Pgd = {:#x}", pgd);

    let mut pgd: u32;
    let mut dir2base: u32;
    let mut dir1base: u32;
    let mut ptbase: u32;

    unsafe { asm!("csrrd {}, 0x1B",out(reg)pgd) };
    unsafe { asm!("lddir {}, {}, 3",out(reg)dir2base,in(reg)pgd) };
    unsafe {
        asm!("
    bstrpick.d {d}, {d}, 63, 12
    slli.d {d}, {d}, 12",d=inout(reg)dir2base)
    };
    unsafe { asm!("lddir {}, {}, 2",out(reg)dir1base,in(reg)dir2base) };
    unsafe {
        asm!("
    bstrpick.d {d}, {d}, 63, 12
    slli.d {d}, {d}, 12",d=inout(reg)dir1base)
    };
    unsafe { asm!("lddir {}, {}, 1",out(reg)ptbase,in(reg)dir1base) };
    unsafe {
        asm!("
    bstrpick.d {d}, {d}, 63, 12
    slli.d {d}, {d}, 12",d=inout(reg)ptbase)
    };

    info!("dir3base : {:#x}", pgd);
    info!("dir2base : {:#x}", dir2base);
    info!("dir1base : {:#x}", dir1base);
    info!("ptbase : {:#x}", ptbase);
    unsafe { asm!("ldpte {}, 0",in(reg)ptbase) };
    unsafe { asm!("ldpte {}, 1",in(reg)ptbase) };
    let tlbrelo0 = TlbRelo::read(0);
    let tlbrelo1 = TlbRelo::read(1);
    info!("{:#x?}", tlbrelo0);
    info!("{:#x?}", tlbrelo1);

    //查看计时器配置
    let tcfg = Tcfg::read();
    let enable = tcfg.get_enable();
    let loop_ = tcfg.get_loop();
    let tval = tcfg.get_val();

    info!("time_enable:{}", enable);
    info!("time_loop:{}", loop_);
    info!("time_tval:{}", tval);
    // 查看地址翻译模式
    let da = crmd.get_da();
    info!("da:{}", da);
    let pg = crmd.get_pg();
    info!("pg:{}", pg);
    info!("dmwo:{:#x}", Dmw0::read().get_value());
    info!("dmw1:{:#x}", Dmw1::read().get_value());
    info!("TLB-reload entry_point :{:#x}", TLBREntry::read().get_val());
    // 查看哪些中断被打开了
    for i in 0..13 {
        let interrupt = ecfg.get_lie_with_index(i);
        info!("local_interrupt {}:{}", i, interrupt);
    }
}

core::arch::global_asm!(
    include_str!("trap.S"),
    trapframe_size = const core::mem::size_of::<TrapFrame>(),
);

fn handle_breakpoint(era: &mut usize) {
    debug!("Exception(Breakpoint) @ {:#x} ", era);
}

#[no_mangle]
fn loongarch64_trap_handler(tf: &mut TrapFrame) {
    let estat = Estat::read();
    let eentry = Eentry::read();
    match estat.cause() {
        Trap::Exception(Exception::Breakpoint) => handle_breakpoint(&mut tf.era),
        //Trap::Exception(Exception::StorePageInvalid) | Trap::Exception(Exception::LoadPageInvalid) => test_csr_register(), Trap::Interrupt(_) => crate::trap::handle_irq_extern(estat.bits),
        _ => {
            panic!(
                "Unhandled trap {:?} @ {:#x}:\n{:#x?}",
                estat.cause(),
                tf.era,
                tf
            );
        }
    }
}
