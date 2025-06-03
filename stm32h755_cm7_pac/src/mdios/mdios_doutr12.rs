#[doc = "Register `MDIOS_DOUTR12` reader"]
pub type R = crate::R<MdiosDoutr12Spec>;
#[doc = "Register `MDIOS_DOUTR12` writer"]
pub type W = crate::W<MdiosDoutr12Spec>;
#[doc = "Field `DOUT12` reader - Output data sent to MDIO Master during read frames"]
pub type Dout12R = crate::FieldReader<u16>;
#[doc = "Field `DOUT12` writer - Output data sent to MDIO Master during read frames"]
pub type Dout12W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout12(&self) -> Dout12R {
        Dout12R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout12(&mut self) -> Dout12W<MdiosDoutr12Spec> {
        Dout12W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 12\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr12::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr12::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr12Spec;
impl crate::RegisterSpec for MdiosDoutr12Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr12::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr12Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr12::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr12Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR12 to value 0"]
impl crate::Resettable for MdiosDoutr12Spec {}
