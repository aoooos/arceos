#define INVTLB_ADDR_GFALSE_AND_ASID	5
#define PAGE_SHIFT          12
#define PTRS_PER_PGD_BITS	(PAGE_SHIFT - 3)
#define PTRS_PER_PUD_BITS	(PAGE_SHIFT - 3)
#define PTRS_PER_PMD_BITS	(PAGE_SHIFT - 3)
#define PTRS_PER_PTE_BITS	(PAGE_SHIFT - 3)

/* Page table bits */
#define	_PAGE_VALID_SHIFT	0
#define	_PAGE_ACCESSED_SHIFT	0  /* Reuse Valid for Accessed */
#define	_PAGE_DIRTY_SHIFT	1
#define	_PAGE_PLV_SHIFT		2  /* 2~3, two bits */
#define	_CACHE_SHIFT		4  /* 4~5, two bits */
#define	_PAGE_GLOBAL_SHIFT	6
#define	_PAGE_HUGE_SHIFT	6  /* HUGE is a PMD bit */
#define	_PAGE_PRESENT_SHIFT	7
#define	_PAGE_WRITE_SHIFT	8
#define	_PAGE_MODIFIED_SHIFT	9
#define	_PAGE_PROTNONE_SHIFT	10
#define	_PAGE_SPECIAL_SHIFT	11
#define	_PAGE_HGLOBAL_SHIFT	12 /* HGlobal is a PMD bit */
#define	_PAGE_PFN_SHIFT		12
#define	_PAGE_SWP_EXCLUSIVE_SHIFT 23
#define	_PAGE_PFN_END_SHIFT	48
#define	_PAGE_NO_READ_SHIFT	61
#define	_PAGE_NO_EXEC_SHIFT	62
#define	_PAGE_RPLV_SHIFT	63

/* Used by software */
#define _PAGE_PRESENT		(_ULCAST_(1) << _PAGE_PRESENT_SHIFT)
#define _PAGE_WRITE		(_ULCAST_(1) << _PAGE_WRITE_SHIFT)
#define _PAGE_ACCESSED		(_ULCAST_(1) << _PAGE_ACCESSED_SHIFT)
#define _PAGE_MODIFIED		(_ULCAST_(1) << _PAGE_MODIFIED_SHIFT)
#define _PAGE_PROTNONE		(_ULCAST_(1) << _PAGE_PROTNONE_SHIFT)
#define _PAGE_SPECIAL		(_ULCAST_(1) << _PAGE_SPECIAL_SHIFT)

/* We borrow bit 23 to store the exclusive marker in swap PTEs. */
#define _PAGE_SWP_EXCLUSIVE	(_ULCAST_(1) << _PAGE_SWP_EXCLUSIVE_SHIFT)

/* Used by TLB hardware (placed in EntryLo*) */
#define _PAGE_VALID		(_ULCAST_(1) << _PAGE_VALID_SHIFT)
#define _PAGE_DIRTY		(_ULCAST_(1) << _PAGE_DIRTY_SHIFT)
#define _PAGE_PLV		(_ULCAST_(3) << _PAGE_PLV_SHIFT)
#define _PAGE_GLOBAL		(_ULCAST_(1) << _PAGE_GLOBAL_SHIFT)
#define _PAGE_HUGE		(_ULCAST_(1) << _PAGE_HUGE_SHIFT)
#define _PAGE_HGLOBAL		(_ULCAST_(1) << _PAGE_HGLOBAL_SHIFT)
#define _PAGE_NO_READ		(_ULCAST_(1) << _PAGE_NO_READ_SHIFT)
#define _PAGE_NO_EXEC		(_ULCAST_(1) << _PAGE_NO_EXEC_SHIFT)
#define _PAGE_RPLV		(_ULCAST_(1) << _PAGE_RPLV_SHIFT)
#define _CACHE_MASK		(_ULCAST_(3) << _CACHE_SHIFT)
#define _PFN_SHIFT		(PAGE_SHIFT - 12 + _PAGE_PFN_SHIFT)

#define _PAGE_USER	(PLV_USER << _PAGE_PLV_SHIFT)
#define _PAGE_KERN	(PLV_KERN << _PAGE_PLV_SHIFT)


.comm    kernelsp, 0x1000, 0x1000

.macro SAVE_STATIC_REGS
	st.d	$s0, $sp, 8*23
	st.d	$s1, $sp, 8*24
	st.d	$s2, $sp, 8*25
	st.d	$s3, $sp, 8*26
	st.d	$s4, $sp, 8*27
	st.d	$s5, $sp, 8*28
	st.d	$s6, $sp, 8*29
	st.d	$s7, $sp, 8*30
	st.d	$s8, $sp, 8*31
.endm

.macro LOAD_STATIC_REGS
	ld.d	$s0, $sp, 8*23
	ld.d	$s1, $sp, 8*24
	ld.d	$s2, $sp, 8*25
	ld.d	$s3, $sp, 8*26
	ld.d	$s4, $sp, 8*27
	ld.d	$s5, $sp, 8*28
	ld.d	$s6, $sp, 8*29
	ld.d	$s7, $sp, 8*30
	ld.d	$s8, $sp, 8*31
.endm

.macro SAVE_ARGUMENT_REGS
	st.d	$a0, $sp, 8*4
	st.d	$a1, $sp, 8*5
	st.d	$a2, $sp, 8*6
	st.d	$a3, $sp, 8*7
	st.d	$a4, $sp, 8*8
	st.d	$a5, $sp, 8*9
	st.d	$a6, $sp, 8*10
	st.d	$a7, $sp, 8*11
.endm

.macro LOAD_ARGUMENT_REGS
	ld.d	$a0, $sp, 8*4
	ld.d	$a1, $sp, 8*5
	ld.d	$a2, $sp, 8*6
	ld.d	$a3, $sp, 8*7
	ld.d	$a4, $sp, 8*8
	ld.d	$a5, $sp, 8*9
	ld.d	$a6, $sp, 8*10
	ld.d	$a7, $sp, 8*11
.endm

.macro SAVE_TEMP_REGS
	st.d	$t0, $sp, 8*12
	st.d	$t1, $sp, 8*13
	st.d	$t2, $sp, 8*14
	st.d	$t3, $sp, 8*15
	st.d	$t4, $sp, 8*16
	st.d	$t5, $sp, 8*17
	st.d	$t6, $sp, 8*18
	st.d	$t7, $sp, 8*19
	st.d	$t8, $sp, 8*20
.endm

.macro LOAD_TEMP_REGS
	ld.d	$t0, $sp, 8*12
	ld.d	$t1, $sp, 8*13
	ld.d	$t2, $sp, 8*14
	ld.d	$t3, $sp, 8*15
	ld.d	$t4, $sp, 8*16
	ld.d	$t5, $sp, 8*17
	ld.d	$t6, $sp, 8*18
	ld.d	$t7, $sp, 8*19
	ld.d	$t8, $sp, 8*20
.endm

.macro SAVE_CSR_REGS
    csrrd		$t2, 0x1
	st.d		$t2, $sp, 8*33  // prmd
	csrrd		$t2, 0x0
	st.d		$t2, $sp, 8*32  // crmd
	csrrd		$t2, 0x2
	st.d		$t2, $sp, 8*34  // euen
	csrrd		$t2, 0x4
	st.d		$t2, $sp, 8*35  // ecfg
	csrrd		$t2, 0x5
	st.d		$t2, $sp, 8*36  // estat
.endm

.macro LOAD_SOME_REGS
	ld.d	$a0, $sp, 8*33  // prmd
	andi    $a0, $a0, 0x3   // extract pplv bit (previous privilege level)
	beqz    $a0, 8f
	//ld.d    $r21, $sp, 8*21 // reserved reg
8:
	ld.d	$a0, $sp, 8*37  // era
	csrwr	$a0, 0x6
	ld.d	$a0, $sp, 8*33  // prmd
	csrwr	$a0, 0x1
	ld.d	$ra, $sp, 8*1   // ra

    LOAD_ARGUMENT_REGS

	ld.d	$tp, $sp, 8*2   // thread pointer
	ld.d	$fp, $sp, 8*22  // frame pointer
.endm


.macro SAVE_REGS
	addi.d		$sp, $sp, -{trapframe_size} // allocate space
	st.d		$t2, $sp, 8*3   // sp
	st.d		$zero, $sp, 8*0 // zero

    SAVE_CSR_REGS
	SAVE_TEMP_REGS

	st.d		$ra, $sp, 8*1   // ra

    SAVE_ARGUMENT_REGS

	csrrd		$ra, 0x6
	st.d		$ra, $sp, 8*37  // era

	st.d		$tp, $sp, 8*2   // thread pointer
	//st.d		$r21, $sp, 8*21 // reserved reg
	st.d		$fp, $sp, 8*22  // frame pointer

    SAVE_STATIC_REGS

	move	$r21, $sp
	move	$a0, $sp
.endm


.macro LOAD_REGS
	move	$sp, $r21
    LOAD_STATIC_REGS

    LOAD_TEMP_REGS

    LOAD_SOME_REGS

	//ld.d	$sp, $sp, 8*3   // sp
	addi.d		$sp, $sp, {trapframe_size}
	csrrd		$r21, 0x6
	addi.d 		$r21, $r21, 0x4
	csrwr		$r21, 0x6
.endm

/*
.macro tlb_do_page_fault, write
	SAVE_REGS

	csrrd		$a2, 0x7
	move		$a0, $sp
	st.d		$a2, $sp, 8*38
	li.d		$a1, \write
	bl			do_page_fault

	LOAD_REGS

	ertn
.endm
*/


.section .text
.balign 4096
.global trap_vector_base
trap_vector_base:
    SAVE_REGS

    bl loongarch64_trap_handler

    LOAD_REGS

    ertn


.section .text.tlbrefill
.balign 4096
.global tlb_refill_handler
tlb_refill_handler:
    csrwr $t0, 0x8B	// tlbrsave
    csrrd $t0, 0x1B	// pgd
    lddir $t0, $t0, 4
	lddir $t0, $t0, 3 
	lddir $t0, $t0, 2
    lddir $t0, $t0, 1 
    ldpte $t0, 0	#取回偶数号页表项
		csrrd $t0, 0x8c	// tlbrelo0, used to debug
    ldpte $t0, 1 	#取回奇数号页表项
		csrrd $t0, 0x8d // tlbrelo1, used to debug
    tlbfill
    csrrd $t0, 0x8B	// tlbrsave
    ertn