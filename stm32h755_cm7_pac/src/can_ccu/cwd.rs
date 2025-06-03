#[doc = "Register `CWD` reader"]
pub type R = crate::R<CwdSpec>;
#[doc = "Register `CWD` writer"]
pub type W = crate::W<CwdSpec>;
#[doc = "Field `WDC` reader - WDC"]
pub type WdcR = crate::FieldReader<u16>;
#[doc = "Field `WDC` writer - WDC"]
pub type WdcW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `WDV` reader - WDV"]
pub type WdvR = crate::FieldReader<u16>;
#[doc = "Field `WDV` writer - WDV"]
pub type WdvW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - WDC"]
    #[inline(always)]
    pub fn wdc(&self) -> WdcR {
        WdcR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - WDV"]
    #[inline(always)]
    pub fn wdv(&self) -> WdvR {
        WdvR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - WDC"]
    #[inline(always)]
    pub fn wdc(&mut self) -> WdcW<CwdSpec> {
        WdcW::new(self, 0)
    }
    #[doc = "Bits 16:31 - WDV"]
    #[inline(always)]
    pub fn wdv(&mut self) -> WdvW<CwdSpec> {
        WdvW::new(self, 16)
    }
}
#[doc = "Calibration Watchdog Register\n\nYou can [`read`](crate::Reg::read) this register and get [`cwd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CwdSpec;
impl crate::RegisterSpec for CwdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cwd::R`](R) reader structure"]
impl crate::Readable for CwdSpec {}
#[doc = "`write(|w| ..)` method takes [`cwd::W`](W) writer structure"]
impl crate::Writable for CwdSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CWD to value 0"]
impl crate::Resettable for CwdSpec {}
