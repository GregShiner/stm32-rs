#[doc = "Register `L1CACR` reader"]
pub type R = crate::R<L1cacrSpec>;
#[doc = "Register `L1CACR` writer"]
pub type W = crate::W<L1cacrSpec>;
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
    pub fn consta(&mut self) -> ConstaW<L1cacrSpec> {
        ConstaW::new(self, 0)
    }
}
#[doc = "Layerx Constant Alpha Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`l1cacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`l1cacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct L1cacrSpec;
impl crate::RegisterSpec for L1cacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`l1cacr::R`](R) reader structure"]
impl crate::Readable for L1cacrSpec {}
#[doc = "`write(|w| ..)` method takes [`l1cacr::W`](W) writer structure"]
impl crate::Writable for L1cacrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets L1CACR to value 0"]
impl crate::Resettable for L1cacrSpec {}
