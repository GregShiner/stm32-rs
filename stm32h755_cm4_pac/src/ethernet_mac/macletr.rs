#[doc = "Register `MACLETR` reader"]
pub type R = crate::R<MacletrSpec>;
#[doc = "Register `MACLETR` writer"]
pub type W = crate::W<MacletrSpec>;
#[doc = "Field `LPIET` reader - LPIET"]
pub type LpietR = crate::FieldReader<u32>;
#[doc = "Field `LPIET` writer - LPIET"]
pub type LpietW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
impl R {
    #[doc = "Bits 0:16 - LPIET"]
    #[inline(always)]
    pub fn lpiet(&self) -> LpietR {
        LpietR::new(self.bits & 0x0001_ffff)
    }
}
impl W {
    #[doc = "Bits 0:16 - LPIET"]
    #[inline(always)]
    pub fn lpiet(&mut self) -> LpietW<MacletrSpec> {
        LpietW::new(self, 0)
    }
}
#[doc = "LPI entry timer register\n\nYou can [`read`](crate::Reg::read) this register and get [`macletr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macletr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacletrSpec;
impl crate::RegisterSpec for MacletrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macletr::R`](R) reader structure"]
impl crate::Readable for MacletrSpec {}
#[doc = "`write(|w| ..)` method takes [`macletr::W`](W) writer structure"]
impl crate::Writable for MacletrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACLETR to value 0"]
impl crate::Resettable for MacletrSpec {}
