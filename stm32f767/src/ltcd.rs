#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    _reserved0: [u8; 0x08],
    #[doc = "0x08 - Synchronization Size Configuration Register"]
    pub sscr: crate::Reg<sscr::SSCR_SPEC>,
    #[doc = "0x0c - Back Porch Configuration Register"]
    pub bpcr: crate::Reg<bpcr::BPCR_SPEC>,
    #[doc = "0x10 - Active Width Configuration Register"]
    pub awcr: crate::Reg<awcr::AWCR_SPEC>,
    #[doc = "0x14 - Total Width Configuration Register"]
    pub twcr: crate::Reg<twcr::TWCR_SPEC>,
    #[doc = "0x18 - Global Control Register"]
    pub gcr: crate::Reg<gcr::GCR_SPEC>,
    _reserved5: [u8; 0x08],
    #[doc = "0x24 - Shadow Reload Configuration Register"]
    pub srcr: crate::Reg<srcr::SRCR_SPEC>,
    _reserved6: [u8; 0x04],
    #[doc = "0x2c - Background Color Configuration Register"]
    pub bccr: crate::Reg<bccr::BCCR_SPEC>,
    _reserved7: [u8; 0x04],
    #[doc = "0x34 - Interrupt Enable Register"]
    pub ier: crate::Reg<ier::IER_SPEC>,
    #[doc = "0x38 - Interrupt Status Register"]
    pub isr: crate::Reg<isr::ISR_SPEC>,
    #[doc = "0x3c - Interrupt Clear Register"]
    pub icr: crate::Reg<icr::ICR_SPEC>,
    #[doc = "0x40 - Line Interrupt Position Configuration Register"]
    pub lipcr: crate::Reg<lipcr::LIPCR_SPEC>,
    #[doc = "0x44 - Current Position Status Register"]
    pub cpsr: crate::Reg<cpsr::CPSR_SPEC>,
    #[doc = "0x48 - Current Display Status Register"]
    pub cdsr: crate::Reg<cdsr::CDSR_SPEC>,
    _reserved13: [u8; 0x38],
    #[doc = "0x84 - Layerx Control Register"]
    pub l1cr: crate::Reg<l1cr::L1CR_SPEC>,
    #[doc = "0x88 - Layerx Window Horizontal Position Configuration Register"]
    pub l1whpcr: crate::Reg<l1whpcr::L1WHPCR_SPEC>,
    #[doc = "0x8c - Layerx Window Vertical Position Configuration Register"]
    pub l1wvpcr: crate::Reg<l1wvpcr::L1WVPCR_SPEC>,
    #[doc = "0x90 - Layerx Color Keying Configuration Register"]
    pub l1ckcr: crate::Reg<l1ckcr::L1CKCR_SPEC>,
    #[doc = "0x94 - Layerx Pixel Format Configuration Register"]
    pub l1pfcr: crate::Reg<l1pfcr::L1PFCR_SPEC>,
    #[doc = "0x98 - Layerx Constant Alpha Configuration Register"]
    pub l1cacr: crate::Reg<l1cacr::L1CACR_SPEC>,
    #[doc = "0x9c - Layerx Default Color Configuration Register"]
    pub l1dccr: crate::Reg<l1dccr::L1DCCR_SPEC>,
    #[doc = "0xa0 - Layerx Blending Factors Configuration Register"]
    pub l1bfcr: crate::Reg<l1bfcr::L1BFCR_SPEC>,
    _reserved21: [u8; 0x08],
    #[doc = "0xac - Layerx Color Frame Buffer Address Register"]
    pub l1cfbar: crate::Reg<l1cfbar::L1CFBAR_SPEC>,
    #[doc = "0xb0 - Layerx Color Frame Buffer Length Register"]
    pub l1cfblr: crate::Reg<l1cfblr::L1CFBLR_SPEC>,
    #[doc = "0xb4 - Layerx ColorFrame Buffer Line Number Register"]
    pub l1cfblnr: crate::Reg<l1cfblnr::L1CFBLNR_SPEC>,
    _reserved24: [u8; 0x0c],
    #[doc = "0xc4 - Layerx CLUT Write Register"]
    pub l1clutwr: crate::Reg<l1clutwr::L1CLUTWR_SPEC>,
    _reserved25: [u8; 0x3c],
    #[doc = "0x104 - Layerx Control Register"]
    pub l2cr: crate::Reg<l2cr::L2CR_SPEC>,
    #[doc = "0x108 - Layerx Window Horizontal Position Configuration Register"]
    pub l2whpcr: crate::Reg<l2whpcr::L2WHPCR_SPEC>,
    #[doc = "0x10c - Layerx Window Vertical Position Configuration Register"]
    pub l2wvpcr: crate::Reg<l2wvpcr::L2WVPCR_SPEC>,
    #[doc = "0x110 - Layerx Color Keying Configuration Register"]
    pub l2ckcr: crate::Reg<l2ckcr::L2CKCR_SPEC>,
    #[doc = "0x114 - Layerx Pixel Format Configuration Register"]
    pub l2pfcr: crate::Reg<l2pfcr::L2PFCR_SPEC>,
    #[doc = "0x118 - Layerx Constant Alpha Configuration Register"]
    pub l2cacr: crate::Reg<l2cacr::L2CACR_SPEC>,
    #[doc = "0x11c - Layerx Default Color Configuration Register"]
    pub l2dccr: crate::Reg<l2dccr::L2DCCR_SPEC>,
    #[doc = "0x120 - Layerx Blending Factors Configuration Register"]
    pub l2bfcr: crate::Reg<l2bfcr::L2BFCR_SPEC>,
    _reserved33: [u8; 0x08],
    #[doc = "0x12c - Layerx Color Frame Buffer Address Register"]
    pub l2cfbar: crate::Reg<l2cfbar::L2CFBAR_SPEC>,
    #[doc = "0x130 - Layerx Color Frame Buffer Length Register"]
    pub l2cfblr: crate::Reg<l2cfblr::L2CFBLR_SPEC>,
    #[doc = "0x134 - Layerx ColorFrame Buffer Line Number Register"]
    pub l2cfblnr: crate::Reg<l2cfblnr::L2CFBLNR_SPEC>,
    _reserved36: [u8; 0x0c],
    #[doc = "0x144 - Layerx CLUT Write Register"]
    pub l2clutwr: crate::Reg<l2clutwr::L2CLUTWR_SPEC>,
}
#[doc = "SSCR register accessor: an alias for `Reg<SSCR_SPEC>`"]
pub type SSCR = crate::Reg<sscr::SSCR_SPEC>;
#[doc = "Synchronization Size Configuration Register"]
pub mod sscr;
#[doc = "BPCR register accessor: an alias for `Reg<BPCR_SPEC>`"]
pub type BPCR = crate::Reg<bpcr::BPCR_SPEC>;
#[doc = "Back Porch Configuration Register"]
pub mod bpcr;
#[doc = "AWCR register accessor: an alias for `Reg<AWCR_SPEC>`"]
pub type AWCR = crate::Reg<awcr::AWCR_SPEC>;
#[doc = "Active Width Configuration Register"]
pub mod awcr;
#[doc = "TWCR register accessor: an alias for `Reg<TWCR_SPEC>`"]
pub type TWCR = crate::Reg<twcr::TWCR_SPEC>;
#[doc = "Total Width Configuration Register"]
pub mod twcr;
#[doc = "GCR register accessor: an alias for `Reg<GCR_SPEC>`"]
pub type GCR = crate::Reg<gcr::GCR_SPEC>;
#[doc = "Global Control Register"]
pub mod gcr;
#[doc = "SRCR register accessor: an alias for `Reg<SRCR_SPEC>`"]
pub type SRCR = crate::Reg<srcr::SRCR_SPEC>;
#[doc = "Shadow Reload Configuration Register"]
pub mod srcr;
#[doc = "BCCR register accessor: an alias for `Reg<BCCR_SPEC>`"]
pub type BCCR = crate::Reg<bccr::BCCR_SPEC>;
#[doc = "Background Color Configuration Register"]
pub mod bccr;
#[doc = "IER register accessor: an alias for `Reg<IER_SPEC>`"]
pub type IER = crate::Reg<ier::IER_SPEC>;
#[doc = "Interrupt Enable Register"]
pub mod ier;
#[doc = "ISR register accessor: an alias for `Reg<ISR_SPEC>`"]
pub type ISR = crate::Reg<isr::ISR_SPEC>;
#[doc = "Interrupt Status Register"]
pub mod isr;
#[doc = "ICR register accessor: an alias for `Reg<ICR_SPEC>`"]
pub type ICR = crate::Reg<icr::ICR_SPEC>;
#[doc = "Interrupt Clear Register"]
pub mod icr;
#[doc = "LIPCR register accessor: an alias for `Reg<LIPCR_SPEC>`"]
pub type LIPCR = crate::Reg<lipcr::LIPCR_SPEC>;
#[doc = "Line Interrupt Position Configuration Register"]
pub mod lipcr;
#[doc = "CPSR register accessor: an alias for `Reg<CPSR_SPEC>`"]
pub type CPSR = crate::Reg<cpsr::CPSR_SPEC>;
#[doc = "Current Position Status Register"]
pub mod cpsr;
#[doc = "CDSR register accessor: an alias for `Reg<CDSR_SPEC>`"]
pub type CDSR = crate::Reg<cdsr::CDSR_SPEC>;
#[doc = "Current Display Status Register"]
pub mod cdsr;
#[doc = "L1CR register accessor: an alias for `Reg<L1CR_SPEC>`"]
pub type L1CR = crate::Reg<l1cr::L1CR_SPEC>;
#[doc = "Layerx Control Register"]
pub mod l1cr;
#[doc = "L1WHPCR register accessor: an alias for `Reg<L1WHPCR_SPEC>`"]
pub type L1WHPCR = crate::Reg<l1whpcr::L1WHPCR_SPEC>;
#[doc = "Layerx Window Horizontal Position Configuration Register"]
pub mod l1whpcr;
#[doc = "L1WVPCR register accessor: an alias for `Reg<L1WVPCR_SPEC>`"]
pub type L1WVPCR = crate::Reg<l1wvpcr::L1WVPCR_SPEC>;
#[doc = "Layerx Window Vertical Position Configuration Register"]
pub mod l1wvpcr;
#[doc = "L1CKCR register accessor: an alias for `Reg<L1CKCR_SPEC>`"]
pub type L1CKCR = crate::Reg<l1ckcr::L1CKCR_SPEC>;
#[doc = "Layerx Color Keying Configuration Register"]
pub mod l1ckcr;
#[doc = "L1PFCR register accessor: an alias for `Reg<L1PFCR_SPEC>`"]
pub type L1PFCR = crate::Reg<l1pfcr::L1PFCR_SPEC>;
#[doc = "Layerx Pixel Format Configuration Register"]
pub mod l1pfcr;
#[doc = "L1CACR register accessor: an alias for `Reg<L1CACR_SPEC>`"]
pub type L1CACR = crate::Reg<l1cacr::L1CACR_SPEC>;
#[doc = "Layerx Constant Alpha Configuration Register"]
pub mod l1cacr;
#[doc = "L1DCCR register accessor: an alias for `Reg<L1DCCR_SPEC>`"]
pub type L1DCCR = crate::Reg<l1dccr::L1DCCR_SPEC>;
#[doc = "Layerx Default Color Configuration Register"]
pub mod l1dccr;
#[doc = "L1BFCR register accessor: an alias for `Reg<L1BFCR_SPEC>`"]
pub type L1BFCR = crate::Reg<l1bfcr::L1BFCR_SPEC>;
#[doc = "Layerx Blending Factors Configuration Register"]
pub mod l1bfcr;
#[doc = "L1CFBAR register accessor: an alias for `Reg<L1CFBAR_SPEC>`"]
pub type L1CFBAR = crate::Reg<l1cfbar::L1CFBAR_SPEC>;
#[doc = "Layerx Color Frame Buffer Address Register"]
pub mod l1cfbar;
#[doc = "L1CFBLR register accessor: an alias for `Reg<L1CFBLR_SPEC>`"]
pub type L1CFBLR = crate::Reg<l1cfblr::L1CFBLR_SPEC>;
#[doc = "Layerx Color Frame Buffer Length Register"]
pub mod l1cfblr;
#[doc = "L1CFBLNR register accessor: an alias for `Reg<L1CFBLNR_SPEC>`"]
pub type L1CFBLNR = crate::Reg<l1cfblnr::L1CFBLNR_SPEC>;
#[doc = "Layerx ColorFrame Buffer Line Number Register"]
pub mod l1cfblnr;
#[doc = "L1CLUTWR register accessor: an alias for `Reg<L1CLUTWR_SPEC>`"]
pub type L1CLUTWR = crate::Reg<l1clutwr::L1CLUTWR_SPEC>;
#[doc = "Layerx CLUT Write Register"]
pub mod l1clutwr;
#[doc = "L2CR register accessor: an alias for `Reg<L2CR_SPEC>`"]
pub type L2CR = crate::Reg<l2cr::L2CR_SPEC>;
#[doc = "Layerx Control Register"]
pub mod l2cr;
#[doc = "L2WHPCR register accessor: an alias for `Reg<L2WHPCR_SPEC>`"]
pub type L2WHPCR = crate::Reg<l2whpcr::L2WHPCR_SPEC>;
#[doc = "Layerx Window Horizontal Position Configuration Register"]
pub mod l2whpcr;
#[doc = "L2WVPCR register accessor: an alias for `Reg<L2WVPCR_SPEC>`"]
pub type L2WVPCR = crate::Reg<l2wvpcr::L2WVPCR_SPEC>;
#[doc = "Layerx Window Vertical Position Configuration Register"]
pub mod l2wvpcr;
#[doc = "L2CKCR register accessor: an alias for `Reg<L2CKCR_SPEC>`"]
pub type L2CKCR = crate::Reg<l2ckcr::L2CKCR_SPEC>;
#[doc = "Layerx Color Keying Configuration Register"]
pub mod l2ckcr;
#[doc = "L2PFCR register accessor: an alias for `Reg<L2PFCR_SPEC>`"]
pub type L2PFCR = crate::Reg<l2pfcr::L2PFCR_SPEC>;
#[doc = "Layerx Pixel Format Configuration Register"]
pub mod l2pfcr;
#[doc = "L2CACR register accessor: an alias for `Reg<L2CACR_SPEC>`"]
pub type L2CACR = crate::Reg<l2cacr::L2CACR_SPEC>;
#[doc = "Layerx Constant Alpha Configuration Register"]
pub mod l2cacr;
#[doc = "L2DCCR register accessor: an alias for `Reg<L2DCCR_SPEC>`"]
pub type L2DCCR = crate::Reg<l2dccr::L2DCCR_SPEC>;
#[doc = "Layerx Default Color Configuration Register"]
pub mod l2dccr;
#[doc = "L2BFCR register accessor: an alias for `Reg<L2BFCR_SPEC>`"]
pub type L2BFCR = crate::Reg<l2bfcr::L2BFCR_SPEC>;
#[doc = "Layerx Blending Factors Configuration Register"]
pub mod l2bfcr;
#[doc = "L2CFBAR register accessor: an alias for `Reg<L2CFBAR_SPEC>`"]
pub type L2CFBAR = crate::Reg<l2cfbar::L2CFBAR_SPEC>;
#[doc = "Layerx Color Frame Buffer Address Register"]
pub mod l2cfbar;
#[doc = "L2CFBLR register accessor: an alias for `Reg<L2CFBLR_SPEC>`"]
pub type L2CFBLR = crate::Reg<l2cfblr::L2CFBLR_SPEC>;
#[doc = "Layerx Color Frame Buffer Length Register"]
pub mod l2cfblr;
#[doc = "L2CFBLNR register accessor: an alias for `Reg<L2CFBLNR_SPEC>`"]
pub type L2CFBLNR = crate::Reg<l2cfblnr::L2CFBLNR_SPEC>;
#[doc = "Layerx ColorFrame Buffer Line Number Register"]
pub mod l2cfblnr;
#[doc = "L2CLUTWR register accessor: an alias for `Reg<L2CLUTWR_SPEC>`"]
pub type L2CLUTWR = crate::Reg<l2clutwr::L2CLUTWR_SPEC>;
#[doc = "Layerx CLUT Write Register"]
pub mod l2clutwr;