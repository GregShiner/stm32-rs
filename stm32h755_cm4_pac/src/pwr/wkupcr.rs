#[doc = "Register `WKUPCR` reader"]
pub type R = crate::R<WkupcrSpec>;
#[doc = "Register `WKUPCR` writer"]
pub type W = crate::W<WkupcrSpec>;
#[doc = "Field `WKUPC` reader - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WkupcR = crate::FieldReader;
#[doc = "Field `WKUPC` writer - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
pub type WkupcW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    pub fn wkupc(&self) -> WkupcR {
        WkupcR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Clear Wakeup pin flag for WKUP. These bits are always read as 0."]
    #[inline(always)]
    pub fn wkupc(&mut self) -> WkupcW<WkupcrSpec> {
        WkupcW::new(self, 0)
    }
}
#[doc = "reset only by system reset, not reset by wakeup from Standby mode5 wait states are required when writing this register (when clearing a WKUPF bit in PWR_WKUPFR, the AHB write access will complete after the WKUPF has been cleared).\n\nYou can [`read`](crate::Reg::read) this register and get [`wkupcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wkupcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WkupcrSpec;
impl crate::RegisterSpec for WkupcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`wkupcr::R`](R) reader structure"]
impl crate::Readable for WkupcrSpec {}
#[doc = "`write(|w| ..)` method takes [`wkupcr::W`](W) writer structure"]
impl crate::Writable for WkupcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WKUPCR to value 0"]
impl crate::Resettable for WkupcrSpec {}
