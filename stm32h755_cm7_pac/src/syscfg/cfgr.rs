#[doc = "Register `CFGR` reader"]
pub type R = crate::R<CfgrSpec>;
#[doc = "Register `CFGR` writer"]
pub type W = crate::W<CfgrSpec>;
#[doc = "Field `CM4L` reader - CM4L"]
pub type Cm4lR = crate::BitReader;
#[doc = "Field `CM4L` writer - CM4L"]
pub type Cm4lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PVDL` reader - PVDL"]
pub type PvdlR = crate::BitReader;
#[doc = "Field `PVDL` writer - PVDL"]
pub type PvdlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FLASHL` reader - FLASHL"]
pub type FlashlR = crate::BitReader;
#[doc = "Field `FLASHL` writer - FLASHL"]
pub type FlashlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CM7L` reader - CM7L"]
pub type Cm7lR = crate::BitReader;
#[doc = "Field `CM7L` writer - CM7L"]
pub type Cm7lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BKRAML` reader - BKRAML"]
pub type BkramlR = crate::BitReader;
#[doc = "Field `BKRAML` writer - BKRAML"]
pub type BkramlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM4L` reader - SRAM4L"]
pub type Sram4lR = crate::BitReader;
#[doc = "Field `SRAM4L` writer - SRAM4L"]
pub type Sram4lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM3L` reader - SRAM3L"]
pub type Sram3lR = crate::BitReader;
#[doc = "Field `SRAM3L` writer - SRAM3L"]
pub type Sram3lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM2L` reader - SRAM2L"]
pub type Sram2lR = crate::BitReader;
#[doc = "Field `SRAM2L` writer - SRAM2L"]
pub type Sram2lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SRAM1L` reader - SRAM1L"]
pub type Sram1lR = crate::BitReader;
#[doc = "Field `SRAM1L` writer - SRAM1L"]
pub type Sram1lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTCML` reader - DTCML"]
pub type DtcmlR = crate::BitReader;
#[doc = "Field `DTCML` writer - DTCML"]
pub type DtcmlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ITCML` reader - ITCML"]
pub type ItcmlR = crate::BitReader;
#[doc = "Field `ITCML` writer - ITCML"]
pub type ItcmlW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AXISRAML` reader - AXISRAML"]
pub type AxisramlR = crate::BitReader;
#[doc = "Field `AXISRAML` writer - AXISRAML"]
pub type AxisramlW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CM4L"]
    #[inline(always)]
    pub fn cm4l(&self) -> Cm4lR {
        Cm4lR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - PVDL"]
    #[inline(always)]
    pub fn pvdl(&self) -> PvdlR {
        PvdlR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FLASHL"]
    #[inline(always)]
    pub fn flashl(&self) -> FlashlR {
        FlashlR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - CM7L"]
    #[inline(always)]
    pub fn cm7l(&self) -> Cm7lR {
        Cm7lR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - BKRAML"]
    #[inline(always)]
    pub fn bkraml(&self) -> BkramlR {
        BkramlR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - SRAM4L"]
    #[inline(always)]
    pub fn sram4l(&self) -> Sram4lR {
        Sram4lR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - SRAM3L"]
    #[inline(always)]
    pub fn sram3l(&self) -> Sram3lR {
        Sram3lR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SRAM2L"]
    #[inline(always)]
    pub fn sram2l(&self) -> Sram2lR {
        Sram2lR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - SRAM1L"]
    #[inline(always)]
    pub fn sram1l(&self) -> Sram1lR {
        Sram1lR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DTCML"]
    #[inline(always)]
    pub fn dtcml(&self) -> DtcmlR {
        DtcmlR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - ITCML"]
    #[inline(always)]
    pub fn itcml(&self) -> ItcmlR {
        ItcmlR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - AXISRAML"]
    #[inline(always)]
    pub fn axisraml(&self) -> AxisramlR {
        AxisramlR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CM4L"]
    #[inline(always)]
    pub fn cm4l(&mut self) -> Cm4lW<CfgrSpec> {
        Cm4lW::new(self, 0)
    }
    #[doc = "Bit 2 - PVDL"]
    #[inline(always)]
    pub fn pvdl(&mut self) -> PvdlW<CfgrSpec> {
        PvdlW::new(self, 2)
    }
    #[doc = "Bit 3 - FLASHL"]
    #[inline(always)]
    pub fn flashl(&mut self) -> FlashlW<CfgrSpec> {
        FlashlW::new(self, 3)
    }
    #[doc = "Bit 6 - CM7L"]
    #[inline(always)]
    pub fn cm7l(&mut self) -> Cm7lW<CfgrSpec> {
        Cm7lW::new(self, 6)
    }
    #[doc = "Bit 7 - BKRAML"]
    #[inline(always)]
    pub fn bkraml(&mut self) -> BkramlW<CfgrSpec> {
        BkramlW::new(self, 7)
    }
    #[doc = "Bit 9 - SRAM4L"]
    #[inline(always)]
    pub fn sram4l(&mut self) -> Sram4lW<CfgrSpec> {
        Sram4lW::new(self, 9)
    }
    #[doc = "Bit 10 - SRAM3L"]
    #[inline(always)]
    pub fn sram3l(&mut self) -> Sram3lW<CfgrSpec> {
        Sram3lW::new(self, 10)
    }
    #[doc = "Bit 11 - SRAM2L"]
    #[inline(always)]
    pub fn sram2l(&mut self) -> Sram2lW<CfgrSpec> {
        Sram2lW::new(self, 11)
    }
    #[doc = "Bit 12 - SRAM1L"]
    #[inline(always)]
    pub fn sram1l(&mut self) -> Sram1lW<CfgrSpec> {
        Sram1lW::new(self, 12)
    }
    #[doc = "Bit 13 - DTCML"]
    #[inline(always)]
    pub fn dtcml(&mut self) -> DtcmlW<CfgrSpec> {
        DtcmlW::new(self, 13)
    }
    #[doc = "Bit 14 - ITCML"]
    #[inline(always)]
    pub fn itcml(&mut self) -> ItcmlW<CfgrSpec> {
        ItcmlW::new(self, 14)
    }
    #[doc = "Bit 15 - AXISRAML"]
    #[inline(always)]
    pub fn axisraml(&mut self) -> AxisramlW<CfgrSpec> {
        AxisramlW::new(self, 15)
    }
}
#[doc = "configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cfgr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfgr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CfgrSpec;
impl crate::RegisterSpec for CfgrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cfgr::R`](R) reader structure"]
impl crate::Readable for CfgrSpec {}
#[doc = "`write(|w| ..)` method takes [`cfgr::W`](W) writer structure"]
impl crate::Writable for CfgrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CfgrSpec {}
