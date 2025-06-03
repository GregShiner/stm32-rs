#[doc = "Register `DSI_FIR1` writer"]
pub type W = crate::W<DsiFir1Spec>;
#[doc = "Field `FTOHSTX` writer - FTOHSTX"]
pub type FtohstxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTOLPRX` writer - FTOLPRX"]
pub type FtolprxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FECCSE` writer - FECCSE"]
pub type FeccseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FECCME` writer - FECCME"]
pub type FeccmeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FCRCE` writer - FCRCE"]
pub type FcrceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FPSE` writer - FPSE"]
pub type FpseW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FEOTPE` writer - FEOTPE"]
pub type FeotpeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLPWRE` writer - FLPWRE"]
pub type FlpwreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGCWRE` writer - FGCWRE"]
pub type FgcwreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGPWRE` writer - FGPWRE"]
pub type FgpwreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGPTXE` writer - FGPTXE"]
pub type FgptxeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGPRDE` writer - FGPRDE"]
pub type FgprdeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FGPRXE` writer - FGPRXE"]
pub type FgprxeW<'a, REG> = crate::BitWriter<'a, REG>;
impl W {
    #[doc = "Bit 0 - FTOHSTX"]
    #[inline(always)]
    pub fn ftohstx(&mut self) -> FtohstxW<DsiFir1Spec> {
        FtohstxW::new(self, 0)
    }
    #[doc = "Bit 1 - FTOLPRX"]
    #[inline(always)]
    pub fn ftolprx(&mut self) -> FtolprxW<DsiFir1Spec> {
        FtolprxW::new(self, 1)
    }
    #[doc = "Bit 2 - FECCSE"]
    #[inline(always)]
    pub fn feccse(&mut self) -> FeccseW<DsiFir1Spec> {
        FeccseW::new(self, 2)
    }
    #[doc = "Bit 3 - FECCME"]
    #[inline(always)]
    pub fn feccme(&mut self) -> FeccmeW<DsiFir1Spec> {
        FeccmeW::new(self, 3)
    }
    #[doc = "Bit 4 - FCRCE"]
    #[inline(always)]
    pub fn fcrce(&mut self) -> FcrceW<DsiFir1Spec> {
        FcrceW::new(self, 4)
    }
    #[doc = "Bit 5 - FPSE"]
    #[inline(always)]
    pub fn fpse(&mut self) -> FpseW<DsiFir1Spec> {
        FpseW::new(self, 5)
    }
    #[doc = "Bit 6 - FEOTPE"]
    #[inline(always)]
    pub fn feotpe(&mut self) -> FeotpeW<DsiFir1Spec> {
        FeotpeW::new(self, 6)
    }
    #[doc = "Bit 7 - FLPWRE"]
    #[inline(always)]
    pub fn flpwre(&mut self) -> FlpwreW<DsiFir1Spec> {
        FlpwreW::new(self, 7)
    }
    #[doc = "Bit 8 - FGCWRE"]
    #[inline(always)]
    pub fn fgcwre(&mut self) -> FgcwreW<DsiFir1Spec> {
        FgcwreW::new(self, 8)
    }
    #[doc = "Bit 9 - FGPWRE"]
    #[inline(always)]
    pub fn fgpwre(&mut self) -> FgpwreW<DsiFir1Spec> {
        FgpwreW::new(self, 9)
    }
    #[doc = "Bit 10 - FGPTXE"]
    #[inline(always)]
    pub fn fgptxe(&mut self) -> FgptxeW<DsiFir1Spec> {
        FgptxeW::new(self, 10)
    }
    #[doc = "Bit 11 - FGPRDE"]
    #[inline(always)]
    pub fn fgprde(&mut self) -> FgprdeW<DsiFir1Spec> {
        FgprdeW::new(self, 11)
    }
    #[doc = "Bit 12 - FGPRXE"]
    #[inline(always)]
    pub fn fgprxe(&mut self) -> FgprxeW<DsiFir1Spec> {
        FgprxeW::new(self, 12)
    }
}
#[doc = "DSI Host force interrupt register 1\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_fir1::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiFir1Spec;
impl crate::RegisterSpec for DsiFir1Spec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`dsi_fir1::W`](W) writer structure"]
impl crate::Writable for DsiFir1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_FIR1 to value 0"]
impl crate::Resettable for DsiFir1Spec {}
