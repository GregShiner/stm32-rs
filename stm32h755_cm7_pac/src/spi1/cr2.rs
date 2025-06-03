#[doc = "Register `CR2` reader"]
pub type R = crate::R<Cr2Spec>;
#[doc = "Register `CR2` writer"]
pub type W = crate::W<Cr2Spec>;
#[doc = "Field `TSIZE` reader - Number of data at current transfer"]
pub type TsizeR = crate::FieldReader<u16>;
#[doc = "Field `TSIZE` writer - Number of data at current transfer"]
pub type TsizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TSER` reader - Number of data transfer extension to be reload into TSIZE just when a previous"]
pub type TserR = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Number of data at current transfer"]
    #[inline(always)]
    pub fn tsize(&self) -> TsizeR {
        TsizeR::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Number of data transfer extension to be reload into TSIZE just when a previous"]
    #[inline(always)]
    pub fn tser(&self) -> TserR {
        TserR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Number of data at current transfer"]
    #[inline(always)]
    pub fn tsize(&mut self) -> TsizeW<Cr2Spec> {
        TsizeW::new(self, 0)
    }
}
#[doc = "control register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`cr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Cr2Spec;
impl crate::RegisterSpec for Cr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr2::R`](R) reader structure"]
impl crate::Readable for Cr2Spec {}
#[doc = "`write(|w| ..)` method takes [`cr2::W`](W) writer structure"]
impl crate::Writable for Cr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for Cr2Spec {}
