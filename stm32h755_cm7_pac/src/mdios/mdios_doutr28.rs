#[doc = "Register `MDIOS_DOUTR28` reader"]
pub type R = crate::R<MdiosDoutr28Spec>;
#[doc = "Register `MDIOS_DOUTR28` writer"]
pub type W = crate::W<MdiosDoutr28Spec>;
#[doc = "Field `DOUT28` reader - Output data sent to MDIO Master during read frames"]
pub type Dout28R = crate::FieldReader<u16>;
#[doc = "Field `DOUT28` writer - Output data sent to MDIO Master during read frames"]
pub type Dout28W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout28(&self) -> Dout28R {
        Dout28R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout28(&mut self) -> Dout28W<MdiosDoutr28Spec> {
        Dout28W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 28\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr28::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr28::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr28Spec;
impl crate::RegisterSpec for MdiosDoutr28Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr28::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr28Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr28::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr28Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR28 to value 0"]
impl crate::Resettable for MdiosDoutr28Spec {}
