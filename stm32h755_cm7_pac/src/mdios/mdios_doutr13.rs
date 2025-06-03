#[doc = "Register `MDIOS_DOUTR13` reader"]
pub type R = crate::R<MdiosDoutr13Spec>;
#[doc = "Register `MDIOS_DOUTR13` writer"]
pub type W = crate::W<MdiosDoutr13Spec>;
#[doc = "Field `DOUT13` reader - Output data sent to MDIO Master during read frames"]
pub type Dout13R = crate::FieldReader<u16>;
#[doc = "Field `DOUT13` writer - Output data sent to MDIO Master during read frames"]
pub type Dout13W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout13(&self) -> Dout13R {
        Dout13R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout13(&mut self) -> Dout13W<MdiosDoutr13Spec> {
        Dout13W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 13\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr13::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr13::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr13Spec;
impl crate::RegisterSpec for MdiosDoutr13Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr13::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr13Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr13::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr13Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR13 to value 0"]
impl crate::Resettable for MdiosDoutr13Spec {}
