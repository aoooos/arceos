//! RISC-V specific page table structures.

use crate::{PageTable64, PagingMetaData};
use loongarch64::{PALEN, VALEN};
use page_table_entry::loongarch64::LA64PTE;
/// Metadata of LoongArch64 page tables.
#[derive(Copy, Clone)]
pub struct LA64MetaData;

impl const PagingMetaData for LA64MetaData {
    const LEVELS: usize = 4;
    const PA_MAX_BITS: usize = PALEN;
    const VA_MAX_BITS: usize = VALEN;
}

pub type LA64PageTable<I> = PageTable64<LA64MetaData, LA64PTE, I>;
