#[doc = "Register `MDIOS_DOUTR11` reader"]
pub type R = crate::R<MdiosDoutr11Spec>;
#[doc = "Register `MDIOS_DOUTR11` writer"]
pub type W = crate::W<MdiosDoutr11Spec>;
#[doc = "Field `DOUT11` reader - Output data sent to MDIO Master during read frames"]
pub type Dout11R = crate::FieldReader<u16>;
#[doc = "Field `DOUT11` writer - Output data sent to MDIO Master during read frames"]
pub type Dout11W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout11(&self) -> Dout11R {
        Dout11R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout11(&mut self) -> Dout11W<MdiosDoutr11Spec> {
        Dout11W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 11\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr11::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr11::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr11Spec;
impl crate::RegisterSpec for MdiosDoutr11Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr11::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr11Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr11::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr11Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR11 to value 0"]
impl crate::Resettable for MdiosDoutr11Spec {}
