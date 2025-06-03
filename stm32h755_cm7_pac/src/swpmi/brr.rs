#[doc = "Register `BRR` reader"]
pub type R = crate::R<BrrSpec>;
#[doc = "Register `BRR` writer"]
pub type W = crate::W<BrrSpec>;
#[doc = "Field `BR` reader - Bitrate prescaler"]
pub type BrR = crate::FieldReader;
#[doc = "Field `BR` writer - Bitrate prescaler"]
pub type BrW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Bitrate prescaler"]
    #[inline(always)]
    pub fn br(&self) -> BrR {
        BrR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Bitrate prescaler"]
    #[inline(always)]
    pub fn br(&mut self) -> BrW<BrrSpec> {
        BrW::new(self, 0)
    }
}
#[doc = "SWPMI Bitrate register\n\nYou can [`read`](crate::Reg::read) this register and get [`brr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BrrSpec;
impl crate::RegisterSpec for BrrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`brr::R`](R) reader structure"]
impl crate::Readable for BrrSpec {}
#[doc = "`write(|w| ..)` method takes [`brr::W`](W) writer structure"]
impl crate::Writable for BrrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets BRR to value 0x01"]
impl crate::Resettable for BrrSpec {
    const RESET_VALUE: u32 = 0x01;
}
