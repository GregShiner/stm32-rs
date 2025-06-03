#[doc = "Register `CSR1` reader"]
pub type R = crate::R<Csr1Spec>;
#[doc = "Field `PVDO` reader - Programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set."]
pub type PvdoR = crate::BitReader;
#[doc = "Field `ACTVOSRDY` reader - Voltage levels ready bit for currently used VOS and SDLEVEL This bit is set to 1 by hardware when the voltage regulator and the SD converter are both disabled and Bypass mode is selected in PWR control register 3 (PWR_CR3)."]
pub type ActvosrdyR = crate::BitReader;
#[doc = "Field `ACTVOS` reader - VOS currently applied for VCORE voltage scaling selection. These bits reflect the last VOS value applied to the PMU."]
pub type ActvosR = crate::FieldReader;
#[doc = "Field `AVDO` reader - Analog voltage detector output on VDDA This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the AVDEN bit is set."]
pub type AvdoR = crate::BitReader;
impl R {
    #[doc = "Bit 4 - Programmable voltage detect output This bit is set and cleared by hardware. It is valid only if the PVD has been enabled by the PVDE bit. Note: since the PVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the PVDE bit is set."]
    #[inline(always)]
    pub fn pvdo(&self) -> PvdoR {
        PvdoR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 13 - Voltage levels ready bit for currently used VOS and SDLEVEL This bit is set to 1 by hardware when the voltage regulator and the SD converter are both disabled and Bypass mode is selected in PWR control register 3 (PWR_CR3)."]
    #[inline(always)]
    pub fn actvosrdy(&self) -> ActvosrdyR {
        ActvosrdyR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - VOS currently applied for VCORE voltage scaling selection. These bits reflect the last VOS value applied to the PMU."]
    #[inline(always)]
    pub fn actvos(&self) -> ActvosR {
        ActvosR::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bit 16 - Analog voltage detector output on VDDA This bit is set and cleared by hardware. It is valid only if AVD on VDDA is enabled by the AVDEN bit. Note: Since the AVD is disabled in Standby mode, this bit is equal to 0 after Standby or reset until the AVDEN bit is set."]
    #[inline(always)]
    pub fn avdo(&self) -> AvdoR {
        AvdoR::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "PWR control status register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`csr1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Csr1Spec;
impl crate::RegisterSpec for Csr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`csr1::R`](R) reader structure"]
impl crate::Readable for Csr1Spec {}
#[doc = "`reset()` method sets CSR1 to value 0x4000"]
impl crate::Resettable for Csr1Spec {
    const RESET_VALUE: u32 = 0x4000;
}
