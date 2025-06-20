#[doc = "Register `OTG_HS_HCFG` reader"]
pub type R = crate::R<OtgHsHcfgSpec>;
#[doc = "Register `OTG_HS_HCFG` writer"]
pub type W = crate::W<OtgHsHcfgSpec>;
#[doc = "Field `FSLSPCS` reader - FS/LS PHY clock select"]
pub type FslspcsR = crate::FieldReader;
#[doc = "Field `FSLSPCS` writer - FS/LS PHY clock select"]
pub type FslspcsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FSLSS` reader - FS- and LS-only support"]
pub type FslssR = crate::BitReader;
impl R {
    #[doc = "Bits 0:1 - FS/LS PHY clock select"]
    #[inline(always)]
    pub fn fslspcs(&self) -> FslspcsR {
        FslspcsR::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - FS- and LS-only support"]
    #[inline(always)]
    pub fn fslss(&self) -> FslssR {
        FslssR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - FS/LS PHY clock select"]
    #[inline(always)]
    pub fn fslspcs(&mut self) -> FslspcsW<OtgHsHcfgSpec> {
        FslspcsW::new(self, 0)
    }
}
#[doc = "OTG_HS host configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_hcfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_hcfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsHcfgSpec;
impl crate::RegisterSpec for OtgHsHcfgSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_hcfg::R`](R) reader structure"]
impl crate::Readable for OtgHsHcfgSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_hcfg::W`](W) writer structure"]
impl crate::Writable for OtgHsHcfgSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_HCFG to value 0"]
impl crate::Resettable for OtgHsHcfgSpec {}
