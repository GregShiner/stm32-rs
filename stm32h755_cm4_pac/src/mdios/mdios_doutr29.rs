#[doc = "Register `MDIOS_DOUTR29` reader"]
pub type R = crate::R<MdiosDoutr29Spec>;
#[doc = "Register `MDIOS_DOUTR29` writer"]
pub type W = crate::W<MdiosDoutr29Spec>;
#[doc = "Field `DOUT29` reader - Output data sent to MDIO Master during read frames"]
pub type Dout29R = crate::FieldReader<u16>;
#[doc = "Field `DOUT29` writer - Output data sent to MDIO Master during read frames"]
pub type Dout29W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout29(&self) -> Dout29R {
        Dout29R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout29(&mut self) -> Dout29W<MdiosDoutr29Spec> {
        Dout29W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 29\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr29::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr29::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr29Spec;
impl crate::RegisterSpec for MdiosDoutr29Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr29::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr29Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr29::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr29Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR29 to value 0"]
impl crate::Resettable for MdiosDoutr29Spec {}
