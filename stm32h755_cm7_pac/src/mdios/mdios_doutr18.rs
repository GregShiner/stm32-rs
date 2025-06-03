#[doc = "Register `MDIOS_DOUTR18` reader"]
pub type R = crate::R<MdiosDoutr18Spec>;
#[doc = "Register `MDIOS_DOUTR18` writer"]
pub type W = crate::W<MdiosDoutr18Spec>;
#[doc = "Field `DOUT18` reader - Output data sent to MDIO Master during read frames"]
pub type Dout18R = crate::FieldReader<u16>;
#[doc = "Field `DOUT18` writer - Output data sent to MDIO Master during read frames"]
pub type Dout18W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout18(&self) -> Dout18R {
        Dout18R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout18(&mut self) -> Dout18W<MdiosDoutr18Spec> {
        Dout18W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 18\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr18::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr18::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr18Spec;
impl crate::RegisterSpec for MdiosDoutr18Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr18::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr18Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr18::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr18Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR18 to value 0"]
impl crate::Resettable for MdiosDoutr18Spec {}
