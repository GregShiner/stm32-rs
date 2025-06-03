#[doc = "Register `MDIOS_DOUTR4` reader"]
pub type R = crate::R<MdiosDoutr4Spec>;
#[doc = "Register `MDIOS_DOUTR4` writer"]
pub type W = crate::W<MdiosDoutr4Spec>;
#[doc = "Field `DOUT4` reader - Output data sent to MDIO Master during read frames"]
pub type Dout4R = crate::FieldReader<u16>;
#[doc = "Field `DOUT4` writer - Output data sent to MDIO Master during read frames"]
pub type Dout4W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout4(&self) -> Dout4R {
        Dout4R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout4(&mut self) -> Dout4W<MdiosDoutr4Spec> {
        Dout4W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 4\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr4Spec;
impl crate::RegisterSpec for MdiosDoutr4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr4::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr4Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr4::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR4 to value 0"]
impl crate::Resettable for MdiosDoutr4Spec {}
