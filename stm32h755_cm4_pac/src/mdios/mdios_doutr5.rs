#[doc = "Register `MDIOS_DOUTR5` reader"]
pub type R = crate::R<MdiosDoutr5Spec>;
#[doc = "Register `MDIOS_DOUTR5` writer"]
pub type W = crate::W<MdiosDoutr5Spec>;
#[doc = "Field `DOUT5` reader - Output data sent to MDIO Master during read frames"]
pub type Dout5R = crate::FieldReader<u16>;
#[doc = "Field `DOUT5` writer - Output data sent to MDIO Master during read frames"]
pub type Dout5W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout5(&self) -> Dout5R {
        Dout5R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout5(&mut self) -> Dout5W<MdiosDoutr5Spec> {
        Dout5W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 5\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr5::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr5::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr5Spec;
impl crate::RegisterSpec for MdiosDoutr5Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr5::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr5Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr5::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr5Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR5 to value 0"]
impl crate::Resettable for MdiosDoutr5Spec {}
