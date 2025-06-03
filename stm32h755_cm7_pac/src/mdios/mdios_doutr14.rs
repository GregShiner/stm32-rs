#[doc = "Register `MDIOS_DOUTR14` reader"]
pub type R = crate::R<MdiosDoutr14Spec>;
#[doc = "Register `MDIOS_DOUTR14` writer"]
pub type W = crate::W<MdiosDoutr14Spec>;
#[doc = "Field `DOUT14` reader - Output data sent to MDIO Master during read frames"]
pub type Dout14R = crate::FieldReader<u16>;
#[doc = "Field `DOUT14` writer - Output data sent to MDIO Master during read frames"]
pub type Dout14W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout14(&self) -> Dout14R {
        Dout14R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout14(&mut self) -> Dout14W<MdiosDoutr14Spec> {
        Dout14W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 14\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr14::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr14::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr14Spec;
impl crate::RegisterSpec for MdiosDoutr14Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr14::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr14Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr14::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr14Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR14 to value 0"]
impl crate::Resettable for MdiosDoutr14Spec {}
