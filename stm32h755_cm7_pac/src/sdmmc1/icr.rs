#[doc = "Register `ICR` reader"]
pub type R = crate::R<IcrSpec>;
#[doc = "Register `ICR` writer"]
pub type W = crate::W<IcrSpec>;
#[doc = "Field `CCRCFAILC` reader - CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag."]
pub type CcrcfailcR = crate::BitReader;
#[doc = "Field `CCRCFAILC` writer - CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag."]
pub type CcrcfailcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRCFAILC` reader - DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag."]
pub type DcrcfailcR = crate::BitReader;
#[doc = "Field `DCRCFAILC` writer - DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag."]
pub type DcrcfailcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIMEOUTC` reader - CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag."]
pub type CtimeoutcR = crate::BitReader;
#[doc = "Field `CTIMEOUTC` writer - CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag."]
pub type CtimeoutcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIMEOUTC` reader - DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag."]
pub type DtimeoutcR = crate::BitReader;
#[doc = "Field `DTIMEOUTC` writer - DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag."]
pub type DtimeoutcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRC` reader - TXUNDERR flag clear bit Set by software to clear TXUNDERR flag."]
pub type TxunderrcR = crate::BitReader;
#[doc = "Field `TXUNDERRC` writer - TXUNDERR flag clear bit Set by software to clear TXUNDERR flag."]
pub type TxunderrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERRC` reader - RXOVERR flag clear bit Set by software to clear the RXOVERR flag."]
pub type RxoverrcR = crate::BitReader;
#[doc = "Field `RXOVERRC` writer - RXOVERR flag clear bit Set by software to clear the RXOVERR flag."]
pub type RxoverrcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDRENDC` reader - CMDREND flag clear bit Set by software to clear the CMDREND flag."]
pub type CmdrendcR = crate::BitReader;
#[doc = "Field `CMDRENDC` writer - CMDREND flag clear bit Set by software to clear the CMDREND flag."]
pub type CmdrendcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSENTC` reader - CMDSENT flag clear bit Set by software to clear the CMDSENT flag."]
pub type CmdsentcR = crate::BitReader;
#[doc = "Field `CMDSENTC` writer - CMDSENT flag clear bit Set by software to clear the CMDSENT flag."]
pub type CmdsentcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAENDC` reader - DATAEND flag clear bit Set by software to clear the DATAEND flag."]
pub type DataendcR = crate::BitReader;
#[doc = "Field `DATAENDC` writer - DATAEND flag clear bit Set by software to clear the DATAEND flag."]
pub type DataendcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DHOLDC` reader - DHOLD flag clear bit Set by software to clear the DHOLD flag."]
pub type DholdcR = crate::BitReader;
#[doc = "Field `DHOLDC` writer - DHOLD flag clear bit Set by software to clear the DHOLD flag."]
pub type DholdcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBCKENDC` reader - DBCKEND flag clear bit Set by software to clear the DBCKEND flag."]
pub type DbckendcR = crate::BitReader;
#[doc = "Field `DBCKENDC` writer - DBCKEND flag clear bit Set by software to clear the DBCKEND flag."]
pub type DbckendcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DABORTC` reader - DABORT flag clear bit Set by software to clear the DABORT flag."]
pub type DabortcR = crate::BitReader;
#[doc = "Field `DABORTC` writer - DABORT flag clear bit Set by software to clear the DABORT flag."]
pub type DabortcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSYD0ENDC` reader - BUSYD0END flag clear bit Set by software to clear the BUSYD0END flag."]
pub type Busyd0endcR = crate::BitReader;
#[doc = "Field `BUSYD0ENDC` writer - BUSYD0END flag clear bit Set by software to clear the BUSYD0END flag."]
pub type Busyd0endcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOITC` reader - SDIOIT flag clear bit Set by software to clear the SDIOIT flag."]
pub type SdioitcR = crate::BitReader;
#[doc = "Field `SDIOITC` writer - SDIOIT flag clear bit Set by software to clear the SDIOIT flag."]
pub type SdioitcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKFAILC` reader - ACKFAIL flag clear bit Set by software to clear the ACKFAIL flag."]
pub type AckfailcR = crate::BitReader;
#[doc = "Field `ACKFAILC` writer - ACKFAIL flag clear bit Set by software to clear the ACKFAIL flag."]
pub type AckfailcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKTIMEOUTC` reader - ACKTIMEOUT flag clear bit Set by software to clear the ACKTIMEOUT flag."]
pub type AcktimeoutcR = crate::BitReader;
#[doc = "Field `ACKTIMEOUTC` writer - ACKTIMEOUT flag clear bit Set by software to clear the ACKTIMEOUT flag."]
pub type AcktimeoutcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSWENDC` reader - VSWEND flag clear bit Set by software to clear the VSWEND flag."]
pub type VswendcR = crate::BitReader;
#[doc = "Field `VSWENDC` writer - VSWEND flag clear bit Set by software to clear the VSWEND flag."]
pub type VswendcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSTOPC` reader - CKSTOP flag clear bit Set by software to clear the CKSTOP flag."]
pub type CkstopcR = crate::BitReader;
#[doc = "Field `CKSTOPC` writer - CKSTOP flag clear bit Set by software to clear the CKSTOP flag."]
pub type CkstopcW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDMATEC` reader - IDMA transfer error clear bit Set by software to clear the IDMATE flag."]
pub type IdmatecR = crate::BitReader;
#[doc = "Field `IDMATEC` writer - IDMA transfer error clear bit Set by software to clear the IDMATE flag."]
pub type IdmatecW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDMABTCC` reader - IDMA buffer transfer complete clear bit Set by software to clear the IDMABTC flag."]
pub type IdmabtccR = crate::BitReader;
#[doc = "Field `IDMABTCC` writer - IDMA buffer transfer complete clear bit Set by software to clear the IDMABTC flag."]
pub type IdmabtccW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag."]
    #[inline(always)]
    pub fn ccrcfailc(&self) -> CcrcfailcR {
        CcrcfailcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag."]
    #[inline(always)]
    pub fn dcrcfailc(&self) -> DcrcfailcR {
        DcrcfailcR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag."]
    #[inline(always)]
    pub fn ctimeoutc(&self) -> CtimeoutcR {
        CtimeoutcR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag."]
    #[inline(always)]
    pub fn dtimeoutc(&self) -> DtimeoutcR {
        DtimeoutcR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - TXUNDERR flag clear bit Set by software to clear TXUNDERR flag."]
    #[inline(always)]
    pub fn txunderrc(&self) -> TxunderrcR {
        TxunderrcR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXOVERR flag clear bit Set by software to clear the RXOVERR flag."]
    #[inline(always)]
    pub fn rxoverrc(&self) -> RxoverrcR {
        RxoverrcR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - CMDREND flag clear bit Set by software to clear the CMDREND flag."]
    #[inline(always)]
    pub fn cmdrendc(&self) -> CmdrendcR {
        CmdrendcR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - CMDSENT flag clear bit Set by software to clear the CMDSENT flag."]
    #[inline(always)]
    pub fn cmdsentc(&self) -> CmdsentcR {
        CmdsentcR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - DATAEND flag clear bit Set by software to clear the DATAEND flag."]
    #[inline(always)]
    pub fn dataendc(&self) -> DataendcR {
        DataendcR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - DHOLD flag clear bit Set by software to clear the DHOLD flag."]
    #[inline(always)]
    pub fn dholdc(&self) -> DholdcR {
        DholdcR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - DBCKEND flag clear bit Set by software to clear the DBCKEND flag."]
    #[inline(always)]
    pub fn dbckendc(&self) -> DbckendcR {
        DbckendcR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - DABORT flag clear bit Set by software to clear the DABORT flag."]
    #[inline(always)]
    pub fn dabortc(&self) -> DabortcR {
        DabortcR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 21 - BUSYD0END flag clear bit Set by software to clear the BUSYD0END flag."]
    #[inline(always)]
    pub fn busyd0endc(&self) -> Busyd0endcR {
        Busyd0endcR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIOIT flag clear bit Set by software to clear the SDIOIT flag."]
    #[inline(always)]
    pub fn sdioitc(&self) -> SdioitcR {
        SdioitcR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - ACKFAIL flag clear bit Set by software to clear the ACKFAIL flag."]
    #[inline(always)]
    pub fn ackfailc(&self) -> AckfailcR {
        AckfailcR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - ACKTIMEOUT flag clear bit Set by software to clear the ACKTIMEOUT flag."]
    #[inline(always)]
    pub fn acktimeoutc(&self) -> AcktimeoutcR {
        AcktimeoutcR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - VSWEND flag clear bit Set by software to clear the VSWEND flag."]
    #[inline(always)]
    pub fn vswendc(&self) -> VswendcR {
        VswendcR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - CKSTOP flag clear bit Set by software to clear the CKSTOP flag."]
    #[inline(always)]
    pub fn ckstopc(&self) -> CkstopcR {
        CkstopcR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - IDMA transfer error clear bit Set by software to clear the IDMATE flag."]
    #[inline(always)]
    pub fn idmatec(&self) -> IdmatecR {
        IdmatecR::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - IDMA buffer transfer complete clear bit Set by software to clear the IDMABTC flag."]
    #[inline(always)]
    pub fn idmabtcc(&self) -> IdmabtccR {
        IdmabtccR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - CCRCFAIL flag clear bit Set by software to clear the CCRCFAIL flag."]
    #[inline(always)]
    pub fn ccrcfailc(&mut self) -> CcrcfailcW<IcrSpec> {
        CcrcfailcW::new(self, 0)
    }
    #[doc = "Bit 1 - DCRCFAIL flag clear bit Set by software to clear the DCRCFAIL flag."]
    #[inline(always)]
    pub fn dcrcfailc(&mut self) -> DcrcfailcW<IcrSpec> {
        DcrcfailcW::new(self, 1)
    }
    #[doc = "Bit 2 - CTIMEOUT flag clear bit Set by software to clear the CTIMEOUT flag."]
    #[inline(always)]
    pub fn ctimeoutc(&mut self) -> CtimeoutcW<IcrSpec> {
        CtimeoutcW::new(self, 2)
    }
    #[doc = "Bit 3 - DTIMEOUT flag clear bit Set by software to clear the DTIMEOUT flag."]
    #[inline(always)]
    pub fn dtimeoutc(&mut self) -> DtimeoutcW<IcrSpec> {
        DtimeoutcW::new(self, 3)
    }
    #[doc = "Bit 4 - TXUNDERR flag clear bit Set by software to clear TXUNDERR flag."]
    #[inline(always)]
    pub fn txunderrc(&mut self) -> TxunderrcW<IcrSpec> {
        TxunderrcW::new(self, 4)
    }
    #[doc = "Bit 5 - RXOVERR flag clear bit Set by software to clear the RXOVERR flag."]
    #[inline(always)]
    pub fn rxoverrc(&mut self) -> RxoverrcW<IcrSpec> {
        RxoverrcW::new(self, 5)
    }
    #[doc = "Bit 6 - CMDREND flag clear bit Set by software to clear the CMDREND flag."]
    #[inline(always)]
    pub fn cmdrendc(&mut self) -> CmdrendcW<IcrSpec> {
        CmdrendcW::new(self, 6)
    }
    #[doc = "Bit 7 - CMDSENT flag clear bit Set by software to clear the CMDSENT flag."]
    #[inline(always)]
    pub fn cmdsentc(&mut self) -> CmdsentcW<IcrSpec> {
        CmdsentcW::new(self, 7)
    }
    #[doc = "Bit 8 - DATAEND flag clear bit Set by software to clear the DATAEND flag."]
    #[inline(always)]
    pub fn dataendc(&mut self) -> DataendcW<IcrSpec> {
        DataendcW::new(self, 8)
    }
    #[doc = "Bit 9 - DHOLD flag clear bit Set by software to clear the DHOLD flag."]
    #[inline(always)]
    pub fn dholdc(&mut self) -> DholdcW<IcrSpec> {
        DholdcW::new(self, 9)
    }
    #[doc = "Bit 10 - DBCKEND flag clear bit Set by software to clear the DBCKEND flag."]
    #[inline(always)]
    pub fn dbckendc(&mut self) -> DbckendcW<IcrSpec> {
        DbckendcW::new(self, 10)
    }
    #[doc = "Bit 11 - DABORT flag clear bit Set by software to clear the DABORT flag."]
    #[inline(always)]
    pub fn dabortc(&mut self) -> DabortcW<IcrSpec> {
        DabortcW::new(self, 11)
    }
    #[doc = "Bit 21 - BUSYD0END flag clear bit Set by software to clear the BUSYD0END flag."]
    #[inline(always)]
    pub fn busyd0endc(&mut self) -> Busyd0endcW<IcrSpec> {
        Busyd0endcW::new(self, 21)
    }
    #[doc = "Bit 22 - SDIOIT flag clear bit Set by software to clear the SDIOIT flag."]
    #[inline(always)]
    pub fn sdioitc(&mut self) -> SdioitcW<IcrSpec> {
        SdioitcW::new(self, 22)
    }
    #[doc = "Bit 23 - ACKFAIL flag clear bit Set by software to clear the ACKFAIL flag."]
    #[inline(always)]
    pub fn ackfailc(&mut self) -> AckfailcW<IcrSpec> {
        AckfailcW::new(self, 23)
    }
    #[doc = "Bit 24 - ACKTIMEOUT flag clear bit Set by software to clear the ACKTIMEOUT flag."]
    #[inline(always)]
    pub fn acktimeoutc(&mut self) -> AcktimeoutcW<IcrSpec> {
        AcktimeoutcW::new(self, 24)
    }
    #[doc = "Bit 25 - VSWEND flag clear bit Set by software to clear the VSWEND flag."]
    #[inline(always)]
    pub fn vswendc(&mut self) -> VswendcW<IcrSpec> {
        VswendcW::new(self, 25)
    }
    #[doc = "Bit 26 - CKSTOP flag clear bit Set by software to clear the CKSTOP flag."]
    #[inline(always)]
    pub fn ckstopc(&mut self) -> CkstopcW<IcrSpec> {
        CkstopcW::new(self, 26)
    }
    #[doc = "Bit 27 - IDMA transfer error clear bit Set by software to clear the IDMATE flag."]
    #[inline(always)]
    pub fn idmatec(&mut self) -> IdmatecW<IcrSpec> {
        IdmatecW::new(self, 27)
    }
    #[doc = "Bit 28 - IDMA buffer transfer complete clear bit Set by software to clear the IDMABTC flag."]
    #[inline(always)]
    pub fn idmabtcc(&mut self) -> IdmabtccW<IcrSpec> {
        IdmabtccW::new(self, 28)
    }
}
#[doc = "The SDMMC_ICR register is a write-only register. Writing a bit with 1 clears the corresponding bit in the SDMMC_STAR status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`icr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IcrSpec;
impl crate::RegisterSpec for IcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icr::R`](R) reader structure"]
impl crate::Readable for IcrSpec {}
#[doc = "`write(|w| ..)` method takes [`icr::W`](W) writer structure"]
impl crate::Writable for IcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICR to value 0"]
impl crate::Resettable for IcrSpec {}
