#[doc = "Register `MDIOS_DOUTR24` reader"]
pub type R = crate::R<MdiosDoutr24Spec>;
#[doc = "Register `MDIOS_DOUTR24` writer"]
pub type W = crate::W<MdiosDoutr24Spec>;
#[doc = "Field `DOUT24` reader - Output data sent to MDIO Master during read frames"]
pub type Dout24R = crate::FieldReader<u16>;
#[doc = "Field `DOUT24` writer - Output data sent to MDIO Master during read frames"]
pub type Dout24W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout24(&self) -> Dout24R {
        Dout24R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout24(&mut self) -> Dout24W<MdiosDoutr24Spec> {
        Dout24W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 24\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr24::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr24::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr24Spec;
impl crate::RegisterSpec for MdiosDoutr24Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr24::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr24Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr24::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr24Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR24 to value 0"]
impl crate::Resettable for MdiosDoutr24Spec {}
