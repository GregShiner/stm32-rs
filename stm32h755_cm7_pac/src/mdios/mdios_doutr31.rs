#[doc = "Register `MDIOS_DOUTR31` reader"]
pub type R = crate::R<MdiosDoutr31Spec>;
#[doc = "Register `MDIOS_DOUTR31` writer"]
pub type W = crate::W<MdiosDoutr31Spec>;
#[doc = "Field `DOUT31` reader - Output data sent to MDIO Master during read frames"]
pub type Dout31R = crate::FieldReader<u16>;
#[doc = "Field `DOUT31` writer - Output data sent to MDIO Master during read frames"]
pub type Dout31W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout31(&self) -> Dout31R {
        Dout31R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Output data sent to MDIO Master during read frames"]
    #[inline(always)]
    pub fn dout31(&mut self) -> Dout31W<MdiosDoutr31Spec> {
        Dout31W::new(self, 0)
    }
}
#[doc = "MDIOS output data register 31\n\nYou can [`read`](crate::Reg::read) this register and get [`mdios_doutr31::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdios_doutr31::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdiosDoutr31Spec;
impl crate::RegisterSpec for MdiosDoutr31Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdios_doutr31::R`](R) reader structure"]
impl crate::Readable for MdiosDoutr31Spec {}
#[doc = "`write(|w| ..)` method takes [`mdios_doutr31::W`](W) writer structure"]
impl crate::Writable for MdiosDoutr31Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDIOS_DOUTR31 to value 0"]
impl crate::Resettable for MdiosDoutr31Spec {}
