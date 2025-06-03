#[doc = "Register `MDIOS_DOUTR20` reader"]
pub type R = crate::R<MdiosDoutr20Spec>;
#[doc = "Register `MDIOS_DOUTR20` writer"]
pub type W = crate::W<MdiosDoutr20Spec>;
#[doc = "Field `DOUT20` reader - Output data sent to MDIO Master during read frames"]
pub type Dout20R = crate::FieldReader<u16>;
#[doc = "Field `DOUT20` writer - Output data sent to MDIO Master during read frames"]
pub type Dout20W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout20(&self) -> Dout20R {
        Dout20R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout20(&mut self) -> Dout20W<MdiosDoutr20Spec> {
        Dout20W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 20\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr20Spec;
impl crate::RegisterSpec for MdiosDoutr20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr20::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr20Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr20::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR20 to value 0"]
impl crate::Resettable for MdiosDoutr20Spec {}
