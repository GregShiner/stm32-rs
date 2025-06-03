#[doc = "Register `DSI_WPCR1` reader"]
pub type R = crate::R<DsiWpcr1Spec>;
#[doc = "Register `DSI_WPCR1` writer"]
pub type W = crate::W<DsiWpcr1Spec>;
#[doc = "Field `HSTXDCL` reader - High-speed transmission delay on clock lane"]
pub type HstxdclR = crate::FieldReader;
#[doc = "Field `HSTXDCL` writer - High-speed transmission delay on clock lane"]
pub type HstxdclW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HSTXDDL` reader - High-speed transmission delay on data lanes"]
pub type HstxddlR = crate::FieldReader;
#[doc = "Field `HSTXDDL` writer - High-speed transmission delay on data lanes"]
pub type HstxddlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPSRCCL` reader - Low-power transmission slew-rate compensation on clock lane"]
pub type LpsrcclR = crate::FieldReader;
#[doc = "Field `LPSRCCL` writer - Low-power transmission slew-rate compensation on clock lane"]
pub type LpsrcclW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LPSRCDL` reader - Low-power transmission slew-rate compensation on data lanes"]
pub type LpsrcdlR = crate::FieldReader;
#[doc = "Field `LPSRCDL` writer - Low-power transmission slew-rate compensation on data lanes"]
pub type LpsrcdlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SDDC` reader - SDD control"]
pub type SddcR = crate::BitReader;
#[doc = "Field `SDDC` writer - SDD control"]
pub type SddcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HSTXSRCCL` reader - High-speed transmission slew-rate control on clock lane"]
pub type HstxsrcclR = crate::FieldReader;
#[doc = "Field `HSTXSRCCL` writer - High-speed transmission slew-rate control on clock lane"]
pub type HstxsrcclW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HSTXSRCDL` reader - High-speed transmission slew-rate control on data lanes"]
pub type HstxsrcdlR = crate::FieldReader;
#[doc = "Field `HSTXSRCDL` writer - High-speed transmission slew-rate control on data lanes"]
pub type HstxsrcdlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FLPRXLPM` reader - Forces LP receiver in low-power mode"]
pub type FlprxlpmR = crate::BitReader;
#[doc = "Field `FLPRXLPM` writer - Forces LP receiver in low-power mode"]
pub type FlprxlpmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPRXFT` reader - Low-power RX low-pass filtering tuning"]
pub type LprxftR = crate::FieldReader;
#[doc = "Field `LPRXFT` writer - Low-power RX low-pass filtering tuning"]
pub type LprxftW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - High-speed transmission delay on clock lane"]
    #[inline(always)]
    pub fn hstxdcl(&self) -> HstxdclR {
        HstxdclR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - High-speed transmission delay on data lanes"]
    #[inline(always)]
    pub fn hstxddl(&self) -> HstxddlR {
        HstxddlR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Low-power transmission slew-rate compensation on clock lane"]
    #[inline(always)]
    pub fn lpsrccl(&self) -> LpsrcclR {
        LpsrcclR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - Low-power transmission slew-rate compensation on data lanes"]
    #[inline(always)]
    pub fn lpsrcdl(&self) -> LpsrcdlR {
        LpsrcdlR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bit 12 - SDD control"]
    #[inline(always)]
    pub fn sddc(&self) -> SddcR {
        SddcR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bits 16:17 - High-speed transmission slew-rate control on clock lane"]
    #[inline(always)]
    pub fn hstxsrccl(&self) -> HstxsrcclR {
        HstxsrcclR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:19 - High-speed transmission slew-rate control on data lanes"]
    #[inline(always)]
    pub fn hstxsrcdl(&self) -> HstxsrcdlR {
        HstxsrcdlR::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bit 22 - Forces LP receiver in low-power mode"]
    #[inline(always)]
    pub fn flprxlpm(&self) -> FlprxlpmR {
        FlprxlpmR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 25:26 - Low-power RX low-pass filtering tuning"]
    #[inline(always)]
    pub fn lprxft(&self) -> LprxftR {
        LprxftR::new(((self.bits >> 25) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - High-speed transmission delay on clock lane"]
    #[inline(always)]
    pub fn hstxdcl(&mut self) -> HstxdclW<DsiWpcr1Spec> {
        HstxdclW::new(self, 0)
    }
    #[doc = "Bits 2:3 - High-speed transmission delay on data lanes"]
    #[inline(always)]
    pub fn hstxddl(&mut self) -> HstxddlW<DsiWpcr1Spec> {
        HstxddlW::new(self, 2)
    }
    #[doc = "Bits 6:7 - Low-power transmission slew-rate compensation on clock lane"]
    #[inline(always)]
    pub fn lpsrccl(&mut self) -> LpsrcclW<DsiWpcr1Spec> {
        LpsrcclW::new(self, 6)
    }
    #[doc = "Bits 8:9 - Low-power transmission slew-rate compensation on data lanes"]
    #[inline(always)]
    pub fn lpsrcdl(&mut self) -> LpsrcdlW<DsiWpcr1Spec> {
        LpsrcdlW::new(self, 8)
    }
    #[doc = "Bit 12 - SDD control"]
    #[inline(always)]
    pub fn sddc(&mut self) -> SddcW<DsiWpcr1Spec> {
        SddcW::new(self, 12)
    }
    #[doc = "Bits 16:17 - High-speed transmission slew-rate control on clock lane"]
    #[inline(always)]
    pub fn hstxsrccl(&mut self) -> HstxsrcclW<DsiWpcr1Spec> {
        HstxsrcclW::new(self, 16)
    }
    #[doc = "Bits 18:19 - High-speed transmission slew-rate control on data lanes"]
    #[inline(always)]
    pub fn hstxsrcdl(&mut self) -> HstxsrcdlW<DsiWpcr1Spec> {
        HstxsrcdlW::new(self, 18)
    }
    #[doc = "Bit 22 - Forces LP receiver in low-power mode"]
    #[inline(always)]
    pub fn flprxlpm(&mut self) -> FlprxlpmW<DsiWpcr1Spec> {
        FlprxlpmW::new(self, 22)
    }
    #[doc = "Bits 25:26 - Low-power RX low-pass filtering tuning"]
    #[inline(always)]
    pub fn lprxft(&mut self) -> LprxftW<DsiWpcr1Spec> {
        LprxftW::new(self, 25)
    }
}
#[doc = "This register shall be programmed only when DSI is stopped (CR. DSIEN=0 and CR.EN = 0).\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_wpcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_wpcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiWpcr1Spec;
impl crate::RegisterSpec for DsiWpcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_wpcr1::R`](R) reader structure"]
impl crate::Readable for DsiWpcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`dsi_wpcr1::W`](W) writer structure"]
impl crate::Writable for DsiWpcr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_WPCR1 to value 0"]
impl crate::Resettable for DsiWpcr1Spec {}
