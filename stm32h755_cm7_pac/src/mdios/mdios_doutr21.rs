#[doc = "Register `MDIOS_DOUTR21` reader"]
pub type R = crate::R<MdiosDoutr21Spec>;
#[doc = "Register `MDIOS_DOUTR21` writer"]
pub type W = crate::W<MdiosDoutr21Spec>;
#[doc = "Field `DOUT21` reader - Output data sent to MDIO Master during read frames"]
pub type Dout21R = crate::FieldReader<u16>;
#[doc = "Field `DOUT21` writer - Output data sent to MDIO Master during read frames"]
pub type Dout21W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout21(&self) -> Dout21R {
        Dout21R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout21(&mut self) -> Dout21W<MdiosDoutr21Spec> {
        Dout21W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 21\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr21::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr21::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr21Spec;
impl crate::RegisterSpec for MdiosDoutr21Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr21::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr21Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr21::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr21Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR21 to value 0"]
impl crate::Resettable for MdiosDoutr21Spec {}
