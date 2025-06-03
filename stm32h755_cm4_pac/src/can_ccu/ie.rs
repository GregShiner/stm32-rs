#[doc = "Register `IE` reader"]
pub type R = crate::R<IeSpec>;
#[doc = "Register `IE` writer"]
pub type W = crate::W<IeSpec>;
#[doc = "Field `CWEE` reader - Calibration Watchdog Event Enable"]
pub type CweeR = crate::BitReader;
#[doc = "Field `CWEE` writer - Calibration Watchdog Event Enable"]
pub type CweeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSCE` reader - Calibration State Changed Enable"]
pub type CsceR = crate::BitReader;
#[doc = "Field `CSCE` writer - Calibration State Changed Enable"]
pub type CsceW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Calibration Watchdog Event Enable"]
    #[inline(always)]
    pub fn cwee(&self) -> CweeR {
        CweeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Calibration State Changed Enable"]
    #[inline(always)]
    pub fn csce(&self) -> CsceR {
        CsceR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Calibration Watchdog Event Enable"]
    #[inline(always)]
    pub fn cwee(&mut self) -> CweeW<IeSpec> {
        CweeW::new(self, 0)
    }
    #[doc = "Bit 1 - Calibration State Changed Enable"]
    #[inline(always)]
    pub fn csce(&mut self) -> CsceW<IeSpec> {
        CsceW::new(self, 1)
    }
}
#[doc = "Clock Calibration Unit Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ie::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ie::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IeSpec;
impl crate::RegisterSpec for IeSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ie::R`](R) reader structure"]
impl crate::Readable for IeSpec {}
#[doc = "`write(|w| ..)` method takes [`ie::W`](W) writer structure"]
impl crate::Writable for IeSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IE to value 0"]
impl crate::Resettable for IeSpec {}
