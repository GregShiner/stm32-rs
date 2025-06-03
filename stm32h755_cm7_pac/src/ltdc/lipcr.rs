#[doc = "Register `LIPCR` reader"]
pub type R = crate::R<LipcrSpec>;
#[doc = "Register `LIPCR` writer"]
pub type W = crate::W<LipcrSpec>;
#[doc = "Field `LIPOS` reader - Line Interrupt Position"]
pub type LiposR = crate::FieldReader<u16>;
#[doc = "Field `LIPOS` writer - Line Interrupt Position"]
pub type LiposW<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10 - Line Interrupt Position"]
    #[inline(always)]
    pub fn lipos(&self) -> LiposR {
        LiposR::new((self.bits & 0x07ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:10 - Line Interrupt Position"]
    #[inline(always)]
    pub fn lipos(&mut self) -> LiposW<LipcrSpec> {
        LiposW::new(self, 0)
    }
}
#[doc = "Line Interrupt Position Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`lipcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lipcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LipcrSpec;
impl crate::RegisterSpec for LipcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lipcr::R`](R) reader structure"]
impl crate::Readable for LipcrSpec {}
#[doc = "`write(|w| ..)` method takes [`lipcr::W`](W) writer structure"]
impl crate::Writable for LipcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LIPCR to value 0"]
impl crate::Resettable for LipcrSpec {}
