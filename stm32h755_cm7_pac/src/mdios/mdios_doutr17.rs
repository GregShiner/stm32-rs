#[doc = "Register `MDIOS_DOUTR17` reader"]
pub type R = crate::R<MdiosDoutr17Spec>;
#[doc = "Register `MDIOS_DOUTR17` writer"]
pub type W = crate::W<MdiosDoutr17Spec>;
#[doc = "Field `DOUT17` reader - Output data sent to MDIO Master during read frames"]
pub type Dout17R = crate::FieldReader<u16>;
#[doc = "Field `DOUT17` writer - Output data sent to MDIO Master during read frames"]
pub type Dout17W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout17(&self) -> Dout17R {
        Dout17R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout17(&mut self) -> Dout17W<MdiosDoutr17Spec> {
        Dout17W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 17\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr17::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr17::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr17Spec;
impl crate::RegisterSpec for MdiosDoutr17Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr17::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr17Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr17::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr17Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR17 to value 0"]
impl crate::Resettable for MdiosDoutr17Spec {}
