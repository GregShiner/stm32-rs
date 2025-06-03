#[doc = "Register `MDIOS_DOUTR9` reader"]
pub type R = crate::R<MdiosDoutr9Spec>;
#[doc = "Register `MDIOS_DOUTR9` writer"]
pub type W = crate::W<MdiosDoutr9Spec>;
#[doc = "Field `DOUT9` reader - Output data sent to MDIO Master during read frames"]
pub type Dout9R = crate::FieldReader<u16>;
#[doc = "Field `DOUT9` writer - Output data sent to MDIO Master during read frames"]
pub type Dout9W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout9(&self) -> Dout9R {
        Dout9R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout9(&mut self) -> Dout9W<MdiosDoutr9Spec> {
        Dout9W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 9\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr9::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr9::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr9Spec;
impl crate::RegisterSpec for MdiosDoutr9Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr9::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr9Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr9::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr9Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR9 to value 0"]
impl crate::Resettable for MdiosDoutr9Spec {}
