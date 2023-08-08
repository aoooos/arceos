use axconfig::{PHYS_VIRT_OFFSET, TASK_STACK_SIZE};
use loongarch64::register::csr::Register;
use loongarch64::tlb::pwch::Pwch;
use loongarch64::tlb::pwcl::Pwcl;

#[link_section = ".bss.stack"]
static mut BOOT_STACK: [u8; TASK_STACK_SIZE] = [0; TASK_STACK_SIZE];

#[link_section = ".data.boot_page_table"]
static mut BOOT_PT: [u64; 512] = [0; 512];

unsafe fn init_mmu() {
    crate::arch::init_tlb();
    Pwcl::read()
        .set_ptbase(12) //页表起始位置
        .set_ptwidth(9) //页表宽度为9位
        .set_dir1_base(21) //第一级页目录表起始位置
        .set_dir1_width(9) //第一级页目录表宽度为9位
        .set_dir2_base(30) //第二级页目录表起始位置
        .set_dir2_width(9) //第二级页目录表宽度为9位
        .write();
    Pwch::read()
        .set_dir3_base(39) //第三级页目录表
        .set_dir3_width(9) //第三级页目录表宽度为9位
        //.set_dir4_base(48) //第四级页目录表
        .set_dir4_width(0) //第四级页目录表
        .write();
}

/// The earliest entry point for the primary CPU.
#[naked]
#[no_mangle]
#[link_section = ".text.boot"]
unsafe extern "C" fn _start() -> ! {
    core::arch::asm!("
            # config direct window (inspired from linux source code)
            ori $t0, $zero, 0x1     # CSR_DMW1_PLV0
            lu52i.d $t0, $t0, -2048 # UC, PLV0, 0x8000 xxxx xxxx xxxx
            csrwr $t0,0x180         #LOONGARCH_CSR_DMWIN0

            ori $t0, $zero, 0x11    # CSR_DMW1_MAT | CSR_DMW1_PLV0
            lu52i.d $t0, $t0, -1792 # CA, PLV0, 0x9000 xxxx xxxx xxxx
            csrwr $t0,0x181         #LOONGARCH_CSR_DMWIN1

            bl          {init_mmu}

            # Enable PG 
            li.w		$t0, 0xb0		# PLV=0, IE=0, PG=1
            csrwr		$t0, 0x0        # LOONGARCH_CSR_CRMD
            li.w		$t0, 0x04		# PLV=0, PIE=1, PWE=0
            csrwr		$t0, 0x1        # LOONGARCH_CSR_PRMD
            li.w		$t0, 0x00		# FPE=0, SXE=0, ASXE=0, BTE=0
            csrwr		$t0, 0x2        # LOONGARCH_CSR_EUEN
            
            la.global   $sp, {boot_stack}
            li.d        $t0, {boot_stack_size}
            add.d       $sp, $sp, $t0              // setup boot stack

            csrrd       $a0, 0x20       # LOONGARCH_CSR_CPUID
            bl {entry}
            ",
        boot_stack_size = const TASK_STACK_SIZE,
        boot_stack = sym BOOT_STACK,
        entry = sym super::rust_entry,
        init_mmu = sym init_mmu,
        options(noreturn),
    )
}

/// The earliest entry point for secondary CPUs.
#[cfg(feature = "smp")]
#[naked]
#[no_mangle]
#[link_section = ".text.boot"]
unsafe extern "C" fn _start_secondary() -> ! {
    // a0 = hartid
    // a1 = SP
    /*
    core::arch::asm!("
        mv      s0, a0                  // save hartid
        mv      sp, a1                  // set SP

        call    {init_mmu}              // setup boot page table and enabel MMU

        li      s1, {phys_virt_offset}  // fix up virtual high address
        add     a1, a1, s1
        add     sp, sp, s1

        mv      a0, s0
        la      a1, {entry}
        add     a1, a1, s1
        jalr    a1                      // call rust_entry_secondary(hartid)
        j       .",
        phys_virt_offset = const PHYS_VIRT_OFFSET,
        init_mmu = sym init_mmu,
        entry = sym super::rust_entry_secondary,
        options(noreturn),
    )*/
}
