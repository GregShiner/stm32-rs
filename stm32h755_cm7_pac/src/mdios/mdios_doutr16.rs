#[doc = "Register `MDIOS_DOUTR16` reader"]
pub type R = crate::R<MdiosDoutr16Spec>;
#[doc = "Register `MDIOS_DOUTR16` writer"]
pub type W = crate::W<MdiosDoutr16Spec>;
#[doc = "Field `DOUT16` reader - Output data sent to MDIO Master during read frames"]
pub type Dout16R = crate::FieldReader<u16>;
#[doc = "Field `DOUT16` writer - Output data sent to MDIO Master during read frames"]
pub type Dout16W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout16(&self) -> Dout16R {
        Dout16R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout16(&mut self) -> Dout16W<MdiosDoutr16Spec> {
        Dout16W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 16\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr16::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr16::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr16Spec;
impl crate::RegisterSpec for MdiosDoutr16Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr16::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr16Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr16::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr16Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR16 to value 0"]
impl crate::Resettable for MdiosDoutr16Spec {}
