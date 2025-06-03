#[doc = "Register `MDIOS_DOUTR27` reader"]
pub type R = crate::R<MdiosDoutr27Spec>;
#[doc = "Register `MDIOS_DOUTR27` writer"]
pub type W = crate::W<MdiosDoutr27Spec>;
#[doc = "Field `DOUT27` reader - Output data sent to MDIO Master during read frames"]
pub type Dout27R = crate::FieldReader<u16>;
#[doc = "Field `DOUT27` writer - Output data sent to MDIO Master during read frames"]
pub type Dout27W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout27(&self) -> Dout27R {
        Dout27R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout27(&mut self) -> Dout27W<MdiosDoutr27Spec> {
        Dout27W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 27\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr27::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr27::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr27Spec;
impl crate::RegisterSpec for MdiosDoutr27Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr27::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr27Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr27::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr27Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR27 to value 0"]
impl crate::Resettable for MdiosDoutr27Spec {}
