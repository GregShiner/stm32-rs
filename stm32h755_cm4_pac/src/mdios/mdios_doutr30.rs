#[doc = "Register `MDIOS_DOUTR30` reader"]
pub type R = crate::R<MdiosDoutr30Spec>;
#[doc = "Register `MDIOS_DOUTR30` writer"]
pub type W = crate::W<MdiosDoutr30Spec>;
#[doc = "Field `DOUT30` reader - Output data sent to MDIO Master during read frames"]
pub type Dout30R = crate::FieldReader<u16>;
#[doc = "Field `DOUT30` writer - Output data sent to MDIO Master during read frames"]
pub type Dout30W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout30(&self) -> Dout30R {
        Dout30R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout30(&mut self) -> Dout30W<MdiosDoutr30Spec> {
        Dout30W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 30\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr30::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr30::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr30Spec;
impl crate::RegisterSpec for MdiosDoutr30Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr30::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr30Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr30::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr30Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR30 to value 0"]
impl crate::Resettable for MdiosDoutr30Spec {}
