#[doc = "Register `CH6DLYR` reader"]
pub type R = crate::R<Ch6dlyrSpec>;
#[doc = "Register `CH6DLYR` writer"]
pub type W = crate::W<Ch6dlyrSpec>;
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
    pub fn plsskp(&mut self) -> PlsskpW<Ch6dlyrSpec> {
        PlsskpW::new(self, 0)
    }
}
#[doc = "channel y delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch6dlyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch6dlyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch6dlyrSpec;
impl crate::RegisterSpec for Ch6dlyrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch6dlyr::R`](R) reader structure"]
impl crate::Readable for Ch6dlyrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch6dlyr::W`](W) writer structure"]
impl crate::Writable for Ch6dlyrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH6DLYR to value 0"]
impl crate::Resettable for Ch6dlyrSpec {}
