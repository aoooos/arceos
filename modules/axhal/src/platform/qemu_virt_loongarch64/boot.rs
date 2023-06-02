use axconfig::{PHYS_VIRT_OFFSET, TASK_STACK_SIZE};

#[link_section = ".bss.stack"]
static mut BOOT_STACK: [u8; TASK_STACK_SIZE] = [0; TASK_STACK_SIZE];

#[link_section = ".data.boot_page_table"]
static mut BOOT_PT: [u64; 512] = [0; 512];

/// The earliest entry point for the primary CPU.
#[naked]
#[no_mangle]
#[link_section = ".text.boot"]
unsafe extern "C" fn _start() -> ! {
    // PC = 0x8020_0000
    // a0 = hartid
    // a1 = dtb
    core::arch::asm!("
    0:
        #设置映射窗口
        li.d $t0,{phys_virt_offset}
        addi.d $t0,$t0,0x11
        csrwr $t0,0x180  #设置LOONGARCH_CSR_DMWIN0
    
        la.global $t0,1f
        jirl $zero, $t0,0
    1:
        la.global $t0, ebss
        la.global $t1, sbss
        bgeu $t0, $t1, 3f   #bge如果前者大于等于后者则跳转
    2:
        st.d $zero, $t0,0
        addi.d $t0, $t0, 8
        bltu $t0, $t1, 2b
    3:
        la.global $sp, {boot_stack}
        li.d      $t0, {boot_stack_size}
        add.d       $sp, $sp, $t0     # setup boot stack
        bl {entry}
        ",
        phys_virt_offset = const PHYS_VIRT_OFFSET,
        boot_stack_size = const TASK_STACK_SIZE,
        boot_stack = sym BOOT_STACK,
        entry = sym super::rust_entry,
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
    )
}
