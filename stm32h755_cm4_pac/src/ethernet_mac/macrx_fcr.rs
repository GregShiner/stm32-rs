#[doc = "Register `MACRxFCR` reader"]
pub type R = crate::R<MacrxFcrSpec>;
#[doc = "Register `MACRxFCR` writer"]
pub type W = crate::W<MacrxFcrSpec>;
#[doc = "Field `RFE` reader - RFE"]
pub type RfeR = crate::BitReader;
#[doc = "Field `RFE` writer - RFE"]
pub type RfeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UP` reader - UP"]
pub type UpR = crate::BitReader;
#[doc = "Field `UP` writer - UP"]
pub type UpW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RFE"]
    #[inline(always)]
    pub fn rfe(&self) -> RfeR {
        RfeR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UP"]
    #[inline(always)]
    pub fn up(&self) -> UpR {
        UpR::new(((self.bits >> 1) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RFE"]
    #[inline(always)]
    pub fn rfe(&mut self) -> RfeW<MacrxFcrSpec> {
        RfeW::new(self, 0)
    }
    #[doc = "Bit 1 - UP"]
    #[inline(always)]
    pub fn up(&mut self) -> UpW<MacrxFcrSpec> {
        UpW::new(self, 1)
    }
}
#[doc = "Rx flow control register\n\nYou can [`read`](crate::Reg::read) this register and get [`macrx_fcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`macrx_fcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MacrxFcrSpec;
impl crate::RegisterSpec for MacrxFcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`macrx_fcr::R`](R) reader structure"]
impl crate::Readable for MacrxFcrSpec {}
#[doc = "`write(|w| ..)` method takes [`macrx_fcr::W`](W) writer structure"]
impl crate::Writable for MacrxFcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACRxFCR to value 0"]
impl crate::Resettable for MacrxFcrSpec {}
