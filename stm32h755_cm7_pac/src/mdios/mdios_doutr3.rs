#[doc = "Register `MDIOS_DOUTR3` reader"]
pub type R = crate::R<MdiosDoutr3Spec>;
#[doc = "Register `MDIOS_DOUTR3` writer"]
pub type W = crate::W<MdiosDoutr3Spec>;
#[doc = "Field `DOUT3` reader - Output data sent to MDIO Master during read frames"]
pub type Dout3R = crate::FieldReader<u16>;
#[doc = "Field `DOUT3` writer - Output data sent to MDIO Master during read frames"]
pub type Dout3W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout3(&self) -> Dout3R {
        Dout3R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout3(&mut self) -> Dout3W<MdiosDoutr3Spec> {
        Dout3W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr3Spec;
impl crate::RegisterSpec for MdiosDoutr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr3::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr3Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr3::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR3 to value 0"]
impl crate::Resettable for MdiosDoutr3Spec {}
