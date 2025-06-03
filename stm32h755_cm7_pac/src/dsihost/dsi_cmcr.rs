#[doc = "Register `DSI_CMCR` reader"]
pub type R = crate::R<DsiCmcrSpec>;
#[doc = "Register `DSI_CMCR` writer"]
pub type W = crate::W<DsiCmcrSpec>;
#[doc = "Field `TEARE` reader - TEARE"]
pub type TeareR = crate::BitReader;
#[doc = "Field `TEARE` writer - TEARE"]
pub type TeareW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ARE` reader - ARE"]
pub type AreR = crate::BitReader;
#[doc = "Field `ARE` writer - ARE"]
pub type AreW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSW0TX` reader - GSW0TX"]
pub type Gsw0txR = crate::BitReader;
#[doc = "Field `GSW0TX` writer - GSW0TX"]
pub type Gsw0txW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSW1TX` reader - GSW1TX"]
pub type Gsw1txR = crate::BitReader;
#[doc = "Field `GSW1TX` writer - GSW1TX"]
pub type Gsw1txW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSW2TX` reader - GSW2TX"]
pub type Gsw2txR = crate::BitReader;
#[doc = "Field `GSW2TX` writer - GSW2TX"]
pub type Gsw2txW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSR0TX` reader - GSR0TX"]
pub type Gsr0txR = crate::BitReader;
#[doc = "Field `GSR0TX` writer - GSR0TX"]
pub type Gsr0txW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSR1TX` reader - GSR1TX"]
pub type Gsr1txR = crate::BitReader;
#[doc = "Field `GSR1TX` writer - GSR1TX"]
pub type Gsr1txW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GSR2TX` reader - GSR2TX"]
pub type Gsr2txR = crate::BitReader;
#[doc = "Field `GSR2TX` writer - GSR2TX"]
pub type Gsr2txW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLWTX` reader - GLWTX"]
pub type GlwtxR = crate::BitReader;
#[doc = "Field `GLWTX` writer - GLWTX"]
pub type GlwtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSW0TX` reader - DSW0TX"]
pub type Dsw0txR = crate::BitReader;
#[doc = "Field `DSW0TX` writer - DSW0TX"]
pub type Dsw0txW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSW1TX` reader - DSW1TX"]
pub type Dsw1txR = crate::BitReader;
#[doc = "Field `DSW1TX` writer - DSW1TX"]
pub type Dsw1txW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DSR0TX` reader - DSR0TX"]
pub type Dsr0txR = crate::BitReader;
#[doc = "Field `DSR0TX` writer - DSR0TX"]
pub type Dsr0txW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLWTX` reader - DLWTX"]
pub type DlwtxR = crate::BitReader;
#[doc = "Field `DLWTX` writer - DLWTX"]
pub type DlwtxW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MRDPS` reader - MRDPS"]
pub type MrdpsR = crate::BitReader;
#[doc = "Field `MRDPS` writer - MRDPS"]
pub type MrdpsW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TEARE"]
    #[inline(always)]
    pub fn teare(&self) -> TeareR {
        TeareR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ARE"]
    #[inline(always)]
    pub fn are(&self) -> AreR {
        AreR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 8 - GSW0TX"]
    #[inline(always)]
    pub fn gsw0tx(&self) -> Gsw0txR {
        Gsw0txR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - GSW1TX"]
    #[inline(always)]
    pub fn gsw1tx(&self) -> Gsw1txR {
        Gsw1txR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - GSW2TX"]
    #[inline(always)]
    pub fn gsw2tx(&self) -> Gsw2txR {
        Gsw2txR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - GSR0TX"]
    #[inline(always)]
    pub fn gsr0tx(&self) -> Gsr0txR {
        Gsr0txR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - GSR1TX"]
    #[inline(always)]
    pub fn gsr1tx(&self) -> Gsr1txR {
        Gsr1txR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - GSR2TX"]
    #[inline(always)]
    pub fn gsr2tx(&self) -> Gsr2txR {
        Gsr2txR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - GLWTX"]
    #[inline(always)]
    pub fn glwtx(&self) -> GlwtxR {
        GlwtxR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - DSW0TX"]
    #[inline(always)]
    pub fn dsw0tx(&self) -> Dsw0txR {
        Dsw0txR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DSW1TX"]
    #[inline(always)]
    pub fn dsw1tx(&self) -> Dsw1txR {
        Dsw1txR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - DSR0TX"]
    #[inline(always)]
    pub fn dsr0tx(&self) -> Dsr0txR {
        Dsr0txR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - DLWTX"]
    #[inline(always)]
    pub fn dlwtx(&self) -> DlwtxR {
        DlwtxR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - MRDPS"]
    #[inline(always)]
    pub fn mrdps(&self) -> MrdpsR {
        MrdpsR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TEARE"]
    #[inline(always)]
    pub fn teare(&mut self) -> TeareW<DsiCmcrSpec> {
        TeareW::new(self, 0)
    }
    #[doc = "Bit 1 - ARE"]
    #[inline(always)]
    pub fn are(&mut self) -> AreW<DsiCmcrSpec> {
        AreW::new(self, 1)
    }
    #[doc = "Bit 8 - GSW0TX"]
    #[inline(always)]
    pub fn gsw0tx(&mut self) -> Gsw0txW<DsiCmcrSpec> {
        Gsw0txW::new(self, 8)
    }
    #[doc = "Bit 9 - GSW1TX"]
    #[inline(always)]
    pub fn gsw1tx(&mut self) -> Gsw1txW<DsiCmcrSpec> {
        Gsw1txW::new(self, 9)
    }
    #[doc = "Bit 10 - GSW2TX"]
    #[inline(always)]
    pub fn gsw2tx(&mut self) -> Gsw2txW<DsiCmcrSpec> {
        Gsw2txW::new(self, 10)
    }
    #[doc = "Bit 11 - GSR0TX"]
    #[inline(always)]
    pub fn gsr0tx(&mut self) -> Gsr0txW<DsiCmcrSpec> {
        Gsr0txW::new(self, 11)
    }
    #[doc = "Bit 12 - GSR1TX"]
    #[inline(always)]
    pub fn gsr1tx(&mut self) -> Gsr1txW<DsiCmcrSpec> {
        Gsr1txW::new(self, 12)
    }
    #[doc = "Bit 13 - GSR2TX"]
    #[inline(always)]
    pub fn gsr2tx(&mut self) -> Gsr2txW<DsiCmcrSpec> {
        Gsr2txW::new(self, 13)
    }
    #[doc = "Bit 14 - GLWTX"]
    #[inline(always)]
    pub fn glwtx(&mut self) -> GlwtxW<DsiCmcrSpec> {
        GlwtxW::new(self, 14)
    }
    #[doc = "Bit 16 - DSW0TX"]
    #[inline(always)]
    pub fn dsw0tx(&mut self) -> Dsw0txW<DsiCmcrSpec> {
        Dsw0txW::new(self, 16)
    }
    #[doc = "Bit 17 - DSW1TX"]
    #[inline(always)]
    pub fn dsw1tx(&mut self) -> Dsw1txW<DsiCmcrSpec> {
        Dsw1txW::new(self, 17)
    }
    #[doc = "Bit 18 - DSR0TX"]
    #[inline(always)]
    pub fn dsr0tx(&mut self) -> Dsr0txW<DsiCmcrSpec> {
        Dsr0txW::new(self, 18)
    }
    #[doc = "Bit 19 - DLWTX"]
    #[inline(always)]
    pub fn dlwtx(&mut self) -> DlwtxW<DsiCmcrSpec> {
        DlwtxW::new(self, 19)
    }
    #[doc = "Bit 24 - MRDPS"]
    #[inline(always)]
    pub fn mrdps(&mut self) -> MrdpsW<DsiCmcrSpec> {
        MrdpsW::new(self, 24)
    }
}
#[doc = "DSI Host command mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`dsi_cmcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsi_cmcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DsiCmcrSpec;
impl crate::RegisterSpec for DsiCmcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dsi_cmcr::R`](R) reader structure"]
impl crate::Readable for DsiCmcrSpec {}
#[doc = "`write(|w| ..)` method takes [`dsi_cmcr::W`](W) writer structure"]
impl crate::Writable for DsiCmcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DSI_CMCR to value 0"]
impl crate::Resettable for DsiCmcrSpec {}
