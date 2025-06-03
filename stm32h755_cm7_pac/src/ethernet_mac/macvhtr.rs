#[doc = "Register `MACVHTR` reader"]
pub type R = crate::R<MacvhtrSpec>;
#[doc = "Register `MACVHTR` writer"]
pub type W = crate::W<MacvhtrSpec>;
#[doc = "Field `VLHT` reader - VLHT"]
pub type VlhtR = crate::FieldReader<u16>;
#[doc = "Field `VLHT` writer - VLHT"]
pub type VlhtW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - VLHT"]
    #[inline(always)]
    pub fn vlht(&self) -> VlhtR {
        VlhtR::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - VLHT"]
    #[inline(always)]
    pub fn vlht(&mut self) -> VlhtW<MacvhtrSpec> {
        VlhtW::new(self, 0)
    }
}
#[doc = "VLAN Hash table register\n\nYou can [`read`](crate::Reg::read) this register and get [`macvhtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macvhtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacvhtrSpec;
impl crate::RegisterSpec for MacvhtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macvhtr::R`](R) reader structure"]
impl crate::Readable for MacvhtrSpec {}
#[doc = "`write(|w| ..)` method takes [`macvhtr::W`](W) writer structure"]
impl crate::Writable for MacvhtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACVHTR to value 0"]
impl crate::Resettable for MacvhtrSpec {}
