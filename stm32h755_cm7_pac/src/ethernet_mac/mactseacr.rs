#[doc = "Register `MACTSEACR` reader"]
pub type R = crate::R<MactseacrSpec>;
#[doc = "Register `MACTSEACR` writer"]
pub type W = crate::W<MactseacrSpec>;
#[doc = "Field `OSTEAC` reader - OSTEAC"]
pub type OsteacR = crate::FieldReader<u32>;
#[doc = "Field `OSTEAC` writer - OSTEAC"]
pub type OsteacW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - OSTEAC"]
    #[inline(always)]
    pub fn osteac(&self) -> OsteacR {
        OsteacR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - OSTEAC"]
    #[inline(always)]
    pub fn osteac(&mut self) -> OsteacW<MactseacrSpec> {
        OsteacW::new(self, 0)
    }
}
#[doc = "Timestamp Egress asymmetric correction register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactseacr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactseacr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MactseacrSpec;
impl crate::RegisterSpec for MactseacrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactseacr::R`](R) reader structure"]
impl crate::Readable for MactseacrSpec {}
#[doc = "`write(|w| ..)` method takes [`mactseacr::W`](W) writer structure"]
impl crate::Writable for MactseacrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACTSEACR to value 0"]
impl crate::Resettable for MactseacrSpec {}
