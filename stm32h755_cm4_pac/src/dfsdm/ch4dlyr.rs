#[doc = "Register `CH4DLYR` reader"]
pub type R = crate::R<Ch4dlyrSpec>;
#[doc = "Register `CH4DLYR` writer"]
pub type W = crate::W<Ch4dlyrSpec>;
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
    pub fn plsskp(&mut self) -> PlsskpW<Ch4dlyrSpec> {
        PlsskpW::new(self, 0)
    }
}
#[doc = "channel y delay register\n\nYou can [`read`](crate::Reg::read) this register and get [`ch4dlyr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ch4dlyr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ch4dlyrSpec;
impl crate::RegisterSpec for Ch4dlyrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ch4dlyr::R`](R) reader structure"]
impl crate::Readable for Ch4dlyrSpec {}
#[doc = "`write(|w| ..)` method takes [`ch4dlyr::W`](W) writer structure"]
impl crate::Writable for Ch4dlyrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CH4DLYR to value 0"]
impl crate::Resettable for Ch4dlyrSpec {}
