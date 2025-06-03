#[doc = "Register `HSICFGR` reader"]
pub type R = crate::R<HsicfgrSpec>;
#[doc = "Register `HSICFGR` writer"]
pub type W = crate::W<HsicfgrSpec>;
#[doc = "Field `HSICAL` reader - HSI clock calibration"]
pub type HsicalR = crate::FieldReader<u16>;
#[doc = "Field `HSITRIM` reader - HSI clock trimming"]
pub type HsitrimR = crate::FieldReader;
#[doc = "Field `HSITRIM` writer - HSI clock trimming"]
pub type HsitrimW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 0:11 - HSI clock calibration"]
    #[inline(always)]
    pub fn hsical(&self) -> HsicalR {
        HsicalR::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 24:30 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&self) -> HsitrimR {
        HsitrimR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 24:30 - HSI clock trimming"]
    #[inline(always)]
    pub fn hsitrim(&mut self) -> HsitrimW<HsicfgrSpec> {
        HsitrimW::new(self, 24)
    }
}
#[doc = "RCC HSI configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`hsicfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hsicfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HsicfgrSpec;
impl crate::RegisterSpec for HsicfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hsicfgr::R`](R) reader structure"]
impl crate::Readable for HsicfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`hsicfgr::W`](W) writer structure"]
impl crate::Writable for HsicfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HSICFGR to value 0x4000_0000"]
impl crate::Resettable for HsicfgrSpec {
    const RESET_VALUE: u32 = 0x4000_0000;
}
