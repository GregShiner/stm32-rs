#[doc = "Register `CONFR3` reader"]
pub type R = crate::R<Confr3Spec>;
#[doc = "Register `CONFR3` writer"]
pub type W = crate::W<Confr3Spec>;
#[doc = "Field `XSIZE` reader - X size This field defines the number of pixels per line."]
pub type XsizeR = crate::FieldReader<u16>;
#[doc = "Field `XSIZE` writer - X size This field defines the number of pixels per line."]
pub type XsizeW<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 16:31 - X size This field defines the number of pixels per line."]
    #[inline(always)]
    pub fn xsize(&self) -> XsizeR {
        XsizeR::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 16:31 - X size This field defines the number of pixels per line."]
    #[inline(always)]
    pub fn xsize(&mut self) -> XsizeW<Confr3Spec> {
        XsizeW::new(self, 16)
    }
}
#[doc = "JPEG codec configuration register 3\n\nYou can [`read`](crate::Reg::read) this register and get [`confr3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`confr3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Confr3Spec;
impl crate::RegisterSpec for Confr3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`confr3::R`](R) reader structure"]
impl crate::Readable for Confr3Spec {}
#[doc = "`write(|w| ..)` method takes [`confr3::W`](W) writer structure"]
impl crate::Writable for Confr3Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONFR3 to value 0"]
impl crate::Resettable for Confr3Spec {}
