#[doc = "Register `MDIOS_DOUTR25` reader"]
pub type R = crate::R<MdiosDoutr25Spec>;
#[doc = "Register `MDIOS_DOUTR25` writer"]
pub type W = crate::W<MdiosDoutr25Spec>;
#[doc = "Field `DOUT25` reader - Output data sent to MDIO Master during read frames"]
pub type Dout25R = crate::FieldReader<u16>;
#[doc = "Field `DOUT25` writer - Output data sent to MDIO Master during read frames"]
pub type Dout25W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout25(&self) -> Dout25R {
        Dout25R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout25(&mut self) -> Dout25W<MdiosDoutr25Spec> {
        Dout25W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 25\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr25::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr25::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr25Spec;
impl crate::RegisterSpec for MdiosDoutr25Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr25::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr25Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr25::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr25Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR25 to value 0"]
impl crate::Resettable for MdiosDoutr25Spec {}
