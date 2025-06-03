#[doc = "Register `MDIOS_DOUTR19` reader"]
pub type R = crate::R<MdiosDoutr19Spec>;
#[doc = "Register `MDIOS_DOUTR19` writer"]
pub type W = crate::W<MdiosDoutr19Spec>;
#[doc = "Field `DOUT19` reader - Output data sent to MDIO Master during read frames"]
pub type Dout19R = crate::FieldReader<u16>;
#[doc = "Field `DOUT19` writer - Output data sent to MDIO Master during read frames"]
pub type Dout19W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout19(&self) -> Dout19R {
        Dout19R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout19(&mut self) -> Dout19W<MdiosDoutr19Spec> {
        Dout19W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 19\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr19::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr19::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr19Spec;
impl crate::RegisterSpec for MdiosDoutr19Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr19::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr19Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr19::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr19Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR19 to value 0"]
impl crate::Resettable for MdiosDoutr19Spec {}
