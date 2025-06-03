#[doc = "Register `L2CACR` reader"]
pub type R = crate::R<L2cacrSpec>;
#[doc = "Register `L2CACR` writer"]
pub type W = crate::W<L2cacrSpec>;
#[doc = "Field `CONSTA` reader - Constant Alpha"]
pub type ConstaR = crate::FieldReader;
#[doc = "Field `CONSTA` writer - Constant Alpha"]
pub type ConstaW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Constant Alpha"]
    #[inline(always)]
    pub fn consta(&self) -> ConstaR {
        ConstaR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Constant Alpha"]
    #[inline(always)]
    pub fn consta(&mut self) -> ConstaW<L2cacrSpec> {
        ConstaW::new(self, 0)
    }
}
#[doc = "Layerx Constant Alpha Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`l2cacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l2cacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L2cacrSpec;
impl crate::RegisterSpec for L2cacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l2cacr::R`](R) reader structure"]
impl crate::Readable for L2cacrSpec {}
#[doc = "`write(|w| ..)` method takes [`l2cacr::W`](W) writer structure"]
impl crate::Writable for L2cacrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L2CACR to value 0"]
impl crate::Resettable for L2cacrSpec {}
