#[doc = "Register `MDIOS_DOUTR22` reader"]
pub type R = crate::R<MdiosDoutr22Spec>;
#[doc = "Register `MDIOS_DOUTR22` writer"]
pub type W = crate::W<MdiosDoutr22Spec>;
#[doc = "Field `DOUT22` reader - Output data sent to MDIO Master during read frames"]
pub type Dout22R = crate::FieldReader<u16>;
#[doc = "Field `DOUT22` writer - Output data sent to MDIO Master during read frames"]
pub type Dout22W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout22(&self) -> Dout22R {
        Dout22R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout22(&mut self) -> Dout22W<MdiosDoutr22Spec> {
        Dout22W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 22\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr22::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr22::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr22Spec;
impl crate::RegisterSpec for MdiosDoutr22Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr22::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr22Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr22::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr22Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR22 to value 0"]
impl crate::Resettable for MdiosDoutr22Spec {}
