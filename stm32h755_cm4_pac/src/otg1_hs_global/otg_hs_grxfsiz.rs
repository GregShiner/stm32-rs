#[doc = "Register `OTG_HS_GRXFSIZ` reader"]
pub type R = crate::R<OtgHsGrxfsizSpec>;
#[doc = "Register `OTG_HS_GRXFSIZ` writer"]
pub type W = crate::W<OtgHsGrxfsizSpec>;
#[doc = "Field `RXFD` reader - RxFIFO depth"]
pub type RxfdR = crate::FieldReader<u16>;
#[doc = "Field `RXFD` writer - RxFIFO depth"]
pub type RxfdW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - RxFIFO depth"]
    #[inline(always)]
    pub fn rxfd(&self) -> RxfdR {
        RxfdR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - RxFIFO depth"]
    #[inline(always)]
    pub fn rxfd(&mut self) -> RxfdW<OtgHsGrxfsizSpec> {
        RxfdW::new(self, 0)
    }
}
#[doc = "OTG_HS Receive FIFO size register\n\nYou can [`read`](crate::Reg::read) this register and get [`otg_hs_grxfsiz::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`otg_hs_grxfsiz::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OtgHsGrxfsizSpec;
impl crate::RegisterSpec for OtgHsGrxfsizSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`otg_hs_grxfsiz::R`](R) reader structure"]
impl crate::Readable for OtgHsGrxfsizSpec {}
#[doc = "`write(|w| ..)` method takes [`otg_hs_grxfsiz::W`](W) writer structure"]
impl crate::Writable for OtgHsGrxfsizSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OTG_HS_GRXFSIZ to value 0x0200"]
impl crate::Resettable for OtgHsGrxfsizSpec {
    const RESET_VALUE: u32 = 0x0200;
}
