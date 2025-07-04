#[doc = "Register `CHPAR` reader"]
pub type R = crate::R<ChparSpec>;
#[doc = "Register `CHPAR` writer"]
pub type W = crate::W<ChparSpec>;
#[doc = "Field `CHPFRQ` reader - Timerx carrier frequency value"]
pub type ChpfrqR = crate::FieldReader;
#[doc = "Field `CHPFRQ` writer - Timerx carrier frequency value"]
pub type ChpfrqW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `CHPDTY` reader - Timerx chopper duty cycle value"]
pub type ChpdtyR = crate::FieldReader;
#[doc = "Field `CHPDTY` writer - Timerx chopper duty cycle value"]
pub type ChpdtyW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `STRTPW` reader - STRTPW"]
pub type StrtpwR = crate::FieldReader;
#[doc = "Field `STRTPW` writer - STRTPW"]
pub type StrtpwW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    pub fn chpfrq(&self) -> ChpfrqR {
        ChpfrqR::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Timerx chopper duty cycle value"]
    #[inline(always)]
    pub fn chpdty(&self) -> ChpdtyR {
        ChpdtyR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:10 - STRTPW"]
    #[inline(always)]
    pub fn strtpw(&self) -> StrtpwR {
        StrtpwR::new(((self.bits >> 7) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Timerx carrier frequency value"]
    #[inline(always)]
    pub fn chpfrq(&mut self) -> ChpfrqW<ChparSpec> {
        ChpfrqW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Timerx chopper duty cycle value"]
    #[inline(always)]
    pub fn chpdty(&mut self) -> ChpdtyW<ChparSpec> {
        ChpdtyW::new(self, 4)
    }
    #[doc = "Bits 7:10 - STRTPW"]
    #[inline(always)]
    pub fn strtpw(&mut self) -> StrtpwW<ChparSpec> {
        StrtpwW::new(self, 7)
    }
}
#[doc = "Timerx Chopper Register\n\nYou can [`read`](crate::Reg::read) this register and get [`chpar::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chpar::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ChparSpec;
impl crate::RegisterSpec for ChparSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chpar::R`](R) reader structure"]
impl crate::Readable for ChparSpec {}
#[doc = "`write(|w| ..)` method takes [`chpar::W`](W) writer structure"]
impl crate::Writable for ChparSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CHPAR to value 0"]
impl crate::Resettable for ChparSpec {}
