#[doc = "Register `CCFG` reader"]
pub type R = crate::R<CcfgSpec>;
#[doc = "Register `CCFG` writer"]
pub type W = crate::W<CcfgSpec>;
#[doc = "Field `TQBT` reader - Time Quanta per Bit Time"]
pub type TqbtR = crate::FieldReader;
#[doc = "Field `TQBT` writer - Time Quanta per Bit Time"]
pub type TqbtW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `BCC` reader - Bypass Clock Calibration"]
pub type BccR = crate::BitReader;
#[doc = "Field `BCC` writer - Bypass Clock Calibration"]
pub type BccW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CFL` reader - Calibration Field Length"]
pub type CflR = crate::BitReader;
#[doc = "Field `CFL` writer - Calibration Field Length"]
pub type CflW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OCPM` reader - Oscillator Clock Periods Minimum"]
pub type OcpmR = crate::FieldReader;
#[doc = "Field `OCPM` writer - Oscillator Clock Periods Minimum"]
pub type OcpmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CDIV` reader - Clock Divider"]
pub type CdivR = crate::FieldReader;
#[doc = "Field `CDIV` writer - Clock Divider"]
pub type CdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `SWR` reader - Software Reset"]
pub type SwrR = crate::BitReader;
#[doc = "Field `SWR` writer - Software Reset"]
pub type SwrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Time Quanta per Bit Time"]
    #[inline(always)]
    pub fn tqbt(&self) -> TqbtR {
        TqbtR::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 6 - Bypass Clock Calibration"]
    #[inline(always)]
    pub fn bcc(&self) -> BccR {
        BccR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Calibration Field Length"]
    #[inline(always)]
    pub fn cfl(&self) -> CflR {
        CflR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Oscillator Clock Periods Minimum"]
    #[inline(always)]
    pub fn ocpm(&self) -> OcpmR {
        OcpmR::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:19 - Clock Divider"]
    #[inline(always)]
    pub fn cdiv(&self) -> CdivR {
        CdivR::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 31 - Software Reset"]
    #[inline(always)]
    pub fn swr(&self) -> SwrR {
        SwrR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Time Quanta per Bit Time"]
    #[inline(always)]
    pub fn tqbt(&mut self) -> TqbtW<CcfgSpec> {
        TqbtW::new(self, 0)
    }
    #[doc = "Bit 6 - Bypass Clock Calibration"]
    #[inline(always)]
    pub fn bcc(&mut self) -> BccW<CcfgSpec> {
        BccW::new(self, 6)
    }
    #[doc = "Bit 7 - Calibration Field Length"]
    #[inline(always)]
    pub fn cfl(&mut self) -> CflW<CcfgSpec> {
        CflW::new(self, 7)
    }
    #[doc = "Bits 8:15 - Oscillator Clock Periods Minimum"]
    #[inline(always)]
    pub fn ocpm(&mut self) -> OcpmW<CcfgSpec> {
        OcpmW::new(self, 8)
    }
    #[doc = "Bits 16:19 - Clock Divider"]
    #[inline(always)]
    pub fn cdiv(&mut self) -> CdivW<CcfgSpec> {
        CdivW::new(self, 16)
    }
    #[doc = "Bit 31 - Software Reset"]
    #[inline(always)]
    pub fn swr(&mut self) -> SwrW<CcfgSpec> {
        SwrW::new(self, 31)
    }
}
#[doc = "Calibration Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CcfgSpec;
impl crate::RegisterSpec for CcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccfg::R`](R) reader structure"]
impl crate::Readable for CcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`ccfg::W`](W) writer structure"]
impl crate::Writable for CcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCFG to value 0"]
impl crate::Resettable for CcfgSpec {}
