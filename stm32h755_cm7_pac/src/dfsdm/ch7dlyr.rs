#[doc = "Register `CH7DLYR` reader"]
pub type R = crate::R<Ch7dlyrSpec>;
#[doc = "Register `CH7DLYR` writer"]
pub type W = crate::W<Ch7dlyrSpec>;
#[doc = "Field `PLSSKP` reader - PLSSKP"]
pub type PlsskpR = crate::FieldReader;
#[doc = "Field `PLSSKP` writer - PLSSKP"]
pub type PlsskpW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - PLSSKP"]
    #[inline(always)]
    pub fn plsskp(&self) -> PlsskpR {
        PlsskpR::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - PLSSKP"]
    #[inline(always)]
    pub fn plsskp(&mut self) -> PlsskpW<Ch7dlyrSpec> {
        PlsskpW::new(self, 0)
    }
}
#[doc = "channel y delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch7dlyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch7dlyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch7dlyrSpec;
impl crate::RegisterSpec for Ch7dlyrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch7dlyr::R`](R) reader structure"]
impl crate::Readable for Ch7dlyrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch7dlyr::W`](W) writer structure"]
impl crate::Writable for Ch7dlyrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH7DLYR to value 0"]
impl crate::Resettable for Ch7dlyrSpec {}
