#[doc = "Register `MDIOS_DOUTR15` reader"]
pub type R = crate::R<MdiosDoutr15Spec>;
#[doc = "Register `MDIOS_DOUTR15` writer"]
pub type W = crate::W<MdiosDoutr15Spec>;
#[doc = "Field `DOUT15` reader - Output data sent to MDIO Master during read frames"]
pub type Dout15R = crate::FieldReader<u16>;
#[doc = "Field `DOUT15` writer - Output data sent to MDIO Master during read frames"]
pub type Dout15W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout15(&self) -> Dout15R {
        Dout15R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout15(&mut self) -> Dout15W<MdiosDoutr15Spec> {
        Dout15W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 15\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr15::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr15::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr15Spec;
impl crate::RegisterSpec for MdiosDoutr15Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr15::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr15Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr15::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr15Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR15 to value 0"]
impl crate::Resettable for MdiosDoutr15Spec {}
