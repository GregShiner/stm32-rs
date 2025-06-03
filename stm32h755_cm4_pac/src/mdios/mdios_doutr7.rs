#[doc = "Register `MDIOS_DOUTR7` reader"]
pub type R = crate::R<MdiosDoutr7Spec>;
#[doc = "Register `MDIOS_DOUTR7` writer"]
pub type W = crate::W<MdiosDoutr7Spec>;
#[doc = "Field `DOUT7` reader - Output data sent to MDIO Master during read frames"]
pub type Dout7R = crate::FieldReader<u16>;
#[doc = "Field `DOUT7` writer - Output data sent to MDIO Master during read frames"]
pub type Dout7W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout7(&self) -> Dout7R {
        Dout7R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout7(&mut self) -> Dout7W<MdiosDoutr7Spec> {
        Dout7W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 7\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr7::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr7::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr7Spec;
impl crate::RegisterSpec for MdiosDoutr7Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr7::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr7Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr7::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr7Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR7 to value 0"]
impl crate::Resettable for MdiosDoutr7Spec {}
