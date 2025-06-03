#[doc = "Register `MDIOS_DOUTR23` reader"]
pub type R = crate::R<MdiosDoutr23Spec>;
#[doc = "Register `MDIOS_DOUTR23` writer"]
pub type W = crate::W<MdiosDoutr23Spec>;
#[doc = "Field `DOUT23` reader - Output data sent to MDIO Master during read frames"]
pub type Dout23R = crate::FieldReader<u16>;
#[doc = "Field `DOUT23` writer - Output data sent to MDIO Master during read frames"]
pub type Dout23W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout23(&self) -> Dout23R {
        Dout23R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout23(&mut self) -> Dout23W<MdiosDoutr23Spec> {
        Dout23W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 23\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr23::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr23::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr23Spec;
impl crate::RegisterSpec for MdiosDoutr23Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr23::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr23Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr23::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr23Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR23 to value 0"]
impl crate::Resettable for MdiosDoutr23Spec {}
