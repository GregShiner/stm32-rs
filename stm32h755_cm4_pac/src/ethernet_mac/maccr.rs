#[doc = "Register `MACCR` reader"]
pub type R = crate::R<MaccrSpec>;
#[doc = "Register `MACCR` writer"]
pub type W = crate::W<MaccrSpec>;
#[doc = "Field `RE` reader - Receiver Enable"]
pub type ReR = crate::BitReader;
#[doc = "Field `RE` writer - Receiver Enable"]
pub type ReW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TE` reader - TE"]
pub type TeR = crate::BitReader;
#[doc = "Field `TE` writer - TE"]
pub type TeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRELEN` reader - PRELEN"]
pub type PrelenR = crate::FieldReader;
#[doc = "Field `PRELEN` writer - PRELEN"]
pub type PrelenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DC` reader - DC"]
pub type DcR = crate::BitReader;
#[doc = "Field `DC` writer - DC"]
pub type DcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BL` reader - BL"]
pub type BlR = crate::FieldReader;
#[doc = "Field `BL` writer - BL"]
pub type BlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DR` reader - DR"]
pub type DrR = crate::BitReader;
#[doc = "Field `DR` writer - DR"]
pub type DrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRS` reader - DCRS"]
pub type DcrsR = crate::BitReader;
#[doc = "Field `DCRS` writer - DCRS"]
pub type DcrsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DO` reader - DO"]
pub type DoR = crate::BitReader;
#[doc = "Field `DO` writer - DO"]
pub type DoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ECRSFD` reader - ECRSFD"]
pub type EcrsfdR = crate::BitReader;
#[doc = "Field `ECRSFD` writer - ECRSFD"]
pub type EcrsfdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LM` reader - LM"]
pub type LmR = crate::BitReader;
#[doc = "Field `LM` writer - LM"]
pub type LmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DM` reader - DM"]
pub type DmR = crate::BitReader;
#[doc = "Field `DM` writer - DM"]
pub type DmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FES` reader - FES"]
pub type FesR = crate::BitReader;
#[doc = "Field `FES` writer - FES"]
pub type FesW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JE` reader - JE"]
pub type JeR = crate::BitReader;
#[doc = "Field `JE` writer - JE"]
pub type JeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `JD` reader - JD"]
pub type JdR = crate::BitReader;
#[doc = "Field `JD` writer - JD"]
pub type JdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WD` reader - WD"]
pub type WdR = crate::BitReader;
#[doc = "Field `WD` writer - WD"]
pub type WdW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACS` reader - ACS"]
pub type AcsR = crate::BitReader;
#[doc = "Field `ACS` writer - ACS"]
pub type AcsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CST` reader - CST"]
pub type CstR = crate::BitReader;
#[doc = "Field `CST` writer - CST"]
pub type CstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `S2KP` reader - S2KP"]
pub type S2kpR = crate::BitReader;
#[doc = "Field `S2KP` writer - S2KP"]
pub type S2kpW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GPSLCE` reader - GPSLCE"]
pub type GpslceR = crate::BitReader;
#[doc = "Field `GPSLCE` writer - GPSLCE"]
pub type GpslceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IPG` reader - IPG"]
pub type IpgR = crate::FieldReader;
#[doc = "Field `IPG` writer - IPG"]
pub type IpgW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `IPC` reader - IPC"]
pub type IpcR = crate::BitReader;
#[doc = "Field `IPC` writer - IPC"]
pub type IpcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SARC` reader - SARC"]
pub type SarcR = crate::FieldReader;
#[doc = "Field `SARC` writer - SARC"]
pub type SarcW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ARPEN` reader - ARPEN"]
pub type ArpenR = crate::BitReader;
#[doc = "Field `ARPEN` writer - ARPEN"]
pub type ArpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&self) -> ReR {
        ReR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TE"]
    #[inline(always)]
    pub fn te(&self) -> TeR {
        TeR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - PRELEN"]
    #[inline(always)]
    pub fn prelen(&self) -> PrelenR {
        PrelenR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 4 - DC"]
    #[inline(always)]
    pub fn dc(&self) -> DcR {
        DcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - BL"]
    #[inline(always)]
    pub fn bl(&self) -> BlR {
        BlR::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 8 - DR"]
    #[inline(always)]
    pub fn dr(&self) -> DrR {
        DrR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DCRS"]
    #[inline(always)]
    pub fn dcrs(&self) -> DcrsR {
        DcrsR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DO"]
    #[inline(always)]
    pub fn do_(&self) -> DoR {
        DoR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - ECRSFD"]
    #[inline(always)]
    pub fn ecrsfd(&self) -> EcrsfdR {
        EcrsfdR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - LM"]
    #[inline(always)]
    pub fn lm(&self) -> LmR {
        LmR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DM"]
    #[inline(always)]
    pub fn dm(&self) -> DmR {
        DmR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - FES"]
    #[inline(always)]
    pub fn fes(&self) -> FesR {
        FesR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - JE"]
    #[inline(always)]
    pub fn je(&self) -> JeR {
        JeR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - JD"]
    #[inline(always)]
    pub fn jd(&self) -> JdR {
        JdR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - WD"]
    #[inline(always)]
    pub fn wd(&self) -> WdR {
        WdR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - ACS"]
    #[inline(always)]
    pub fn acs(&self) -> AcsR {
        AcsR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - CST"]
    #[inline(always)]
    pub fn cst(&self) -> CstR {
        CstR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - S2KP"]
    #[inline(always)]
    pub fn s2kp(&self) -> S2kpR {
        S2kpR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - GPSLCE"]
    #[inline(always)]
    pub fn gpslce(&self) -> GpslceR {
        GpslceR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - IPG"]
    #[inline(always)]
    pub fn ipg(&self) -> IpgR {
        IpgR::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - IPC"]
    #[inline(always)]
    pub fn ipc(&self) -> IpcR {
        IpcR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - SARC"]
    #[inline(always)]
    pub fn sarc(&self) -> SarcR {
        SarcR::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - ARPEN"]
    #[inline(always)]
    pub fn arpen(&self) -> ArpenR {
        ArpenR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Receiver Enable"]
    #[inline(always)]
    pub fn re(&mut self) -> ReW<MaccrSpec> {
        ReW::new(self, 0)
    }
    #[doc = "Bit 1 - TE"]
    #[inline(always)]
    pub fn te(&mut self) -> TeW<MaccrSpec> {
        TeW::new(self, 1)
    }
    #[doc = "Bits 2:3 - PRELEN"]
    #[inline(always)]
    pub fn prelen(&mut self) -> PrelenW<MaccrSpec> {
        PrelenW::new(self, 2)
    }
    #[doc = "Bit 4 - DC"]
    #[inline(always)]
    pub fn dc(&mut self) -> DcW<MaccrSpec> {
        DcW::new(self, 4)
    }
    #[doc = "Bits 5:6 - BL"]
    #[inline(always)]
    pub fn bl(&mut self) -> BlW<MaccrSpec> {
        BlW::new(self, 5)
    }
    #[doc = "Bit 8 - DR"]
    #[inline(always)]
    pub fn dr(&mut self) -> DrW<MaccrSpec> {
        DrW::new(self, 8)
    }
    #[doc = "Bit 9 - DCRS"]
    #[inline(always)]
    pub fn dcrs(&mut self) -> DcrsW<MaccrSpec> {
        DcrsW::new(self, 9)
    }
    #[doc = "Bit 10 - DO"]
    #[inline(always)]
    pub fn do_(&mut self) -> DoW<MaccrSpec> {
        DoW::new(self, 10)
    }
    #[doc = "Bit 11 - ECRSFD"]
    #[inline(always)]
    pub fn ecrsfd(&mut self) -> EcrsfdW<MaccrSpec> {
        EcrsfdW::new(self, 11)
    }
    #[doc = "Bit 12 - LM"]
    #[inline(always)]
    pub fn lm(&mut self) -> LmW<MaccrSpec> {
        LmW::new(self, 12)
    }
    #[doc = "Bit 13 - DM"]
    #[inline(always)]
    pub fn dm(&mut self) -> DmW<MaccrSpec> {
        DmW::new(self, 13)
    }
    #[doc = "Bit 14 - FES"]
    #[inline(always)]
    pub fn fes(&mut self) -> FesW<MaccrSpec> {
        FesW::new(self, 14)
    }
    #[doc = "Bit 16 - JE"]
    #[inline(always)]
    pub fn je(&mut self) -> JeW<MaccrSpec> {
        JeW::new(self, 16)
    }
    #[doc = "Bit 17 - JD"]
    #[inline(always)]
    pub fn jd(&mut self) -> JdW<MaccrSpec> {
        JdW::new(self, 17)
    }
    #[doc = "Bit 19 - WD"]
    #[inline(always)]
    pub fn wd(&mut self) -> WdW<MaccrSpec> {
        WdW::new(self, 19)
    }
    #[doc = "Bit 20 - ACS"]
    #[inline(always)]
    pub fn acs(&mut self) -> AcsW<MaccrSpec> {
        AcsW::new(self, 20)
    }
    #[doc = "Bit 21 - CST"]
    #[inline(always)]
    pub fn cst(&mut self) -> CstW<MaccrSpec> {
        CstW::new(self, 21)
    }
    #[doc = "Bit 22 - S2KP"]
    #[inline(always)]
    pub fn s2kp(&mut self) -> S2kpW<MaccrSpec> {
        S2kpW::new(self, 22)
    }
    #[doc = "Bit 23 - GPSLCE"]
    #[inline(always)]
    pub fn gpslce(&mut self) -> GpslceW<MaccrSpec> {
        GpslceW::new(self, 23)
    }
    #[doc = "Bits 24:26 - IPG"]
    #[inline(always)]
    pub fn ipg(&mut self) -> IpgW<MaccrSpec> {
        IpgW::new(self, 24)
    }
    #[doc = "Bit 27 - IPC"]
    #[inline(always)]
    pub fn ipc(&mut self) -> IpcW<MaccrSpec> {
        IpcW::new(self, 27)
    }
    #[doc = "Bits 28:30 - SARC"]
    #[inline(always)]
    pub fn sarc(&mut self) -> SarcW<MaccrSpec> {
        SarcW::new(self, 28)
    }
    #[doc = "Bit 31 - ARPEN"]
    #[inline(always)]
    pub fn arpen(&mut self) -> ArpenW<MaccrSpec> {
        ArpenW::new(self, 31)
    }
}
#[doc = "Operating mode configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`maccr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maccr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaccrSpec;
impl crate::RegisterSpec for MaccrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maccr::R`](R) reader structure"]
impl crate::Readable for MaccrSpec {}
#[doc = "`write(|w| ..)` method takes [`maccr::W`](W) writer structure"]
impl crate::Writable for MaccrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACCR to value 0"]
impl crate::Resettable for MaccrSpec {}
