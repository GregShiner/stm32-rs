#[doc = "Register `MDIOS_DOUTR0` reader"]
pub type R = crate::R<MdiosDoutr0Spec>;
#[doc = "Register `MDIOS_DOUTR0` writer"]
pub type W = crate::W<MdiosDoutr0Spec>;
#[doc = "Field `DOUT0` reader - Output data sent to MDIO Master during read frames"]
pub type Dout0R = crate::FieldReader<u16>;
#[doc = "Field `DOUT0` writer - Output data sent to MDIO Master during read frames"]
pub type Dout0W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout0(&self) -> Dout0R {
        Dout0R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout0(&mut self) -> Dout0W<MdiosDoutr0Spec> {
        Dout0W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 0\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr0Spec;
impl crate::RegisterSpec for MdiosDoutr0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr0::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr0Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr0::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr0Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR0 to value 0"]
impl crate::Resettable for MdiosDoutr0Spec {}
