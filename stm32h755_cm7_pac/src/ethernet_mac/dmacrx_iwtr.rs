#[doc = "Register `DMACRxIWTR` reader"]
pub type R = crate::R<DmacrxIwtrSpec>;
#[doc = "Register `DMACRxIWTR` writer"]
pub type W = crate::W<DmacrxIwtrSpec>;
#[doc = "Field `RWT` reader - Receive Interrupt Watchdog Timer Count"]
pub type RwtR = crate::FieldReader;
#[doc = "Field `RWT` writer - Receive Interrupt Watchdog Timer Count"]
pub type RwtW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Receive Interrupt Watchdog Timer Count"]
    #[inline(always)]
    pub fn rwt(&self) -> RwtR {
        RwtR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Receive Interrupt Watchdog Timer Count"]
    #[inline(always)]
    pub fn rwt(&mut self) -> RwtW<DmacrxIwtrSpec> {
        RwtW::new(self, 0)
    }
}
#[doc = "Channel Rx interrupt watchdog timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`dmacrx_iwtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacrx_iwtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DmacrxIwtrSpec;
impl crate::RegisterSpec for DmacrxIwtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dmacrx_iwtr::R`](R) reader structure"]
impl crate::Readable for DmacrxIwtrSpec {}
#[doc = "`write(|w| ..)` method takes [`dmacrx_iwtr::W`](W) writer structure"]
impl crate::Writable for DmacrxIwtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DMACRxIWTR to value 0"]
impl crate::Resettable for DmacrxIwtrSpec {}
