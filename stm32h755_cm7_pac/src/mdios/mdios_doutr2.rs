#[doc = "Register `MDIOS_DOUTR2` reader"]
pub type R = crate::R<MdiosDoutr2Spec>;
#[doc = "Register `MDIOS_DOUTR2` writer"]
pub type W = crate::W<MdiosDoutr2Spec>;
#[doc = "Field `DOUT2` reader - Output data sent to MDIO Master during read frames"]
pub type Dout2R = crate::FieldReader<u16>;
#[doc = "Field `DOUT2` writer - Output data sent to MDIO Master during read frames"]
pub type Dout2W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout2(&self) -> Dout2R {
        Dout2R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout2(&mut self) -> Dout2W<MdiosDoutr2Spec> {
        Dout2W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr2Spec;
impl crate::RegisterSpec for MdiosDoutr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr2::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr2Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr2::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR2 to value 0"]
impl crate::Resettable for MdiosDoutr2Spec {}
