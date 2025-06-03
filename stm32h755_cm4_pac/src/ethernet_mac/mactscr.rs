#[doc = "Register `MACTSCR` reader"]
pub type R = crate::R<MactscrSpec>;
#[doc = "Register `MACTSCR` writer"]
pub type W = crate::W<MactscrSpec>;
#[doc = "Field `TSENA` reader - TSENA"]
pub type TsenaR = crate::BitReader;
#[doc = "Field `TSENA` writer - TSENA"]
pub type TsenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCFUPDT` reader - TSCFUPDT"]
pub type TscfupdtR = crate::BitReader;
#[doc = "Field `TSCFUPDT` writer - TSCFUPDT"]
pub type TscfupdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSINIT` reader - TSINIT"]
pub type TsinitR = crate::BitReader;
#[doc = "Field `TSINIT` writer - TSINIT"]
pub type TsinitW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSUPDT` reader - TSUPDT"]
pub type TsupdtR = crate::BitReader;
#[doc = "Field `TSUPDT` writer - TSUPDT"]
pub type TsupdtW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSADDREG` reader - TSADDREG"]
pub type TsaddregR = crate::BitReader;
#[doc = "Field `TSADDREG` writer - TSADDREG"]
pub type TsaddregW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSENALL` reader - TSENALL"]
pub type TsenallR = crate::BitReader;
#[doc = "Field `TSENALL` writer - TSENALL"]
pub type TsenallW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSCTRLSSR` reader - TSCTRLSSR"]
pub type TsctrlssrR = crate::BitReader;
#[doc = "Field `TSCTRLSSR` writer - TSCTRLSSR"]
pub type TsctrlssrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSVER2ENA` reader - TSVER2ENA"]
pub type Tsver2enaR = crate::BitReader;
#[doc = "Field `TSVER2ENA` writer - TSVER2ENA"]
pub type Tsver2enaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPENA` reader - TSIPENA"]
pub type TsipenaR = crate::BitReader;
#[doc = "Field `TSIPENA` writer - TSIPENA"]
pub type TsipenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPV6ENA` reader - TSIPV6ENA"]
pub type Tsipv6enaR = crate::BitReader;
#[doc = "Field `TSIPV6ENA` writer - TSIPV6ENA"]
pub type Tsipv6enaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSIPV4ENA` reader - TSIPV4ENA"]
pub type Tsipv4enaR = crate::BitReader;
#[doc = "Field `TSIPV4ENA` writer - TSIPV4ENA"]
pub type Tsipv4enaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSEVNTENA` reader - TSEVNTENA"]
pub type TsevntenaR = crate::BitReader;
#[doc = "Field `TSEVNTENA` writer - TSEVNTENA"]
pub type TsevntenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSMSTRENA` reader - TSMSTRENA"]
pub type TsmstrenaR = crate::BitReader;
#[doc = "Field `TSMSTRENA` writer - TSMSTRENA"]
pub type TsmstrenaW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SNAPTYPSEL` reader - SNAPTYPSEL"]
pub type SnaptypselR = crate::FieldReader;
#[doc = "Field `SNAPTYPSEL` writer - SNAPTYPSEL"]
pub type SnaptypselW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSENMACADDR` reader - TSENMACADDR"]
pub type TsenmacaddrR = crate::BitReader;
#[doc = "Field `TSENMACADDR` writer - TSENMACADDR"]
pub type TsenmacaddrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSC` reader - CSC"]
pub type CscR = crate::BitReader;
#[doc = "Field `TXTSSTSM` reader - TXTSSTSM"]
pub type TxtsstsmR = crate::BitReader;
#[doc = "Field `TXTSSTSM` writer - TXTSSTSM"]
pub type TxtsstsmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TSENA"]
    #[inline(always)]
    pub fn tsena(&self) -> TsenaR {
        TsenaR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TSCFUPDT"]
    #[inline(always)]
    pub fn tscfupdt(&self) -> TscfupdtR {
        TscfupdtR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - TSINIT"]
    #[inline(always)]
    pub fn tsinit(&self) -> TsinitR {
        TsinitR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - TSUPDT"]
    #[inline(always)]
    pub fn tsupdt(&self) -> TsupdtR {
        TsupdtR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - TSADDREG"]
    #[inline(always)]
    pub fn tsaddreg(&self) -> TsaddregR {
        TsaddregR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - TSENALL"]
    #[inline(always)]
    pub fn tsenall(&self) -> TsenallR {
        TsenallR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - TSCTRLSSR"]
    #[inline(always)]
    pub fn tsctrlssr(&self) -> TsctrlssrR {
        TsctrlssrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - TSVER2ENA"]
    #[inline(always)]
    pub fn tsver2ena(&self) -> Tsver2enaR {
        Tsver2enaR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - TSIPENA"]
    #[inline(always)]
    pub fn tsipena(&self) -> TsipenaR {
        TsipenaR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TSIPV6ENA"]
    #[inline(always)]
    pub fn tsipv6ena(&self) -> Tsipv6enaR {
        Tsipv6enaR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TSIPV4ENA"]
    #[inline(always)]
    pub fn tsipv4ena(&self) -> Tsipv4enaR {
        Tsipv4enaR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TSEVNTENA"]
    #[inline(always)]
    pub fn tsevntena(&self) -> TsevntenaR {
        TsevntenaR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - TSMSTRENA"]
    #[inline(always)]
    pub fn tsmstrena(&self) -> TsmstrenaR {
        TsmstrenaR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - SNAPTYPSEL"]
    #[inline(always)]
    pub fn snaptypsel(&self) -> SnaptypselR {
        SnaptypselR::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - TSENMACADDR"]
    #[inline(always)]
    pub fn tsenmacaddr(&self) -> TsenmacaddrR {
        TsenmacaddrR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - CSC"]
    #[inline(always)]
    pub fn csc(&self) -> CscR {
        CscR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 24 - TXTSSTSM"]
    #[inline(always)]
    pub fn txtsstsm(&self) -> TxtsstsmR {
        TxtsstsmR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TSENA"]
    #[inline(always)]
    pub fn tsena(&mut self) -> TsenaW<MactscrSpec> {
        TsenaW::new(self, 0)
    }
    #[doc = "Bit 1 - TSCFUPDT"]
    #[inline(always)]
    pub fn tscfupdt(&mut self) -> TscfupdtW<MactscrSpec> {
        TscfupdtW::new(self, 1)
    }
    #[doc = "Bit 2 - TSINIT"]
    #[inline(always)]
    pub fn tsinit(&mut self) -> TsinitW<MactscrSpec> {
        TsinitW::new(self, 2)
    }
    #[doc = "Bit 3 - TSUPDT"]
    #[inline(always)]
    pub fn tsupdt(&mut self) -> TsupdtW<MactscrSpec> {
        TsupdtW::new(self, 3)
    }
    #[doc = "Bit 5 - TSADDREG"]
    #[inline(always)]
    pub fn tsaddreg(&mut self) -> TsaddregW<MactscrSpec> {
        TsaddregW::new(self, 5)
    }
    #[doc = "Bit 8 - TSENALL"]
    #[inline(always)]
    pub fn tsenall(&mut self) -> TsenallW<MactscrSpec> {
        TsenallW::new(self, 8)
    }
    #[doc = "Bit 9 - TSCTRLSSR"]
    #[inline(always)]
    pub fn tsctrlssr(&mut self) -> TsctrlssrW<MactscrSpec> {
        TsctrlssrW::new(self, 9)
    }
    #[doc = "Bit 10 - TSVER2ENA"]
    #[inline(always)]
    pub fn tsver2ena(&mut self) -> Tsver2enaW<MactscrSpec> {
        Tsver2enaW::new(self, 10)
    }
    #[doc = "Bit 11 - TSIPENA"]
    #[inline(always)]
    pub fn tsipena(&mut self) -> TsipenaW<MactscrSpec> {
        TsipenaW::new(self, 11)
    }
    #[doc = "Bit 12 - TSIPV6ENA"]
    #[inline(always)]
    pub fn tsipv6ena(&mut self) -> Tsipv6enaW<MactscrSpec> {
        Tsipv6enaW::new(self, 12)
    }
    #[doc = "Bit 13 - TSIPV4ENA"]
    #[inline(always)]
    pub fn tsipv4ena(&mut self) -> Tsipv4enaW<MactscrSpec> {
        Tsipv4enaW::new(self, 13)
    }
    #[doc = "Bit 14 - TSEVNTENA"]
    #[inline(always)]
    pub fn tsevntena(&mut self) -> TsevntenaW<MactscrSpec> {
        TsevntenaW::new(self, 14)
    }
    #[doc = "Bit 15 - TSMSTRENA"]
    #[inline(always)]
    pub fn tsmstrena(&mut self) -> TsmstrenaW<MactscrSpec> {
        TsmstrenaW::new(self, 15)
    }
    #[doc = "Bits 16:17 - SNAPTYPSEL"]
    #[inline(always)]
    pub fn snaptypsel(&mut self) -> SnaptypselW<MactscrSpec> {
        SnaptypselW::new(self, 16)
    }
    #[doc = "Bit 18 - TSENMACADDR"]
    #[inline(always)]
    pub fn tsenmacaddr(&mut self) -> TsenmacaddrW<MactscrSpec> {
        TsenmacaddrW::new(self, 18)
    }
    #[doc = "Bit 24 - TXTSSTSM"]
    #[inline(always)]
    pub fn txtsstsm(&mut self) -> TxtsstsmW<MactscrSpec> {
        TxtsstsmW::new(self, 24)
    }
}
#[doc = "Timestamp control Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mactscr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mactscr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MactscrSpec;
impl crate::RegisterSpec for MactscrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mactscr::R`](R) reader structure"]
impl crate::Readable for MactscrSpec {}
#[doc = "`write(|w| ..)` method takes [`mactscr::W`](W) writer structure"]
impl crate::Writable for MactscrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MACTSCR to value 0x0200"]
impl crate::Resettable for MactscrSpec {
    const RESET_VALUE: u32 = 0x0200;
}
