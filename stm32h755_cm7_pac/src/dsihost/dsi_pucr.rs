#[doc = "Register `DSI_PUCR` reader"]
pub type R = crate::R<DsiPucrSpec>;
#[doc = "Register `DSI_PUCR` writer"]
pub type W = crate::W<DsiPucrSpec>;
#[doc = "Field `URCL` reader - URCL"]
pub type UrclR = crate::BitReader;
#[doc = "Field `URCL` writer - URCL"]
pub type UrclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UECL` reader - UECL"]
pub type UeclR = crate::BitReader;
#[doc = "Field `UECL` writer - UECL"]
pub type UeclW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `URDL` reader - URDL"]
pub type UrdlR = crate::BitReader;
#[doc = "Field `URDL` writer - URDL"]
pub type UrdlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `UEDL` reader - UEDL"]
pub type UedlR = crate::BitReader;
#[doc = "Field `UEDL` writer - UEDL"]
pub type UedlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - URCL"]
    #[inline(always)]
    pub fn urcl(&self) -> UrclR {
        UrclR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - UECL"]
    #[inline(always)]
    pub fn uecl(&self) -> UeclR {
        UeclR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - URDL"]
    #[inline(always)]
    pub fn urdl(&self) -> UrdlR {
        UrdlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - UEDL"]
    #[inline(always)]
    pub fn uedl(&self) -> UedlR {
        UedlR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - URCL"]
    #[inline(always)]
    pub fn urcl(&mut self) -> UrclW<DsiPucrSpec> {
        UrclW::new(self, 0)
    }
    #[doc = "Bit 1 - UECL"]
    #[inline(always)]
    pub fn uecl(&mut self) -> UeclW<DsiPucrSpec> {
        UeclW::new(self, 1)
    }
    #[doc = "Bit 2 - URDL"]
    #[inline(always)]
    pub fn urdl(&mut self) -> UrdlW<DsiPucrSpec> {
        UrdlW::new(self, 2)
    }
    #[doc = "Bit 3 - UEDL"]
    #[inline(always)]
    pub fn uedl(&mut self) -> UedlW<DsiPucrSpec> {
        UedlW::new(self, 3)
    }
}
#[doc = "DSI Host PHY ULPS control register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_pucr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_pucr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiPucrSpec;
impl crate::RegisterSpec for DsiPucrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_pucr::R`](R) reader structure"]
impl crate::Readable for DsiPucrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_pucr::W`](W) writer structure"]
impl crate::Writable for DsiPucrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_PUCR to value 0"]
impl crate::Resettable for DsiPucrSpec {}
