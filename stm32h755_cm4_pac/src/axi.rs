#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    _reserved0: [u8; 0x1fd0],
    axi_periph_id_4: AxiPeriphId4,
    _reserved1: [u8; 0x0c],
    axi_periph_id_0: AxiPeriphId0,
    axi_periph_id_1: AxiPeriphId1,
    axi_periph_id_2: AxiPeriphId2,
    axi_periph_id_3: AxiPeriphId3,
    axi_comp_id_0: AxiCompId0,
    axi_comp_id_1: AxiCompId1,
    axi_comp_id_2: AxiCompId2,
    axi_comp_id_3: AxiCompId3,
    _reserved9: [u8; 0x08],
    axi_targ1_fn_mod_iss_bm: AxiTarg1FnModIssBm,
    _reserved10: [u8; 0x18],
    axi_targ1_fn_mod2: AxiTarg1FnMod2,
    _reserved11: [u8; 0x04],
    axi_targ1_fn_mod_lb: AxiTarg1FnModLb,
    _reserved12: [u8; 0xd8],
    axi_targ1_fn_mod: AxiTarg1FnMod,
    _reserved13: [u8; 0x0efc],
    axi_targ2_fn_mod_iss_bm: AxiTarg2FnModIssBm,
    _reserved14: [u8; 0x18],
    axi_targ2_fn_mod2: AxiTarg2FnMod2,
    _reserved15: [u8; 0x04],
    axi_targ2_fn_mod_lb: AxiTarg2FnModLb,
    _reserved16: [u8; 0xd8],
    axi_targ2_fn_mod: AxiTarg2FnMod,
    _reserved17: [u8; 0x0efc],
    axi_targ3_fn_mod_iss_bm: AxiTarg3FnModIssBm,
    _reserved18: [u8; 0x0ffc],
    axi_targ4_fn_mod_iss_bm: AxiTarg4FnModIssBm,
    _reserved19: [u8; 0x0ffc],
    axi_targ5_fn_mod_iss_bm: AxiTarg5FnModIssBm,
    _reserved20: [u8; 0x0ffc],
    axi_targ6_fn_mod_iss_bm: AxiTarg6FnModIssBm,
    _reserved21: [u8; 0x1000],
    axi_targ7_fn_mod_iss_bm: AxiTarg7FnModIssBm,
    _reserved22: [u8; 0x14],
    axi_targ7_fn_mod2: AxiTarg7FnMod2,
    _reserved23: [u8; 0xe0],
    axi_targ7_fn_mod: AxiTarg7FnMod,
    _reserved24: [u8; 0x0003_9f18],
    axi_ini1_fn_mod2: AxiIni1FnMod2,
    axi_ini1_fn_mod_ahb: AxiIni1FnModAhb,
    _reserved26: [u8; 0xd4],
    axi_ini1_read_qos: AxiIni1ReadQos,
    axi_ini1_write_qos: AxiIni1WriteQos,
    axi_ini1_fn_mod: AxiIni1FnMod,
    _reserved29: [u8; 0x0ff4],
    axi_ini2_read_qos: AxiIni2ReadQos,
    axi_ini2_write_qos: AxiIni2WriteQos,
    axi_ini2_fn_mod: AxiIni2FnMod,
    _reserved32: [u8; 0x0f18],
    axi_ini3_fn_mod2: AxiIni3FnMod2,
    axi_ini3_fn_mod_ahb: AxiIni3FnModAhb,
    _reserved34: [u8; 0xd4],
    axi_ini3_read_qos: AxiIni3ReadQos,
    axi_ini3_write_qos: AxiIni3WriteQos,
    axi_ini3_fn_mod: AxiIni3FnMod,
    _reserved37: [u8; 0x0ff4],
    axi_ini4_read_qos: AxiIni4ReadQos,
    axi_ini4_write_qos: AxiIni4WriteQos,
    axi_ini4_fn_mod: AxiIni4FnMod,
    _reserved40: [u8; 0x0ff4],
    axi_ini5_read_qos: AxiIni5ReadQos,
    axi_ini5_write_qos: AxiIni5WriteQos,
    axi_ini5_fn_mod: AxiIni5FnMod,
    _reserved43: [u8; 0x0ff4],
    axi_ini6_read_qos: AxiIni6ReadQos,
    axi_ini6_write_qos: AxiIni6WriteQos,
    axi_ini6_fn_mod: AxiIni6FnMod,
}
impl RegisterBlock {
    #[doc = "0x1fd0 - AXI interconnect - peripheral ID4 register"]
    #[inline(always)]
    pub const fn axi_periph_id_4(&self) -> &AxiPeriphId4 {
        &self.axi_periph_id_4
    }
    #[doc = "0x1fe0 - AXI interconnect - peripheral ID0 register"]
    #[inline(always)]
    pub const fn axi_periph_id_0(&self) -> &AxiPeriphId0 {
        &self.axi_periph_id_0
    }
    #[doc = "0x1fe4 - AXI interconnect - peripheral ID1 register"]
    #[inline(always)]
    pub const fn axi_periph_id_1(&self) -> &AxiPeriphId1 {
        &self.axi_periph_id_1
    }
    #[doc = "0x1fe8 - AXI interconnect - peripheral ID2 register"]
    #[inline(always)]
    pub const fn axi_periph_id_2(&self) -> &AxiPeriphId2 {
        &self.axi_periph_id_2
    }
    #[doc = "0x1fec - AXI interconnect - peripheral ID3 register"]
    #[inline(always)]
    pub const fn axi_periph_id_3(&self) -> &AxiPeriphId3 {
        &self.axi_periph_id_3
    }
    #[doc = "0x1ff0 - AXI interconnect - component ID0 register"]
    #[inline(always)]
    pub const fn axi_comp_id_0(&self) -> &AxiCompId0 {
        &self.axi_comp_id_0
    }
    #[doc = "0x1ff4 - AXI interconnect - component ID1 register"]
    #[inline(always)]
    pub const fn axi_comp_id_1(&self) -> &AxiCompId1 {
        &self.axi_comp_id_1
    }
    #[doc = "0x1ff8 - AXI interconnect - component ID2 register"]
    #[inline(always)]
    pub const fn axi_comp_id_2(&self) -> &AxiCompId2 {
        &self.axi_comp_id_2
    }
    #[doc = "0x1ffc - AXI interconnect - component ID3 register"]
    #[inline(always)]
    pub const fn axi_comp_id_3(&self) -> &AxiCompId3 {
        &self.axi_comp_id_3
    }
    #[doc = "0x2008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    #[inline(always)]
    pub const fn axi_targ1_fn_mod_iss_bm(&self) -> &AxiTarg1FnModIssBm {
        &self.axi_targ1_fn_mod_iss_bm
    }
    #[doc = "0x2024 - AXI interconnect - TARG x bus matrix functionality 2 register"]
    #[inline(always)]
    pub const fn axi_targ1_fn_mod2(&self) -> &AxiTarg1FnMod2 {
        &self.axi_targ1_fn_mod2
    }
    #[doc = "0x202c - AXI interconnect - TARG x long burst functionality modification"]
    #[inline(always)]
    pub const fn axi_targ1_fn_mod_lb(&self) -> &AxiTarg1FnModLb {
        &self.axi_targ1_fn_mod_lb
    }
    #[doc = "0x2108 - AXI interconnect - TARG x long burst functionality modification"]
    #[inline(always)]
    pub const fn axi_targ1_fn_mod(&self) -> &AxiTarg1FnMod {
        &self.axi_targ1_fn_mod
    }
    #[doc = "0x3008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    #[inline(always)]
    pub const fn axi_targ2_fn_mod_iss_bm(&self) -> &AxiTarg2FnModIssBm {
        &self.axi_targ2_fn_mod_iss_bm
    }
    #[doc = "0x3024 - AXI interconnect - TARG x bus matrix functionality 2 register"]
    #[inline(always)]
    pub const fn axi_targ2_fn_mod2(&self) -> &AxiTarg2FnMod2 {
        &self.axi_targ2_fn_mod2
    }
    #[doc = "0x302c - AXI interconnect - TARG x long burst functionality modification"]
    #[inline(always)]
    pub const fn axi_targ2_fn_mod_lb(&self) -> &AxiTarg2FnModLb {
        &self.axi_targ2_fn_mod_lb
    }
    #[doc = "0x3108 - AXI interconnect - TARG x long burst functionality modification"]
    #[inline(always)]
    pub const fn axi_targ2_fn_mod(&self) -> &AxiTarg2FnMod {
        &self.axi_targ2_fn_mod
    }
    #[doc = "0x4008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    #[inline(always)]
    pub const fn axi_targ3_fn_mod_iss_bm(&self) -> &AxiTarg3FnModIssBm {
        &self.axi_targ3_fn_mod_iss_bm
    }
    #[doc = "0x5008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    #[inline(always)]
    pub const fn axi_targ4_fn_mod_iss_bm(&self) -> &AxiTarg4FnModIssBm {
        &self.axi_targ4_fn_mod_iss_bm
    }
    #[doc = "0x6008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    #[inline(always)]
    pub const fn axi_targ5_fn_mod_iss_bm(&self) -> &AxiTarg5FnModIssBm {
        &self.axi_targ5_fn_mod_iss_bm
    }
    #[doc = "0x7008 - AXI interconnect - TARG x bus matrix issuing functionality register"]
    #[inline(always)]
    pub const fn axi_targ6_fn_mod_iss_bm(&self) -> &AxiTarg6FnModIssBm {
        &self.axi_targ6_fn_mod_iss_bm
    }
    #[doc = "0x800c - AXI interconnect - TARG x bus matrix issuing functionality register"]
    #[inline(always)]
    pub const fn axi_targ7_fn_mod_iss_bm(&self) -> &AxiTarg7FnModIssBm {
        &self.axi_targ7_fn_mod_iss_bm
    }
    #[doc = "0x8024 - AXI interconnect - TARG x bus matrix functionality 2 register"]
    #[inline(always)]
    pub const fn axi_targ7_fn_mod2(&self) -> &AxiTarg7FnMod2 {
        &self.axi_targ7_fn_mod2
    }
    #[doc = "0x8108 - AXI interconnect - TARG x long burst functionality modification"]
    #[inline(always)]
    pub const fn axi_targ7_fn_mod(&self) -> &AxiTarg7FnMod {
        &self.axi_targ7_fn_mod
    }
    #[doc = "0x42024 - AXI interconnect - INI x functionality modification 2 register"]
    #[inline(always)]
    pub const fn axi_ini1_fn_mod2(&self) -> &AxiIni1FnMod2 {
        &self.axi_ini1_fn_mod2
    }
    #[doc = "0x42028 - AXI interconnect - INI x AHB functionality modification register"]
    #[inline(always)]
    pub const fn axi_ini1_fn_mod_ahb(&self) -> &AxiIni1FnModAhb {
        &self.axi_ini1_fn_mod_ahb
    }
    #[doc = "0x42100 - AXI interconnect - INI x read QoS register"]
    #[inline(always)]
    pub const fn axi_ini1_read_qos(&self) -> &AxiIni1ReadQos {
        &self.axi_ini1_read_qos
    }
    #[doc = "0x42104 - AXI interconnect - INI x write QoS register"]
    #[inline(always)]
    pub const fn axi_ini1_write_qos(&self) -> &AxiIni1WriteQos {
        &self.axi_ini1_write_qos
    }
    #[doc = "0x42108 - AXI interconnect - INI x issuing functionality modification register"]
    #[inline(always)]
    pub const fn axi_ini1_fn_mod(&self) -> &AxiIni1FnMod {
        &self.axi_ini1_fn_mod
    }
    #[doc = "0x43100 - AXI interconnect - INI x read QoS register"]
    #[inline(always)]
    pub const fn axi_ini2_read_qos(&self) -> &AxiIni2ReadQos {
        &self.axi_ini2_read_qos
    }
    #[doc = "0x43104 - AXI interconnect - INI x write QoS register"]
    #[inline(always)]
    pub const fn axi_ini2_write_qos(&self) -> &AxiIni2WriteQos {
        &self.axi_ini2_write_qos
    }
    #[doc = "0x43108 - AXI interconnect - INI x issuing functionality modification register"]
    #[inline(always)]
    pub const fn axi_ini2_fn_mod(&self) -> &AxiIni2FnMod {
        &self.axi_ini2_fn_mod
    }
    #[doc = "0x44024 - AXI interconnect - INI x functionality modification 2 register"]
    #[inline(always)]
    pub const fn axi_ini3_fn_mod2(&self) -> &AxiIni3FnMod2 {
        &self.axi_ini3_fn_mod2
    }
    #[doc = "0x44028 - AXI interconnect - INI x AHB functionality modification register"]
    #[inline(always)]
    pub const fn axi_ini3_fn_mod_ahb(&self) -> &AxiIni3FnModAhb {
        &self.axi_ini3_fn_mod_ahb
    }
    #[doc = "0x44100 - AXI interconnect - INI x read QoS register"]
    #[inline(always)]
    pub const fn axi_ini3_read_qos(&self) -> &AxiIni3ReadQos {
        &self.axi_ini3_read_qos
    }
    #[doc = "0x44104 - AXI interconnect - INI x write QoS register"]
    #[inline(always)]
    pub const fn axi_ini3_write_qos(&self) -> &AxiIni3WriteQos {
        &self.axi_ini3_write_qos
    }
    #[doc = "0x44108 - AXI interconnect - INI x issuing functionality modification register"]
    #[inline(always)]
    pub const fn axi_ini3_fn_mod(&self) -> &AxiIni3FnMod {
        &self.axi_ini3_fn_mod
    }
    #[doc = "0x45100 - AXI interconnect - INI x read QoS register"]
    #[inline(always)]
    pub const fn axi_ini4_read_qos(&self) -> &AxiIni4ReadQos {
        &self.axi_ini4_read_qos
    }
    #[doc = "0x45104 - AXI interconnect - INI x write QoS register"]
    #[inline(always)]
    pub const fn axi_ini4_write_qos(&self) -> &AxiIni4WriteQos {
        &self.axi_ini4_write_qos
    }
    #[doc = "0x45108 - AXI interconnect - INI x issuing functionality modification register"]
    #[inline(always)]
    pub const fn axi_ini4_fn_mod(&self) -> &AxiIni4FnMod {
        &self.axi_ini4_fn_mod
    }
    #[doc = "0x46100 - AXI interconnect - INI x read QoS register"]
    #[inline(always)]
    pub const fn axi_ini5_read_qos(&self) -> &AxiIni5ReadQos {
        &self.axi_ini5_read_qos
    }
    #[doc = "0x46104 - AXI interconnect - INI x write QoS register"]
    #[inline(always)]
    pub const fn axi_ini5_write_qos(&self) -> &AxiIni5WriteQos {
        &self.axi_ini5_write_qos
    }
    #[doc = "0x46108 - AXI interconnect - INI x issuing functionality modification register"]
    #[inline(always)]
    pub const fn axi_ini5_fn_mod(&self) -> &AxiIni5FnMod {
        &self.axi_ini5_fn_mod
    }
    #[doc = "0x47100 - AXI interconnect - INI x read QoS register"]
    #[inline(always)]
    pub const fn axi_ini6_read_qos(&self) -> &AxiIni6ReadQos {
        &self.axi_ini6_read_qos
    }
    #[doc = "0x47104 - AXI interconnect - INI x write QoS register"]
    #[inline(always)]
    pub const fn axi_ini6_write_qos(&self) -> &AxiIni6WriteQos {
        &self.axi_ini6_write_qos
    }
    #[doc = "0x47108 - AXI interconnect - INI x issuing functionality modification register"]
    #[inline(always)]
    pub const fn axi_ini6_fn_mod(&self) -> &AxiIni6FnMod {
        &self.axi_ini6_fn_mod
    }
}
#[doc = "AXI_PERIPH_ID_4 (r) register accessor: AXI interconnect - peripheral ID4 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_periph_id_4::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_periph_id_4`] module"]
#[doc(alias = "AXI_PERIPH_ID_4")]
pub type AxiPeriphId4 = crate::Reg<axi_periph_id_4::AxiPeriphId4Spec>;
#[doc = "AXI interconnect - peripheral ID4 register"]
pub mod axi_periph_id_4;
#[doc = "AXI_PERIPH_ID_0 (r) register accessor: AXI interconnect - peripheral ID0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_periph_id_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_periph_id_0`] module"]
#[doc(alias = "AXI_PERIPH_ID_0")]
pub type AxiPeriphId0 = crate::Reg<axi_periph_id_0::AxiPeriphId0Spec>;
#[doc = "AXI interconnect - peripheral ID0 register"]
pub mod axi_periph_id_0;
#[doc = "AXI_PERIPH_ID_1 (r) register accessor: AXI interconnect - peripheral ID1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_periph_id_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_periph_id_1`] module"]
#[doc(alias = "AXI_PERIPH_ID_1")]
pub type AxiPeriphId1 = crate::Reg<axi_periph_id_1::AxiPeriphId1Spec>;
#[doc = "AXI interconnect - peripheral ID1 register"]
pub mod axi_periph_id_1;
#[doc = "AXI_PERIPH_ID_2 (r) register accessor: AXI interconnect - peripheral ID2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_periph_id_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_periph_id_2`] module"]
#[doc(alias = "AXI_PERIPH_ID_2")]
pub type AxiPeriphId2 = crate::Reg<axi_periph_id_2::AxiPeriphId2Spec>;
#[doc = "AXI interconnect - peripheral ID2 register"]
pub mod axi_periph_id_2;
#[doc = "AXI_PERIPH_ID_3 (r) register accessor: AXI interconnect - peripheral ID3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_periph_id_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_periph_id_3`] module"]
#[doc(alias = "AXI_PERIPH_ID_3")]
pub type AxiPeriphId3 = crate::Reg<axi_periph_id_3::AxiPeriphId3Spec>;
#[doc = "AXI interconnect - peripheral ID3 register"]
pub mod axi_periph_id_3;
#[doc = "AXI_COMP_ID_0 (r) register accessor: AXI interconnect - component ID0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_comp_id_0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_comp_id_0`] module"]
#[doc(alias = "AXI_COMP_ID_0")]
pub type AxiCompId0 = crate::Reg<axi_comp_id_0::AxiCompId0Spec>;
#[doc = "AXI interconnect - component ID0 register"]
pub mod axi_comp_id_0;
#[doc = "AXI_COMP_ID_1 (r) register accessor: AXI interconnect - component ID1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_comp_id_1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_comp_id_1`] module"]
#[doc(alias = "AXI_COMP_ID_1")]
pub type AxiCompId1 = crate::Reg<axi_comp_id_1::AxiCompId1Spec>;
#[doc = "AXI interconnect - component ID1 register"]
pub mod axi_comp_id_1;
#[doc = "AXI_COMP_ID_2 (r) register accessor: AXI interconnect - component ID2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_comp_id_2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_comp_id_2`] module"]
#[doc(alias = "AXI_COMP_ID_2")]
pub type AxiCompId2 = crate::Reg<axi_comp_id_2::AxiCompId2Spec>;
#[doc = "AXI interconnect - component ID2 register"]
pub mod axi_comp_id_2;
#[doc = "AXI_COMP_ID_3 (r) register accessor: AXI interconnect - component ID3 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_comp_id_3::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_comp_id_3`] module"]
#[doc(alias = "AXI_COMP_ID_3")]
pub type AxiCompId3 = crate::Reg<axi_comp_id_3::AxiCompId3Spec>;
#[doc = "AXI interconnect - component ID3 register"]
pub mod axi_comp_id_3;
#[doc = "AXI_TARG1_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ1_fn_mod_iss_bm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ1_fn_mod_iss_bm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_targ1_fn_mod_iss_bm`] module"]
#[doc(alias = "AXI_TARG1_FN_MOD_ISS_BM")]
pub type AxiTarg1FnModIssBm = crate::Reg<axi_targ1_fn_mod_iss_bm::AxiTarg1FnModIssBmSpec>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod axi_targ1_fn_mod_iss_bm;
#[doc = "AXI_TARG2_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ2_fn_mod_iss_bm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ2_fn_mod_iss_bm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_targ2_fn_mod_iss_bm`] module"]
#[doc(alias = "AXI_TARG2_FN_MOD_ISS_BM")]
pub type AxiTarg2FnModIssBm = crate::Reg<axi_targ2_fn_mod_iss_bm::AxiTarg2FnModIssBmSpec>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod axi_targ2_fn_mod_iss_bm;
#[doc = "AXI_TARG3_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ3_fn_mod_iss_bm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ3_fn_mod_iss_bm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_targ3_fn_mod_iss_bm`] module"]
#[doc(alias = "AXI_TARG3_FN_MOD_ISS_BM")]
pub type AxiTarg3FnModIssBm = crate::Reg<axi_targ3_fn_mod_iss_bm::AxiTarg3FnModIssBmSpec>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod axi_targ3_fn_mod_iss_bm;
#[doc = "AXI_TARG4_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ4_fn_mod_iss_bm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ4_fn_mod_iss_bm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_targ4_fn_mod_iss_bm`] module"]
#[doc(alias = "AXI_TARG4_FN_MOD_ISS_BM")]
pub type AxiTarg4FnModIssBm = crate::Reg<axi_targ4_fn_mod_iss_bm::AxiTarg4FnModIssBmSpec>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod axi_targ4_fn_mod_iss_bm;
#[doc = "AXI_TARG5_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ5_fn_mod_iss_bm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ5_fn_mod_iss_bm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_targ5_fn_mod_iss_bm`] module"]
#[doc(alias = "AXI_TARG5_FN_MOD_ISS_BM")]
pub type AxiTarg5FnModIssBm = crate::Reg<axi_targ5_fn_mod_iss_bm::AxiTarg5FnModIssBmSpec>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod axi_targ5_fn_mod_iss_bm;
#[doc = "AXI_TARG6_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ6_fn_mod_iss_bm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ6_fn_mod_iss_bm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_targ6_fn_mod_iss_bm`] module"]
#[doc(alias = "AXI_TARG6_FN_MOD_ISS_BM")]
pub type AxiTarg6FnModIssBm = crate::Reg<axi_targ6_fn_mod_iss_bm::AxiTarg6FnModIssBmSpec>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod axi_targ6_fn_mod_iss_bm;
#[doc = "AXI_TARG7_FN_MOD_ISS_BM (rw) register accessor: AXI interconnect - TARG x bus matrix issuing functionality register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ7_fn_mod_iss_bm::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ7_fn_mod_iss_bm::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_targ7_fn_mod_iss_bm`] module"]
#[doc(alias = "AXI_TARG7_FN_MOD_ISS_BM")]
pub type AxiTarg7FnModIssBm = crate::Reg<axi_targ7_fn_mod_iss_bm::AxiTarg7FnModIssBmSpec>;
#[doc = "AXI interconnect - TARG x bus matrix issuing functionality register"]
pub mod axi_targ7_fn_mod_iss_bm;
#[doc = "AXI_TARG1_FN_MOD2 (rw) register accessor: AXI interconnect - TARG x bus matrix functionality 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ1_fn_mod2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ1_fn_mod2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_targ1_fn_mod2`] module"]
#[doc(alias = "AXI_TARG1_FN_MOD2")]
pub type AxiTarg1FnMod2 = crate::Reg<axi_targ1_fn_mod2::AxiTarg1FnMod2Spec>;
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register"]
pub mod axi_targ1_fn_mod2;
#[doc = "AXI_TARG2_FN_MOD2 (rw) register accessor: AXI interconnect - TARG x bus matrix functionality 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ2_fn_mod2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ2_fn_mod2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_targ2_fn_mod2`] module"]
#[doc(alias = "AXI_TARG2_FN_MOD2")]
pub type AxiTarg2FnMod2 = crate::Reg<axi_targ2_fn_mod2::AxiTarg2FnMod2Spec>;
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register"]
pub mod axi_targ2_fn_mod2;
#[doc = "AXI_TARG7_FN_MOD2 (rw) register accessor: AXI interconnect - TARG x bus matrix functionality 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ7_fn_mod2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ7_fn_mod2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_targ7_fn_mod2`] module"]
#[doc(alias = "AXI_TARG7_FN_MOD2")]
pub type AxiTarg7FnMod2 = crate::Reg<axi_targ7_fn_mod2::AxiTarg7FnMod2Spec>;
#[doc = "AXI interconnect - TARG x bus matrix functionality 2 register"]
pub mod axi_targ7_fn_mod2;
#[doc = "AXI_TARG1_FN_MOD_LB (rw) register accessor: AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ1_fn_mod_lb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ1_fn_mod_lb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_targ1_fn_mod_lb`] module"]
#[doc(alias = "AXI_TARG1_FN_MOD_LB")]
pub type AxiTarg1FnModLb = crate::Reg<axi_targ1_fn_mod_lb::AxiTarg1FnModLbSpec>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod axi_targ1_fn_mod_lb;
#[doc = "AXI_TARG2_FN_MOD_LB (rw) register accessor: AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ2_fn_mod_lb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ2_fn_mod_lb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_targ2_fn_mod_lb`] module"]
#[doc(alias = "AXI_TARG2_FN_MOD_LB")]
pub type AxiTarg2FnModLb = crate::Reg<axi_targ2_fn_mod_lb::AxiTarg2FnModLbSpec>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod axi_targ2_fn_mod_lb;
#[doc = "AXI_TARG1_FN_MOD (rw) register accessor: AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ1_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ1_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_targ1_fn_mod`] module"]
#[doc(alias = "AXI_TARG1_FN_MOD")]
pub type AxiTarg1FnMod = crate::Reg<axi_targ1_fn_mod::AxiTarg1FnModSpec>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod axi_targ1_fn_mod;
#[doc = "AXI_TARG2_FN_MOD (rw) register accessor: AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ2_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ2_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_targ2_fn_mod`] module"]
#[doc(alias = "AXI_TARG2_FN_MOD")]
pub type AxiTarg2FnMod = crate::Reg<axi_targ2_fn_mod::AxiTarg2FnModSpec>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod axi_targ2_fn_mod;
#[doc = "AXI_TARG7_FN_MOD (rw) register accessor: AXI interconnect - TARG x long burst functionality modification\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_targ7_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_targ7_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_targ7_fn_mod`] module"]
#[doc(alias = "AXI_TARG7_FN_MOD")]
pub type AxiTarg7FnMod = crate::Reg<axi_targ7_fn_mod::AxiTarg7FnModSpec>;
#[doc = "AXI interconnect - TARG x long burst functionality modification"]
pub mod axi_targ7_fn_mod;
#[doc = "AXI_INI1_FN_MOD2 (rw) register accessor: AXI interconnect - INI x functionality modification 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini1_fn_mod2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini1_fn_mod2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini1_fn_mod2`] module"]
#[doc(alias = "AXI_INI1_FN_MOD2")]
pub type AxiIni1FnMod2 = crate::Reg<axi_ini1_fn_mod2::AxiIni1FnMod2Spec>;
#[doc = "AXI interconnect - INI x functionality modification 2 register"]
pub mod axi_ini1_fn_mod2;
#[doc = "AXI_INI3_FN_MOD2 (rw) register accessor: AXI interconnect - INI x functionality modification 2 register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini3_fn_mod2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini3_fn_mod2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini3_fn_mod2`] module"]
#[doc(alias = "AXI_INI3_FN_MOD2")]
pub type AxiIni3FnMod2 = crate::Reg<axi_ini3_fn_mod2::AxiIni3FnMod2Spec>;
#[doc = "AXI interconnect - INI x functionality modification 2 register"]
pub mod axi_ini3_fn_mod2;
#[doc = "AXI_INI1_FN_MOD_AHB (rw) register accessor: AXI interconnect - INI x AHB functionality modification register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini1_fn_mod_ahb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini1_fn_mod_ahb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini1_fn_mod_ahb`] module"]
#[doc(alias = "AXI_INI1_FN_MOD_AHB")]
pub type AxiIni1FnModAhb = crate::Reg<axi_ini1_fn_mod_ahb::AxiIni1FnModAhbSpec>;
#[doc = "AXI interconnect - INI x AHB functionality modification register"]
pub mod axi_ini1_fn_mod_ahb;
#[doc = "AXI_INI3_FN_MOD_AHB (rw) register accessor: AXI interconnect - INI x AHB functionality modification register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini3_fn_mod_ahb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini3_fn_mod_ahb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini3_fn_mod_ahb`] module"]
#[doc(alias = "AXI_INI3_FN_MOD_AHB")]
pub type AxiIni3FnModAhb = crate::Reg<axi_ini3_fn_mod_ahb::AxiIni3FnModAhbSpec>;
#[doc = "AXI interconnect - INI x AHB functionality modification register"]
pub mod axi_ini3_fn_mod_ahb;
#[doc = "AXI_INI1_READ_QOS (rw) register accessor: AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini1_read_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini1_read_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini1_read_qos`] module"]
#[doc(alias = "AXI_INI1_READ_QOS")]
pub type AxiIni1ReadQos = crate::Reg<axi_ini1_read_qos::AxiIni1ReadQosSpec>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod axi_ini1_read_qos;
#[doc = "AXI_INI2_READ_QOS (rw) register accessor: AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini2_read_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini2_read_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini2_read_qos`] module"]
#[doc(alias = "AXI_INI2_READ_QOS")]
pub type AxiIni2ReadQos = crate::Reg<axi_ini2_read_qos::AxiIni2ReadQosSpec>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod axi_ini2_read_qos;
#[doc = "AXI_INI3_READ_QOS (rw) register accessor: AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini3_read_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini3_read_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini3_read_qos`] module"]
#[doc(alias = "AXI_INI3_READ_QOS")]
pub type AxiIni3ReadQos = crate::Reg<axi_ini3_read_qos::AxiIni3ReadQosSpec>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod axi_ini3_read_qos;
#[doc = "AXI_INI4_READ_QOS (rw) register accessor: AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini4_read_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini4_read_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini4_read_qos`] module"]
#[doc(alias = "AXI_INI4_READ_QOS")]
pub type AxiIni4ReadQos = crate::Reg<axi_ini4_read_qos::AxiIni4ReadQosSpec>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod axi_ini4_read_qos;
#[doc = "AXI_INI5_READ_QOS (rw) register accessor: AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini5_read_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini5_read_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini5_read_qos`] module"]
#[doc(alias = "AXI_INI5_READ_QOS")]
pub type AxiIni5ReadQos = crate::Reg<axi_ini5_read_qos::AxiIni5ReadQosSpec>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod axi_ini5_read_qos;
#[doc = "AXI_INI6_READ_QOS (rw) register accessor: AXI interconnect - INI x read QoS register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini6_read_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini6_read_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini6_read_qos`] module"]
#[doc(alias = "AXI_INI6_READ_QOS")]
pub type AxiIni6ReadQos = crate::Reg<axi_ini6_read_qos::AxiIni6ReadQosSpec>;
#[doc = "AXI interconnect - INI x read QoS register"]
pub mod axi_ini6_read_qos;
#[doc = "AXI_INI1_WRITE_QOS (rw) register accessor: AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini1_write_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini1_write_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini1_write_qos`] module"]
#[doc(alias = "AXI_INI1_WRITE_QOS")]
pub type AxiIni1WriteQos = crate::Reg<axi_ini1_write_qos::AxiIni1WriteQosSpec>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod axi_ini1_write_qos;
#[doc = "AXI_INI2_WRITE_QOS (rw) register accessor: AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini2_write_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini2_write_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini2_write_qos`] module"]
#[doc(alias = "AXI_INI2_WRITE_QOS")]
pub type AxiIni2WriteQos = crate::Reg<axi_ini2_write_qos::AxiIni2WriteQosSpec>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod axi_ini2_write_qos;
#[doc = "AXI_INI3_WRITE_QOS (rw) register accessor: AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini3_write_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini3_write_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini3_write_qos`] module"]
#[doc(alias = "AXI_INI3_WRITE_QOS")]
pub type AxiIni3WriteQos = crate::Reg<axi_ini3_write_qos::AxiIni3WriteQosSpec>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod axi_ini3_write_qos;
#[doc = "AXI_INI4_WRITE_QOS (rw) register accessor: AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini4_write_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini4_write_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini4_write_qos`] module"]
#[doc(alias = "AXI_INI4_WRITE_QOS")]
pub type AxiIni4WriteQos = crate::Reg<axi_ini4_write_qos::AxiIni4WriteQosSpec>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod axi_ini4_write_qos;
#[doc = "AXI_INI5_WRITE_QOS (rw) register accessor: AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini5_write_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini5_write_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini5_write_qos`] module"]
#[doc(alias = "AXI_INI5_WRITE_QOS")]
pub type AxiIni5WriteQos = crate::Reg<axi_ini5_write_qos::AxiIni5WriteQosSpec>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod axi_ini5_write_qos;
#[doc = "AXI_INI6_WRITE_QOS (rw) register accessor: AXI interconnect - INI x write QoS register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini6_write_qos::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini6_write_qos::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini6_write_qos`] module"]
#[doc(alias = "AXI_INI6_WRITE_QOS")]
pub type AxiIni6WriteQos = crate::Reg<axi_ini6_write_qos::AxiIni6WriteQosSpec>;
#[doc = "AXI interconnect - INI x write QoS register"]
pub mod axi_ini6_write_qos;
#[doc = "AXI_INI1_FN_MOD (rw) register accessor: AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini1_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini1_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini1_fn_mod`] module"]
#[doc(alias = "AXI_INI1_FN_MOD")]
pub type AxiIni1FnMod = crate::Reg<axi_ini1_fn_mod::AxiIni1FnModSpec>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod axi_ini1_fn_mod;
#[doc = "AXI_INI2_FN_MOD (rw) register accessor: AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini2_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini2_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini2_fn_mod`] module"]
#[doc(alias = "AXI_INI2_FN_MOD")]
pub type AxiIni2FnMod = crate::Reg<axi_ini2_fn_mod::AxiIni2FnModSpec>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod axi_ini2_fn_mod;
#[doc = "AXI_INI3_FN_MOD (rw) register accessor: AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini3_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini3_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini3_fn_mod`] module"]
#[doc(alias = "AXI_INI3_FN_MOD")]
pub type AxiIni3FnMod = crate::Reg<axi_ini3_fn_mod::AxiIni3FnModSpec>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod axi_ini3_fn_mod;
#[doc = "AXI_INI4_FN_MOD (rw) register accessor: AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini4_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini4_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini4_fn_mod`] module"]
#[doc(alias = "AXI_INI4_FN_MOD")]
pub type AxiIni4FnMod = crate::Reg<axi_ini4_fn_mod::AxiIni4FnModSpec>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod axi_ini4_fn_mod;
#[doc = "AXI_INI5_FN_MOD (rw) register accessor: AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini5_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini5_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini5_fn_mod`] module"]
#[doc(alias = "AXI_INI5_FN_MOD")]
pub type AxiIni5FnMod = crate::Reg<axi_ini5_fn_mod::AxiIni5FnModSpec>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod axi_ini5_fn_mod;
#[doc = "AXI_INI6_FN_MOD (rw) register accessor: AXI interconnect - INI x issuing functionality modification register\n\nYou can [`read`](crate::Reg::read) this register and get [`axi_ini6_fn_mod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`axi_ini6_fn_mod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@axi_ini6_fn_mod`] module"]
#[doc(alias = "AXI_INI6_FN_MOD")]
pub type AxiIni6FnMod = crate::Reg<axi_ini6_fn_mod::AxiIni6FnModSpec>;
#[doc = "AXI interconnect - INI x issuing functionality modification register"]
pub mod axi_ini6_fn_mod;
