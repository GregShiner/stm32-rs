#[doc = "Register `OTG_HS_GNPTXSTS` reader"]
pub type R = crate::R<OtgHsGnptxstsSpec>;
#[doc = "Field `NPTXFSAV` reader - Nonperiodic TxFIFO space available"]
pub type NptxfsavR = crate::FieldReader<u16>;
#[doc = "Field `NPTQXSAV` reader - Nonperiodic transmit request queue space available"]
pub type NptqxsavR = crate::FieldReader;
#[doc = "Field `NPTXQTOP` reader - Top of the nonperiodic transmit request queue"]
pub type NptxqtopR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Nonperiodic TxFIFO space available"]
    #[inline(always)]
    pub fn nptxfsav(&self) -> NptxfsavR {
        NptxfsavR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Nonperiodic transmit request queue space available"]
    #[inline(always)]
    pub fn nptqxsav(&self) -> NptqxsavR {
        NptqxsavR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:30 - Top of the nonperiodic transmit request queue"]
    #[inline(always)]
    pub fn nptxqtop(&self) -> NptxqtopR {
        NptxqtopR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "OTG_HS nonperiodic transmit FIFO/queue status register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_gnptxsts::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsGnptxstsSpec;
impl crate::RegisterSpec for OtgHsGnptxstsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_gnptxsts::R`](R) reader structure"]
impl crate::Readable for OtgHsGnptxstsSpec {}
#[doc = "`reset()` method sets OTG_HS_GNPTXSTS to value 0x0008_0200"]
impl crate::Resettable for OtgHsGnptxstsSpec {
    const RESET_VALUE: u32 = 0x0008_0200;
}
