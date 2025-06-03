#[doc = "Register `MDIOS_DOUTR1` reader"]
pub type R = crate::R<MdiosDoutr1Spec>;
#[doc = "Register `MDIOS_DOUTR1` writer"]
pub type W = crate::W<MdiosDoutr1Spec>;
#[doc = "Field `DOUT1` reader - Output data sent to MDIO Master during read frames"]
pub type Dout1R = crate::FieldReader<u16>;
#[doc = "Field `DOUT1` writer - Output data sent to MDIO Master during read frames"]
pub type Dout1W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout1(&self) -> Dout1R {
        Dout1R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout1(&mut self) -> Dout1W<MdiosDoutr1Spec> {
        Dout1W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr1Spec;
impl crate::RegisterSpec for MdiosDoutr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr1::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr1Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr1::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR1 to value 0"]
impl crate::Resettable for MdiosDoutr1Spec {}
