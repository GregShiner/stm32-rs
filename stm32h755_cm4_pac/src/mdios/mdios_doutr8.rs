#[doc = "Register `MDIOS_DOUTR8` reader"]
pub type R = crate::R<MdiosDoutr8Spec>;
#[doc = "Register `MDIOS_DOUTR8` writer"]
pub type W = crate::W<MdiosDoutr8Spec>;
#[doc = "Field `DOUT8` reader - Output data sent to MDIO Master during read frames"]
pub type Dout8R = crate::FieldReader<u16>;
#[doc = "Field `DOUT8` writer - Output data sent to MDIO Master during read frames"]
pub type Dout8W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout8(&self) -> Dout8R {
        Dout8R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout8(&mut self) -> Dout8W<MdiosDoutr8Spec> {
        Dout8W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 8\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr8Spec;
impl crate::RegisterSpec for MdiosDoutr8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr8::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr8Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr8::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR8 to value 0"]
impl crate::Resettable for MdiosDoutr8Spec {}
