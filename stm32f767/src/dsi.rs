#[doc = r"Register block"]
#[repr(C)]
pub struct RegisterBlock {
    #[doc = "0x00 - DSI Host Version Register"]
    pub dsi_vr: crate::Reg<dsi_vr::DSI_VR_SPEC>,
    #[doc = "0x04 - DSI Host Control Register"]
    pub dsi_cr: crate::Reg<dsi_cr::DSI_CR_SPEC>,
    #[doc = "0x08 - DSI HOST Clock Control Register"]
    pub dsi_ccr: crate::Reg<dsi_ccr::DSI_CCR_SPEC>,
    #[doc = "0x0c - DSI Host LTDC VCID Register"]
    pub dsi_lvcidr: crate::Reg<dsi_lvcidr::DSI_LVCIDR_SPEC>,
    #[doc = "0x10 - DSI Host LTDC Color Coding Register"]
    pub dsi_lcolcr: crate::Reg<dsi_lcolcr::DSI_LCOLCR_SPEC>,
    #[doc = "0x14 - DSI Host LTDC Polarity Configuration Register"]
    pub dsi_lpcr: crate::Reg<dsi_lpcr::DSI_LPCR_SPEC>,
    #[doc = "0x18 - DSI Host Low-Power mode Configuration Register"]
    pub dsi_lpmcr: crate::Reg<dsi_lpmcr::DSI_LPMCR_SPEC>,
    _reserved7: [u8; 0x10],
    #[doc = "0x2c - DSI Host Protocol Configuration Register"]
    pub dsi_pcr: crate::Reg<dsi_pcr::DSI_PCR_SPEC>,
    #[doc = "0x30 - DSI Host Generic VCID Register"]
    pub dsi_gvcidr: crate::Reg<dsi_gvcidr::DSI_GVCIDR_SPEC>,
    #[doc = "0x34 - DSI Host mode Configuration Register"]
    pub dsi_mcr: crate::Reg<dsi_mcr::DSI_MCR_SPEC>,
    #[doc = "0x38 - DSI Host Video mode Configuration Register"]
    pub dsi_vmcr: crate::Reg<dsi_vmcr::DSI_VMCR_SPEC>,
    #[doc = "0x3c - DSI Host Video Packet Configuration Register"]
    pub dsi_vpcr: crate::Reg<dsi_vpcr::DSI_VPCR_SPEC>,
    #[doc = "0x40 - DSI Host Video Chunks Configuration Register"]
    pub dsi_vccr: crate::Reg<dsi_vccr::DSI_VCCR_SPEC>,
    #[doc = "0x44 - DSI Host Video Null Packet Configuration Register"]
    pub dsi_vnpcr: crate::Reg<dsi_vnpcr::DSI_VNPCR_SPEC>,
    #[doc = "0x48 - DSI Host Video HSA Configuration Register"]
    pub dsi_vhsacr: crate::Reg<dsi_vhsacr::DSI_VHSACR_SPEC>,
    #[doc = "0x4c - DSI Host Video HBP Configuration Register"]
    pub dsi_vhbpcr: crate::Reg<dsi_vhbpcr::DSI_VHBPCR_SPEC>,
    #[doc = "0x50 - DSI Host Video Line Configuration Register"]
    pub dsi_vlcr: crate::Reg<dsi_vlcr::DSI_VLCR_SPEC>,
    #[doc = "0x54 - DSI Host Video VSA Configuration Register"]
    pub dsi_vvsacr: crate::Reg<dsi_vvsacr::DSI_VVSACR_SPEC>,
    #[doc = "0x58 - DSI Host Video VBP Configuration Register"]
    pub dsi_vvbpcr: crate::Reg<dsi_vvbpcr::DSI_VVBPCR_SPEC>,
    #[doc = "0x5c - DSI Host Video VFP Configuration Register"]
    pub dsi_vvfpcr: crate::Reg<dsi_vvfpcr::DSI_VVFPCR_SPEC>,
    #[doc = "0x60 - DSI Host Video VA Configuration Register"]
    pub dsi_vvacr: crate::Reg<dsi_vvacr::DSI_VVACR_SPEC>,
    #[doc = "0x64 - DSI Host LTDC Command Configuration Register"]
    pub dsi_lccr: crate::Reg<dsi_lccr::DSI_LCCR_SPEC>,
    #[doc = "0x68 - DSI Host Command mode Configuration Register"]
    pub dsi_cmcr: crate::Reg<dsi_cmcr::DSI_CMCR_SPEC>,
    #[doc = "0x6c - DSI Host Generic Header Configuration Register"]
    pub dsi_ghcr: crate::Reg<dsi_ghcr::DSI_GHCR_SPEC>,
    #[doc = "0x70 - DSI Host Generic Payload Data Register"]
    pub dsi_gpdr: crate::Reg<dsi_gpdr::DSI_GPDR_SPEC>,
    #[doc = "0x74 - DSI Host Generic Packet Status Register"]
    pub dsi_gpsr: crate::Reg<dsi_gpsr::DSI_GPSR_SPEC>,
    #[doc = "0x78 - DSI Host Timeout Counter Configuration Register 0"]
    pub dsi_tccr0: crate::Reg<dsi_tccr0::DSI_TCCR0_SPEC>,
    #[doc = "0x7c - DSI Host Timeout Counter Configuration Register 1"]
    pub dsi_tccr1: crate::Reg<dsi_tccr1::DSI_TCCR1_SPEC>,
    #[doc = "0x80 - DSI Host Timeout Counter Configuration Register 2"]
    pub dsi_tccr2: crate::Reg<dsi_tccr2::DSI_TCCR2_SPEC>,
    #[doc = "0x84 - DSI Host Timeout Counter Configuration Register 3"]
    pub dsi_tccr3: crate::Reg<dsi_tccr3::DSI_TCCR3_SPEC>,
    #[doc = "0x88 - DSI Host Timeout Counter Configuration Register 4"]
    pub dsi_tccr4: crate::Reg<dsi_tccr4::DSI_TCCR4_SPEC>,
    #[doc = "0x8c - DSI Host Timeout Counter Configuration Register 5"]
    pub dsi_tccr5: crate::Reg<dsi_tccr5::DSI_TCCR5_SPEC>,
    _reserved32: [u8; 0x04],
    #[doc = "0x94 - DSI Host Clock Lane Configuration Register"]
    pub dsi_clcr: crate::Reg<dsi_clcr::DSI_CLCR_SPEC>,
    #[doc = "0x98 - DSI Host Clock Lane Timer Configuration Register"]
    pub dsi_cltcr: crate::Reg<dsi_cltcr::DSI_CLTCR_SPEC>,
    #[doc = "0x9c - DSI Host Data Lane Timer Configuration Register"]
    pub dsi_dltcr: crate::Reg<dsi_dltcr::DSI_DLTCR_SPEC>,
    #[doc = "0xa0 - DSI Host PHY Control Register"]
    pub dsi_pctlr: crate::Reg<dsi_pctlr::DSI_PCTLR_SPEC>,
    #[doc = "0xa4 - DSI Host PHY Configuration Register"]
    pub dsi_pconfr: crate::Reg<dsi_pconfr::DSI_PCONFR_SPEC>,
    #[doc = "0xa8 - DSI Host PHY ULPS Control Register"]
    pub dsi_pucr: crate::Reg<dsi_pucr::DSI_PUCR_SPEC>,
    #[doc = "0xac - DSI Host PHY TX Triggers Configuration Register"]
    pub dsi_pttcr: crate::Reg<dsi_pttcr::DSI_PTTCR_SPEC>,
    #[doc = "0xb0 - DSI Host PHY Status Register"]
    pub dsi_psr: crate::Reg<dsi_psr::DSI_PSR_SPEC>,
    _reserved40: [u8; 0x08],
    #[doc = "0xbc - DSI Host Interrupt & Status Register 0"]
    pub dsi_isr0: crate::Reg<dsi_isr0::DSI_ISR0_SPEC>,
    #[doc = "0xc0 - DSI Host Interrupt & Status Register 1"]
    pub dsi_isr1: crate::Reg<dsi_isr1::DSI_ISR1_SPEC>,
    #[doc = "0xc4 - DSI Host Interrupt Enable Register 0"]
    pub dsi_ier0: crate::Reg<dsi_ier0::DSI_IER0_SPEC>,
    #[doc = "0xc8 - DSI Host Interrupt Enable Register 1"]
    pub dsi_ier1: crate::Reg<dsi_ier1::DSI_IER1_SPEC>,
    _reserved44: [u8; 0x0c],
    #[doc = "0xd8 - DSI Host Force Interrupt Register 0"]
    pub dsi_fir0: crate::Reg<dsi_fir0::DSI_FIR0_SPEC>,
    #[doc = "0xdc - DSI Host Force Interrupt Register 1"]
    pub dsi_fir1: crate::Reg<dsi_fir1::DSI_FIR1_SPEC>,
    _reserved46: [u8; 0x20],
    #[doc = "0x100 - DSI Host Video Shadow Control Register"]
    pub dsi_vscr: crate::Reg<dsi_vscr::DSI_VSCR_SPEC>,
    _reserved47: [u8; 0x08],
    #[doc = "0x10c - DSI Host LTDC Current VCID Register"]
    pub dsi_lcvcidr: crate::Reg<dsi_lcvcidr::DSI_LCVCIDR_SPEC>,
    #[doc = "0x110 - DSI Host LTDC Current Color Coding Register"]
    pub dsi_lcccr: crate::Reg<dsi_lcccr::DSI_LCCCR_SPEC>,
    _reserved49: [u8; 0x04],
    #[doc = "0x118 - DSI Host Low-Power mode Current Configuration Register"]
    pub dsi_lpmccr: crate::Reg<dsi_lpmccr::DSI_LPMCCR_SPEC>,
    _reserved50: [u8; 0x1c],
    #[doc = "0x138 - DSI Host Video mode Current Configuration Register"]
    pub dsi_vmccr: crate::Reg<dsi_vmccr::DSI_VMCCR_SPEC>,
    #[doc = "0x13c - DSI Host Video Packet Current Configuration Register"]
    pub dsi_vpccr: crate::Reg<dsi_vpccr::DSI_VPCCR_SPEC>,
    #[doc = "0x140 - DSI Host Video Chunks Current Configuration Register"]
    pub dsi_vcccr: crate::Reg<dsi_vcccr::DSI_VCCCR_SPEC>,
    #[doc = "0x144 - DSI Host Video Null Packet Current Configuration Register"]
    pub dsi_vnpccr: crate::Reg<dsi_vnpccr::DSI_VNPCCR_SPEC>,
    #[doc = "0x148 - DSI Host Video HSA Current Configuration Register"]
    pub dsi_vhsaccr: crate::Reg<dsi_vhsaccr::DSI_VHSACCR_SPEC>,
    #[doc = "0x14c - DSI Host Video HBP Current Configuration Register"]
    pub dsi_vhbpccr: crate::Reg<dsi_vhbpccr::DSI_VHBPCCR_SPEC>,
    #[doc = "0x150 - DSI Host Video Line Current Configuration Register"]
    pub dsi_vlccr: crate::Reg<dsi_vlccr::DSI_VLCCR_SPEC>,
    #[doc = "0x154 - DSI Host Video VSA Current Configuration Register"]
    pub dsi_vvsaccr: crate::Reg<dsi_vvsaccr::DSI_VVSACCR_SPEC>,
    #[doc = "0x158 - DSI Host Video VBP Current Configuration Register"]
    pub dsi_vvbpccr: crate::Reg<dsi_vvbpccr::DSI_VVBPCCR_SPEC>,
    #[doc = "0x15c - DSI Host Video VFP Current Configuration Register"]
    pub dsi_vvfpccr: crate::Reg<dsi_vvfpccr::DSI_VVFPCCR_SPEC>,
    #[doc = "0x160 - DSI Host Video VA Current Configuration Register"]
    pub dsi_vvaccr: crate::Reg<dsi_vvaccr::DSI_VVACCR_SPEC>,
    _reserved61: [u8; 0x029c],
    #[doc = "0x400 - DSI Wrapper Configuration Register"]
    pub dsi_wcfgr: crate::Reg<dsi_wcfgr::DSI_WCFGR_SPEC>,
    #[doc = "0x404 - DSI Wrapper Control Register"]
    pub dsi_wcr: crate::Reg<dsi_wcr::DSI_WCR_SPEC>,
    #[doc = "0x408 - DSI Wrapper Interrupt Enable Register"]
    pub dsi_wier: crate::Reg<dsi_wier::DSI_WIER_SPEC>,
    #[doc = "0x40c - DSI Wrapper Interrupt & Status Register"]
    pub dsi_wisr: crate::Reg<dsi_wisr::DSI_WISR_SPEC>,
    #[doc = "0x410 - DSI Wrapper Interrupt Flag Clear Register"]
    pub dsi_wifcr: crate::Reg<dsi_wifcr::DSI_WIFCR_SPEC>,
    _reserved66: [u8; 0x04],
    #[doc = "0x418 - DSI Wrapper PHY Configuration Register 1"]
    pub dsi_wpcr1: crate::Reg<dsi_wpcr1::DSI_WPCR1_SPEC>,
    #[doc = "0x41c - DSI Wrapper PHY Configuration Register 2"]
    pub dsi_wpcr2: crate::Reg<dsi_wpcr2::DSI_WPCR2_SPEC>,
    #[doc = "0x420 - DSI Wrapper PHY Configuration Register 3"]
    pub dsi_wpcr3: crate::Reg<dsi_wpcr3::DSI_WPCR3_SPEC>,
    #[doc = "0x424 - DSI_WPCR4"]
    pub dsi_wpcr4: crate::Reg<dsi_wpcr4::DSI_WPCR4_SPEC>,
    #[doc = "0x428 - DSI Wrapper PHY Configuration Register 5"]
    pub dsi_wpcr5: crate::Reg<dsi_wpcr5::DSI_WPCR5_SPEC>,
    _reserved71: [u8; 0x04],
    #[doc = "0x430 - DSI Wrapper Regulator and PLL Control Register"]
    pub dsi_wrpcr: crate::Reg<dsi_wrpcr::DSI_WRPCR_SPEC>,
}
#[doc = "DSI_VR register accessor: an alias for `Reg<DSI_VR_SPEC>`"]
pub type DSI_VR = crate::Reg<dsi_vr::DSI_VR_SPEC>;
#[doc = "DSI Host Version Register"]
pub mod dsi_vr;
#[doc = "DSI_CR register accessor: an alias for `Reg<DSI_CR_SPEC>`"]
pub type DSI_CR = crate::Reg<dsi_cr::DSI_CR_SPEC>;
#[doc = "DSI Host Control Register"]
pub mod dsi_cr;
#[doc = "DSI_CCR register accessor: an alias for `Reg<DSI_CCR_SPEC>`"]
pub type DSI_CCR = crate::Reg<dsi_ccr::DSI_CCR_SPEC>;
#[doc = "DSI HOST Clock Control Register"]
pub mod dsi_ccr;
#[doc = "DSI_LVCIDR register accessor: an alias for `Reg<DSI_LVCIDR_SPEC>`"]
pub type DSI_LVCIDR = crate::Reg<dsi_lvcidr::DSI_LVCIDR_SPEC>;
#[doc = "DSI Host LTDC VCID Register"]
pub mod dsi_lvcidr;
#[doc = "DSI_LCOLCR register accessor: an alias for `Reg<DSI_LCOLCR_SPEC>`"]
pub type DSI_LCOLCR = crate::Reg<dsi_lcolcr::DSI_LCOLCR_SPEC>;
#[doc = "DSI Host LTDC Color Coding Register"]
pub mod dsi_lcolcr;
#[doc = "DSI_LPCR register accessor: an alias for `Reg<DSI_LPCR_SPEC>`"]
pub type DSI_LPCR = crate::Reg<dsi_lpcr::DSI_LPCR_SPEC>;
#[doc = "DSI Host LTDC Polarity Configuration Register"]
pub mod dsi_lpcr;
#[doc = "DSI_LPMCR register accessor: an alias for `Reg<DSI_LPMCR_SPEC>`"]
pub type DSI_LPMCR = crate::Reg<dsi_lpmcr::DSI_LPMCR_SPEC>;
#[doc = "DSI Host Low-Power mode Configuration Register"]
pub mod dsi_lpmcr;
#[doc = "DSI_PCR register accessor: an alias for `Reg<DSI_PCR_SPEC>`"]
pub type DSI_PCR = crate::Reg<dsi_pcr::DSI_PCR_SPEC>;
#[doc = "DSI Host Protocol Configuration Register"]
pub mod dsi_pcr;
#[doc = "DSI_GVCIDR register accessor: an alias for `Reg<DSI_GVCIDR_SPEC>`"]
pub type DSI_GVCIDR = crate::Reg<dsi_gvcidr::DSI_GVCIDR_SPEC>;
#[doc = "DSI Host Generic VCID Register"]
pub mod dsi_gvcidr;
#[doc = "DSI_MCR register accessor: an alias for `Reg<DSI_MCR_SPEC>`"]
pub type DSI_MCR = crate::Reg<dsi_mcr::DSI_MCR_SPEC>;
#[doc = "DSI Host mode Configuration Register"]
pub mod dsi_mcr;
#[doc = "DSI_VMCR register accessor: an alias for `Reg<DSI_VMCR_SPEC>`"]
pub type DSI_VMCR = crate::Reg<dsi_vmcr::DSI_VMCR_SPEC>;
#[doc = "DSI Host Video mode Configuration Register"]
pub mod dsi_vmcr;
#[doc = "DSI_VPCR register accessor: an alias for `Reg<DSI_VPCR_SPEC>`"]
pub type DSI_VPCR = crate::Reg<dsi_vpcr::DSI_VPCR_SPEC>;
#[doc = "DSI Host Video Packet Configuration Register"]
pub mod dsi_vpcr;
#[doc = "DSI_VCCR register accessor: an alias for `Reg<DSI_VCCR_SPEC>`"]
pub type DSI_VCCR = crate::Reg<dsi_vccr::DSI_VCCR_SPEC>;
#[doc = "DSI Host Video Chunks Configuration Register"]
pub mod dsi_vccr;
#[doc = "DSI_VNPCR register accessor: an alias for `Reg<DSI_VNPCR_SPEC>`"]
pub type DSI_VNPCR = crate::Reg<dsi_vnpcr::DSI_VNPCR_SPEC>;
#[doc = "DSI Host Video Null Packet Configuration Register"]
pub mod dsi_vnpcr;
#[doc = "DSI_VHSACR register accessor: an alias for `Reg<DSI_VHSACR_SPEC>`"]
pub type DSI_VHSACR = crate::Reg<dsi_vhsacr::DSI_VHSACR_SPEC>;
#[doc = "DSI Host Video HSA Configuration Register"]
pub mod dsi_vhsacr;
#[doc = "DSI_VHBPCR register accessor: an alias for `Reg<DSI_VHBPCR_SPEC>`"]
pub type DSI_VHBPCR = crate::Reg<dsi_vhbpcr::DSI_VHBPCR_SPEC>;
#[doc = "DSI Host Video HBP Configuration Register"]
pub mod dsi_vhbpcr;
#[doc = "DSI_VLCR register accessor: an alias for `Reg<DSI_VLCR_SPEC>`"]
pub type DSI_VLCR = crate::Reg<dsi_vlcr::DSI_VLCR_SPEC>;
#[doc = "DSI Host Video Line Configuration Register"]
pub mod dsi_vlcr;
#[doc = "DSI_VVSACR register accessor: an alias for `Reg<DSI_VVSACR_SPEC>`"]
pub type DSI_VVSACR = crate::Reg<dsi_vvsacr::DSI_VVSACR_SPEC>;
#[doc = "DSI Host Video VSA Configuration Register"]
pub mod dsi_vvsacr;
#[doc = "DSI_VVBPCR register accessor: an alias for `Reg<DSI_VVBPCR_SPEC>`"]
pub type DSI_VVBPCR = crate::Reg<dsi_vvbpcr::DSI_VVBPCR_SPEC>;
#[doc = "DSI Host Video VBP Configuration Register"]
pub mod dsi_vvbpcr;
#[doc = "DSI_VVFPCR register accessor: an alias for `Reg<DSI_VVFPCR_SPEC>`"]
pub type DSI_VVFPCR = crate::Reg<dsi_vvfpcr::DSI_VVFPCR_SPEC>;
#[doc = "DSI Host Video VFP Configuration Register"]
pub mod dsi_vvfpcr;
#[doc = "DSI_VVACR register accessor: an alias for `Reg<DSI_VVACR_SPEC>`"]
pub type DSI_VVACR = crate::Reg<dsi_vvacr::DSI_VVACR_SPEC>;
#[doc = "DSI Host Video VA Configuration Register"]
pub mod dsi_vvacr;
#[doc = "DSI_LCCR register accessor: an alias for `Reg<DSI_LCCR_SPEC>`"]
pub type DSI_LCCR = crate::Reg<dsi_lccr::DSI_LCCR_SPEC>;
#[doc = "DSI Host LTDC Command Configuration Register"]
pub mod dsi_lccr;
#[doc = "DSI_CMCR register accessor: an alias for `Reg<DSI_CMCR_SPEC>`"]
pub type DSI_CMCR = crate::Reg<dsi_cmcr::DSI_CMCR_SPEC>;
#[doc = "DSI Host Command mode Configuration Register"]
pub mod dsi_cmcr;
#[doc = "DSI_GHCR register accessor: an alias for `Reg<DSI_GHCR_SPEC>`"]
pub type DSI_GHCR = crate::Reg<dsi_ghcr::DSI_GHCR_SPEC>;
#[doc = "DSI Host Generic Header Configuration Register"]
pub mod dsi_ghcr;
#[doc = "DSI_GPDR register accessor: an alias for `Reg<DSI_GPDR_SPEC>`"]
pub type DSI_GPDR = crate::Reg<dsi_gpdr::DSI_GPDR_SPEC>;
#[doc = "DSI Host Generic Payload Data Register"]
pub mod dsi_gpdr;
#[doc = "DSI_GPSR register accessor: an alias for `Reg<DSI_GPSR_SPEC>`"]
pub type DSI_GPSR = crate::Reg<dsi_gpsr::DSI_GPSR_SPEC>;
#[doc = "DSI Host Generic Packet Status Register"]
pub mod dsi_gpsr;
#[doc = "DSI_TCCR0 register accessor: an alias for `Reg<DSI_TCCR0_SPEC>`"]
pub type DSI_TCCR0 = crate::Reg<dsi_tccr0::DSI_TCCR0_SPEC>;
#[doc = "DSI Host Timeout Counter Configuration Register 0"]
pub mod dsi_tccr0;
#[doc = "DSI_TCCR1 register accessor: an alias for `Reg<DSI_TCCR1_SPEC>`"]
pub type DSI_TCCR1 = crate::Reg<dsi_tccr1::DSI_TCCR1_SPEC>;
#[doc = "DSI Host Timeout Counter Configuration Register 1"]
pub mod dsi_tccr1;
#[doc = "DSI_TCCR2 register accessor: an alias for `Reg<DSI_TCCR2_SPEC>`"]
pub type DSI_TCCR2 = crate::Reg<dsi_tccr2::DSI_TCCR2_SPEC>;
#[doc = "DSI Host Timeout Counter Configuration Register 2"]
pub mod dsi_tccr2;
#[doc = "DSI_TCCR3 register accessor: an alias for `Reg<DSI_TCCR3_SPEC>`"]
pub type DSI_TCCR3 = crate::Reg<dsi_tccr3::DSI_TCCR3_SPEC>;
#[doc = "DSI Host Timeout Counter Configuration Register 3"]
pub mod dsi_tccr3;
#[doc = "DSI_TCCR4 register accessor: an alias for `Reg<DSI_TCCR4_SPEC>`"]
pub type DSI_TCCR4 = crate::Reg<dsi_tccr4::DSI_TCCR4_SPEC>;
#[doc = "DSI Host Timeout Counter Configuration Register 4"]
pub mod dsi_tccr4;
#[doc = "DSI_TCCR5 register accessor: an alias for `Reg<DSI_TCCR5_SPEC>`"]
pub type DSI_TCCR5 = crate::Reg<dsi_tccr5::DSI_TCCR5_SPEC>;
#[doc = "DSI Host Timeout Counter Configuration Register 5"]
pub mod dsi_tccr5;
#[doc = "DSI_CLCR register accessor: an alias for `Reg<DSI_CLCR_SPEC>`"]
pub type DSI_CLCR = crate::Reg<dsi_clcr::DSI_CLCR_SPEC>;
#[doc = "DSI Host Clock Lane Configuration Register"]
pub mod dsi_clcr;
#[doc = "DSI_CLTCR register accessor: an alias for `Reg<DSI_CLTCR_SPEC>`"]
pub type DSI_CLTCR = crate::Reg<dsi_cltcr::DSI_CLTCR_SPEC>;
#[doc = "DSI Host Clock Lane Timer Configuration Register"]
pub mod dsi_cltcr;
#[doc = "DSI_DLTCR register accessor: an alias for `Reg<DSI_DLTCR_SPEC>`"]
pub type DSI_DLTCR = crate::Reg<dsi_dltcr::DSI_DLTCR_SPEC>;
#[doc = "DSI Host Data Lane Timer Configuration Register"]
pub mod dsi_dltcr;
#[doc = "DSI_PCTLR register accessor: an alias for `Reg<DSI_PCTLR_SPEC>`"]
pub type DSI_PCTLR = crate::Reg<dsi_pctlr::DSI_PCTLR_SPEC>;
#[doc = "DSI Host PHY Control Register"]
pub mod dsi_pctlr;
#[doc = "DSI_PCONFR register accessor: an alias for `Reg<DSI_PCONFR_SPEC>`"]
pub type DSI_PCONFR = crate::Reg<dsi_pconfr::DSI_PCONFR_SPEC>;
#[doc = "DSI Host PHY Configuration Register"]
pub mod dsi_pconfr;
#[doc = "DSI_PUCR register accessor: an alias for `Reg<DSI_PUCR_SPEC>`"]
pub type DSI_PUCR = crate::Reg<dsi_pucr::DSI_PUCR_SPEC>;
#[doc = "DSI Host PHY ULPS Control Register"]
pub mod dsi_pucr;
#[doc = "DSI_PTTCR register accessor: an alias for `Reg<DSI_PTTCR_SPEC>`"]
pub type DSI_PTTCR = crate::Reg<dsi_pttcr::DSI_PTTCR_SPEC>;
#[doc = "DSI Host PHY TX Triggers Configuration Register"]
pub mod dsi_pttcr;
#[doc = "DSI_PSR register accessor: an alias for `Reg<DSI_PSR_SPEC>`"]
pub type DSI_PSR = crate::Reg<dsi_psr::DSI_PSR_SPEC>;
#[doc = "DSI Host PHY Status Register"]
pub mod dsi_psr;
#[doc = "DSI_ISR0 register accessor: an alias for `Reg<DSI_ISR0_SPEC>`"]
pub type DSI_ISR0 = crate::Reg<dsi_isr0::DSI_ISR0_SPEC>;
#[doc = "DSI Host Interrupt & Status Register 0"]
pub mod dsi_isr0;
#[doc = "DSI_ISR1 register accessor: an alias for `Reg<DSI_ISR1_SPEC>`"]
pub type DSI_ISR1 = crate::Reg<dsi_isr1::DSI_ISR1_SPEC>;
#[doc = "DSI Host Interrupt & Status Register 1"]
pub mod dsi_isr1;
#[doc = "DSI_IER0 register accessor: an alias for `Reg<DSI_IER0_SPEC>`"]
pub type DSI_IER0 = crate::Reg<dsi_ier0::DSI_IER0_SPEC>;
#[doc = "DSI Host Interrupt Enable Register 0"]
pub mod dsi_ier0;
#[doc = "DSI_IER1 register accessor: an alias for `Reg<DSI_IER1_SPEC>`"]
pub type DSI_IER1 = crate::Reg<dsi_ier1::DSI_IER1_SPEC>;
#[doc = "DSI Host Interrupt Enable Register 1"]
pub mod dsi_ier1;
#[doc = "DSI_FIR0 register accessor: an alias for `Reg<DSI_FIR0_SPEC>`"]
pub type DSI_FIR0 = crate::Reg<dsi_fir0::DSI_FIR0_SPEC>;
#[doc = "DSI Host Force Interrupt Register 0"]
pub mod dsi_fir0;
#[doc = "DSI_FIR1 register accessor: an alias for `Reg<DSI_FIR1_SPEC>`"]
pub type DSI_FIR1 = crate::Reg<dsi_fir1::DSI_FIR1_SPEC>;
#[doc = "DSI Host Force Interrupt Register 1"]
pub mod dsi_fir1;
#[doc = "DSI_VSCR register accessor: an alias for `Reg<DSI_VSCR_SPEC>`"]
pub type DSI_VSCR = crate::Reg<dsi_vscr::DSI_VSCR_SPEC>;
#[doc = "DSI Host Video Shadow Control Register"]
pub mod dsi_vscr;
#[doc = "DSI_LCVCIDR register accessor: an alias for `Reg<DSI_LCVCIDR_SPEC>`"]
pub type DSI_LCVCIDR = crate::Reg<dsi_lcvcidr::DSI_LCVCIDR_SPEC>;
#[doc = "DSI Host LTDC Current VCID Register"]
pub mod dsi_lcvcidr;
#[doc = "DSI_LCCCR register accessor: an alias for `Reg<DSI_LCCCR_SPEC>`"]
pub type DSI_LCCCR = crate::Reg<dsi_lcccr::DSI_LCCCR_SPEC>;
#[doc = "DSI Host LTDC Current Color Coding Register"]
pub mod dsi_lcccr;
#[doc = "DSI_LPMCCR register accessor: an alias for `Reg<DSI_LPMCCR_SPEC>`"]
pub type DSI_LPMCCR = crate::Reg<dsi_lpmccr::DSI_LPMCCR_SPEC>;
#[doc = "DSI Host Low-Power mode Current Configuration Register"]
pub mod dsi_lpmccr;
#[doc = "DSI_VMCCR register accessor: an alias for `Reg<DSI_VMCCR_SPEC>`"]
pub type DSI_VMCCR = crate::Reg<dsi_vmccr::DSI_VMCCR_SPEC>;
#[doc = "DSI Host Video mode Current Configuration Register"]
pub mod dsi_vmccr;
#[doc = "DSI_VPCCR register accessor: an alias for `Reg<DSI_VPCCR_SPEC>`"]
pub type DSI_VPCCR = crate::Reg<dsi_vpccr::DSI_VPCCR_SPEC>;
#[doc = "DSI Host Video Packet Current Configuration Register"]
pub mod dsi_vpccr;
#[doc = "DSI_VCCCR register accessor: an alias for `Reg<DSI_VCCCR_SPEC>`"]
pub type DSI_VCCCR = crate::Reg<dsi_vcccr::DSI_VCCCR_SPEC>;
#[doc = "DSI Host Video Chunks Current Configuration Register"]
pub mod dsi_vcccr;
#[doc = "DSI_VNPCCR register accessor: an alias for `Reg<DSI_VNPCCR_SPEC>`"]
pub type DSI_VNPCCR = crate::Reg<dsi_vnpccr::DSI_VNPCCR_SPEC>;
#[doc = "DSI Host Video Null Packet Current Configuration Register"]
pub mod dsi_vnpccr;
#[doc = "DSI_VHSACCR register accessor: an alias for `Reg<DSI_VHSACCR_SPEC>`"]
pub type DSI_VHSACCR = crate::Reg<dsi_vhsaccr::DSI_VHSACCR_SPEC>;
#[doc = "DSI Host Video HSA Current Configuration Register"]
pub mod dsi_vhsaccr;
#[doc = "DSI_VHBPCCR register accessor: an alias for `Reg<DSI_VHBPCCR_SPEC>`"]
pub type DSI_VHBPCCR = crate::Reg<dsi_vhbpccr::DSI_VHBPCCR_SPEC>;
#[doc = "DSI Host Video HBP Current Configuration Register"]
pub mod dsi_vhbpccr;
#[doc = "DSI_VLCCR register accessor: an alias for `Reg<DSI_VLCCR_SPEC>`"]
pub type DSI_VLCCR = crate::Reg<dsi_vlccr::DSI_VLCCR_SPEC>;
#[doc = "DSI Host Video Line Current Configuration Register"]
pub mod dsi_vlccr;
#[doc = "DSI_VVSACCR register accessor: an alias for `Reg<DSI_VVSACCR_SPEC>`"]
pub type DSI_VVSACCR = crate::Reg<dsi_vvsaccr::DSI_VVSACCR_SPEC>;
#[doc = "DSI Host Video VSA Current Configuration Register"]
pub mod dsi_vvsaccr;
#[doc = "DSI_VVBPCCR register accessor: an alias for `Reg<DSI_VVBPCCR_SPEC>`"]
pub type DSI_VVBPCCR = crate::Reg<dsi_vvbpccr::DSI_VVBPCCR_SPEC>;
#[doc = "DSI Host Video VBP Current Configuration Register"]
pub mod dsi_vvbpccr;
#[doc = "DSI_VVFPCCR register accessor: an alias for `Reg<DSI_VVFPCCR_SPEC>`"]
pub type DSI_VVFPCCR = crate::Reg<dsi_vvfpccr::DSI_VVFPCCR_SPEC>;
#[doc = "DSI Host Video VFP Current Configuration Register"]
pub mod dsi_vvfpccr;
#[doc = "DSI_VVACCR register accessor: an alias for `Reg<DSI_VVACCR_SPEC>`"]
pub type DSI_VVACCR = crate::Reg<dsi_vvaccr::DSI_VVACCR_SPEC>;
#[doc = "DSI Host Video VA Current Configuration Register"]
pub mod dsi_vvaccr;
#[doc = "DSI_WCFGR register accessor: an alias for `Reg<DSI_WCFGR_SPEC>`"]
pub type DSI_WCFGR = crate::Reg<dsi_wcfgr::DSI_WCFGR_SPEC>;
#[doc = "DSI Wrapper Configuration Register"]
pub mod dsi_wcfgr;
#[doc = "DSI_WCR register accessor: an alias for `Reg<DSI_WCR_SPEC>`"]
pub type DSI_WCR = crate::Reg<dsi_wcr::DSI_WCR_SPEC>;
#[doc = "DSI Wrapper Control Register"]
pub mod dsi_wcr;
#[doc = "DSI_WIER register accessor: an alias for `Reg<DSI_WIER_SPEC>`"]
pub type DSI_WIER = crate::Reg<dsi_wier::DSI_WIER_SPEC>;
#[doc = "DSI Wrapper Interrupt Enable Register"]
pub mod dsi_wier;
#[doc = "DSI_WISR register accessor: an alias for `Reg<DSI_WISR_SPEC>`"]
pub type DSI_WISR = crate::Reg<dsi_wisr::DSI_WISR_SPEC>;
#[doc = "DSI Wrapper Interrupt & Status Register"]
pub mod dsi_wisr;
#[doc = "DSI_WIFCR register accessor: an alias for `Reg<DSI_WIFCR_SPEC>`"]
pub type DSI_WIFCR = crate::Reg<dsi_wifcr::DSI_WIFCR_SPEC>;
#[doc = "DSI Wrapper Interrupt Flag Clear Register"]
pub mod dsi_wifcr;
#[doc = "DSI_WPCR1 register accessor: an alias for `Reg<DSI_WPCR1_SPEC>`"]
pub type DSI_WPCR1 = crate::Reg<dsi_wpcr1::DSI_WPCR1_SPEC>;
#[doc = "DSI Wrapper PHY Configuration Register 1"]
pub mod dsi_wpcr1;
#[doc = "DSI_WPCR2 register accessor: an alias for `Reg<DSI_WPCR2_SPEC>`"]
pub type DSI_WPCR2 = crate::Reg<dsi_wpcr2::DSI_WPCR2_SPEC>;
#[doc = "DSI Wrapper PHY Configuration Register 2"]
pub mod dsi_wpcr2;
#[doc = "DSI_WPCR3 register accessor: an alias for `Reg<DSI_WPCR3_SPEC>`"]
pub type DSI_WPCR3 = crate::Reg<dsi_wpcr3::DSI_WPCR3_SPEC>;
#[doc = "DSI Wrapper PHY Configuration Register 3"]
pub mod dsi_wpcr3;
#[doc = "DSI_WPCR4 register accessor: an alias for `Reg<DSI_WPCR4_SPEC>`"]
pub type DSI_WPCR4 = crate::Reg<dsi_wpcr4::DSI_WPCR4_SPEC>;
#[doc = "DSI_WPCR4"]
pub mod dsi_wpcr4;
#[doc = "DSI_WPCR5 register accessor: an alias for `Reg<DSI_WPCR5_SPEC>`"]
pub type DSI_WPCR5 = crate::Reg<dsi_wpcr5::DSI_WPCR5_SPEC>;
#[doc = "DSI Wrapper PHY Configuration Register 5"]
pub mod dsi_wpcr5;
#[doc = "DSI_WRPCR register accessor: an alias for `Reg<DSI_WRPCR_SPEC>`"]
pub type DSI_WRPCR = crate::Reg<dsi_wrpcr::DSI_WRPCR_SPEC>;
#[doc = "DSI Wrapper Regulator and PLL Control Register"]
pub mod dsi_wrpcr;
