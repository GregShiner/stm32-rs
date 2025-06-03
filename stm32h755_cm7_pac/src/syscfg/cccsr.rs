#[doc = "Register `CCCSR` reader"]
pub type R = crate::R<CccsrSpec>;
#[doc = "Register `CCCSR` writer"]
pub type W = crate::W<CccsrSpec>;
#[doc = "Field `EN` reader - enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS` reader - Code selection"]
pub type CsR = crate::BitReader;
#[doc = "Field `CS` writer - Code selection"]
pub type CsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `READY` reader - Compensation cell ready flag"]
pub type ReadyR = crate::BitReader;
#[doc = "Field `READY` writer - Compensation cell ready flag"]
pub type ReadyW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSLV` reader - High-speed at low-voltage"]
pub type HslvR = crate::BitReader;
#[doc = "Field `HSLV` writer - High-speed at low-voltage"]
pub type HslvW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Code selection"]
    #[inline(always)]
    pub fn cs(&self) -> CsR {
        CsR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - Compensation cell ready flag"]
    #[inline(always)]
    pub fn ready(&self) -> ReadyR {
        ReadyR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - High-speed at low-voltage"]
    #[inline(always)]
    pub fn hslv(&self) -> HslvR {
        HslvR::new(((self.bits >> 16) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CccsrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Code selection"]
    #[inline(always)]
    pub fn cs(&mut self) -> CsW<CccsrSpec> {
        CsW::new(self, 1)
    }
    #[doc = "Bit 8 - Compensation cell ready flag"]
    #[inline(always)]
    pub fn ready(&mut self) -> ReadyW<CccsrSpec> {
        ReadyW::new(self, 8)
    }
    #[doc = "Bit 16 - High-speed at low-voltage"]
    #[inline(always)]
    pub fn hslv(&mut self) -> HslvW<CccsrSpec> {
        HslvW::new(self, 16)
    }
}
#[doc = "compensation cell control/status register\n\nYou can [`read`](crate::Reg::read) this register and get [`cccsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cccsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CccsrSpec;
impl crate::RegisterSpec for CccsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cccsr::R`](R) reader structure"]
impl crate::Readable for CccsrSpec {}
#[doc = "`write(|w| ..)` method takes [`cccsr::W`](W) writer structure"]
impl crate::Writable for CccsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCCSR to value 0"]
impl crate::Resettable for CccsrSpec {}
