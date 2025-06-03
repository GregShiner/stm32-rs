#[doc = "Register `MDIOS_DOUTR10` reader"]
pub type R = crate::R<MdiosDoutr10Spec>;
#[doc = "Register `MDIOS_DOUTR10` writer"]
pub type W = crate::W<MdiosDoutr10Spec>;
#[doc = "Field `DOUT10` reader - Output data sent to MDIO Master during read frames"]
pub type Dout10R = crate::FieldReader<u16>;
#[doc = "Field `DOUT10` writer - Output data sent to MDIO Master during read frames"]
pub type Dout10W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout10(&self) -> Dout10R {
        Dout10R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout10(&mut self) -> Dout10W<MdiosDoutr10Spec> {
        Dout10W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 10\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr10::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr10::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr10Spec;
impl crate::RegisterSpec for MdiosDoutr10Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr10::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr10Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr10::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr10Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR10 to value 0"]
impl crate::Resettable for MdiosDoutr10Spec {}
