#[doc = "Register `MDIOS_DOUTR26` reader"]
pub type R = crate::R<MdiosDoutr26Spec>;
#[doc = "Register `MDIOS_DOUTR26` writer"]
pub type W = crate::W<MdiosDoutr26Spec>;
#[doc = "Field `DOUT26` reader - Output data sent to MDIO Master during read frames"]
pub type Dout26R = crate::FieldReader<u16>;
#[doc = "Field `DOUT26` writer - Output data sent to MDIO Master during read frames"]
pub type Dout26W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout26(&self) -> Dout26R {
        Dout26R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout26(&mut self) -> Dout26W<MdiosDoutr26Spec> {
        Dout26W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 26\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr26::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr26::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr26Spec;
impl crate::RegisterSpec for MdiosDoutr26Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr26::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr26Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr26::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr26Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR26 to value 0"]
impl crate::Resettable for MdiosDoutr26Spec {}
