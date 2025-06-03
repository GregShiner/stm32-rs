#[doc = "Register `CFR` reader"]
pub type R = crate::R<CfrSpec>;
#[doc = "Register `CFR` writer"]
pub type W = crate::W<CfrSpec>;
#[doc = "Field `CEOCF` reader - Clear End of Conversion Flag Writing 1 clears the End of Conversion Flag of the JPEG Status Register."]
pub type CeocfR = crate::BitReader;
#[doc = "Field `CEOCF` writer - Clear End of Conversion Flag Writing 1 clears the End of Conversion Flag of the JPEG Status Register."]
pub type CeocfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CHPDF` reader - Clear Header Parsing Done Flag Writing 1 clears the Header Parsing Done Flag of the JPEG Status Register."]
pub type ChpdfR = crate::BitReader;
#[doc = "Field `CHPDF` writer - Clear Header Parsing Done Flag Writing 1 clears the Header Parsing Done Flag of the JPEG Status Register."]
pub type ChpdfW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 5 - Clear End of Conversion Flag Writing 1 clears the End of Conversion Flag of the JPEG Status Register."]
    #[inline(always)]
    pub fn ceocf(&self) -> CeocfR {
        CeocfR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Clear Header Parsing Done Flag Writing 1 clears the Header Parsing Done Flag of the JPEG Status Register."]
    #[inline(always)]
    pub fn chpdf(&self) -> ChpdfR {
        ChpdfR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - Clear End of Conversion Flag Writing 1 clears the End of Conversion Flag of the JPEG Status Register."]
    #[inline(always)]
    pub fn ceocf(&mut self) -> CeocfW<CfrSpec> {
        CeocfW::new(self, 5)
    }
    #[doc = "Bit 6 - Clear Header Parsing Done Flag Writing 1 clears the Header Parsing Done Flag of the JPEG Status Register."]
    #[inline(always)]
    pub fn chpdf(&mut self) -> ChpdfW<CfrSpec> {
        ChpdfW::new(self, 6)
    }
}
#[doc = "JPEG clear flag register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfrSpec;
impl crate::RegisterSpec for CfrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfr::R`](R) reader structure"]
impl crate::Readable for CfrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfr::W`](W) writer structure"]
impl crate::Writable for CfrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFR to value 0"]
impl crate::Resettable for CfrSpec {}
