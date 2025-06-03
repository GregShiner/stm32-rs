#[doc = "Register `MAC1USTCR` reader"]
pub type R = crate::R<Mac1ustcrSpec>;
#[doc = "Register `MAC1USTCR` writer"]
pub type W = crate::W<Mac1ustcrSpec>;
#[doc = "Field `TIC_1US_CNTR` reader - TIC_1US_CNTR"]
pub type Tic1usCntrR = crate::FieldReader<u16>;
#[doc = "Field `TIC_1US_CNTR` writer - TIC_1US_CNTR"]
pub type Tic1usCntrW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - TIC_1US_CNTR"]
    #[inline(always)]
    pub fn tic_1us_cntr(&self) -> Tic1usCntrR {
        Tic1usCntrR::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - TIC_1US_CNTR"]
    #[inline(always)]
    pub fn tic_1us_cntr(&mut self) -> Tic1usCntrW<Mac1ustcrSpec> {
        Tic1usCntrW::new(self, 0)
    }
}
#[doc = "1-microsecond-tick counter register\n\nYou can [`read`](crate::Reg::read) this register and get [`mac1ustcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mac1ustcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Mac1ustcrSpec;
impl crate::RegisterSpec for Mac1ustcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mac1ustcr::R`](R) reader structure"]
impl crate::Readable for Mac1ustcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mac1ustcr::W`](W) writer structure"]
impl crate::Writable for Mac1ustcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MAC1USTCR to value 0"]
impl crate::Resettable for Mac1ustcrSpec {}
