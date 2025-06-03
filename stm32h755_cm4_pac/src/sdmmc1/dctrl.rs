#[doc = "Register `DCTRL` reader"]
pub type R = crate::R<DctrlSpec>;
#[doc = "Register `DCTRL` writer"]
pub type W = crate::W<DctrlSpec>;
#[doc = "Field `DTEN` reader - Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit shall only be used to transfer data when no associated data transfer command is used, i.e. shall not be used with SD or eMMC cards."]
pub type DtenR = crate::BitReader;
#[doc = "Field `DTEN` writer - Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit shall only be used to transfer data when no associated data transfer command is used, i.e. shall not be used with SD or eMMC cards."]
pub type DtenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTDIR` reader - Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type DtdirR = crate::BitReader;
#[doc = "Field `DTDIR` writer - Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type DtdirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTMODE` reader - Data transfer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type DtmodeR = crate::FieldReader;
#[doc = "Field `DTMODE` writer - Data transfer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type DtmodeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DBLOCKSIZE` reader - Data block size This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). Define the data block length when the block data transfer mode is selected: When DATALENGTH is not a multiple of DBLOCKSIZE, the transfered data is truncated at a multiple of DBLOCKSIZE. (Any remain data will not be transfered.) When DDR = 1, DBLOCKSIZE = 0000 shall not be used. (No data will be transfered)"]
pub type DblocksizeR = crate::FieldReader;
#[doc = "Field `DBLOCKSIZE` writer - Data block size This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). Define the data block length when the block data transfer mode is selected: When DATALENGTH is not a multiple of DBLOCKSIZE, the transfered data is truncated at a multiple of DBLOCKSIZE. (Any remain data will not be transfered.) When DDR = 1, DBLOCKSIZE = 0000 shall not be used. (No data will be transfered)"]
pub type DblocksizeW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `RWSTART` reader - Read wait start. If this bit is set, read wait operation starts."]
pub type RwstartR = crate::BitReader;
#[doc = "Field `RWSTART` writer - Read wait start. If this bit is set, read wait operation starts."]
pub type RwstartW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWSTOP` reader - Read wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the READ_WAIT state to the WAIT_R or IDLE state."]
pub type RwstopR = crate::BitReader;
#[doc = "Field `RWSTOP` writer - Read wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the READ_WAIT state to the WAIT_R or IDLE state."]
pub type RwstopW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RWMOD` reader - Read wait mode. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type RwmodR = crate::BitReader;
#[doc = "Field `RWMOD` writer - Read wait mode. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type RwmodW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOEN` reader - SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation."]
pub type SdioenR = crate::BitReader;
#[doc = "Field `SDIOEN` writer - SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation."]
pub type SdioenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BOOTACKEN` reader - Enable the reception of the boot acknowledgment. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type BootackenR = crate::BitReader;
#[doc = "Field `BOOTACKEN` writer - Enable the reception of the boot acknowledgment. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
pub type BootackenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFORST` reader - FIFO reset, will flush any remaining data. This bit can only be written by firmware when IDMAEN= 0 and DPSM is active (DPSMACT = 1). This bit will only take effect when a transfer error or transfer hold occurs."]
pub type FiforstR = crate::BitReader;
#[doc = "Field `FIFORST` writer - FIFO reset, will flush any remaining data. This bit can only be written by firmware when IDMAEN= 0 and DPSM is active (DPSMACT = 1). This bit will only take effect when a transfer error or transfer hold occurs."]
pub type FiforstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit shall only be used to transfer data when no associated data transfer command is used, i.e. shall not be used with SD or eMMC cards."]
    #[inline(always)]
    pub fn dten(&self) -> DtenR {
        DtenR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn dtdir(&self) -> DtdirR {
        DtdirR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Data transfer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn dtmode(&self) -> DtmodeR {
        DtmodeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Data block size This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). Define the data block length when the block data transfer mode is selected: When DATALENGTH is not a multiple of DBLOCKSIZE, the transfered data is truncated at a multiple of DBLOCKSIZE. (Any remain data will not be transfered.) When DDR = 1, DBLOCKSIZE = 0000 shall not be used. (No data will be transfered)"]
    #[inline(always)]
    pub fn dblocksize(&self) -> DblocksizeR {
        DblocksizeR::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bit 8 - Read wait start. If this bit is set, read wait operation starts."]
    #[inline(always)]
    pub fn rwstart(&self) -> RwstartR {
        RwstartR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Read wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the READ_WAIT state to the WAIT_R or IDLE state."]
    #[inline(always)]
    pub fn rwstop(&self) -> RwstopR {
        RwstopR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Read wait mode. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn rwmod(&self) -> RwmodR {
        RwmodR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation."]
    #[inline(always)]
    pub fn sdioen(&self) -> SdioenR {
        SdioenR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable the reception of the boot acknowledgment. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn bootacken(&self) -> BootackenR {
        BootackenR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - FIFO reset, will flush any remaining data. This bit can only be written by firmware when IDMAEN= 0 and DPSM is active (DPSMACT = 1). This bit will only take effect when a transfer error or transfer hold occurs."]
    #[inline(always)]
    pub fn fiforst(&self) -> FiforstR {
        FiforstR::new(((self.bits >> 13) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Data transfer enable bit This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). This bit is cleared by Hardware when data transfer completes. This bit shall only be used to transfer data when no associated data transfer command is used, i.e. shall not be used with SD or eMMC cards."]
    #[inline(always)]
    pub fn dten(&mut self) -> DtenW<DctrlSpec> {
        DtenW::new(self, 0)
    }
    #[doc = "Bit 1 - Data transfer direction selection This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn dtdir(&mut self) -> DtdirW<DctrlSpec> {
        DtdirW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Data transfer mode selection. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn dtmode(&mut self) -> DtmodeW<DctrlSpec> {
        DtmodeW::new(self, 2)
    }
    #[doc = "Bits 4:7 - Data block size This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). Define the data block length when the block data transfer mode is selected: When DATALENGTH is not a multiple of DBLOCKSIZE, the transfered data is truncated at a multiple of DBLOCKSIZE. (Any remain data will not be transfered.) When DDR = 1, DBLOCKSIZE = 0000 shall not be used. (No data will be transfered)"]
    #[inline(always)]
    pub fn dblocksize(&mut self) -> DblocksizeW<DctrlSpec> {
        DblocksizeW::new(self, 4)
    }
    #[doc = "Bit 8 - Read wait start. If this bit is set, read wait operation starts."]
    #[inline(always)]
    pub fn rwstart(&mut self) -> RwstartW<DctrlSpec> {
        RwstartW::new(self, 8)
    }
    #[doc = "Bit 9 - Read wait stop This bit is written by firmware and auto cleared by hardware when the DPSM moves from the READ_WAIT state to the WAIT_R or IDLE state."]
    #[inline(always)]
    pub fn rwstop(&mut self) -> RwstopW<DctrlSpec> {
        RwstopW::new(self, 9)
    }
    #[doc = "Bit 10 - Read wait mode. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn rwmod(&mut self) -> RwmodW<DctrlSpec> {
        RwmodW::new(self, 10)
    }
    #[doc = "Bit 11 - SD I/O interrupt enable functions This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0). If this bit is set, the DPSM enables the SD I/O card specific interrupt operation."]
    #[inline(always)]
    pub fn sdioen(&mut self) -> SdioenW<DctrlSpec> {
        SdioenW::new(self, 11)
    }
    #[doc = "Bit 12 - Enable the reception of the boot acknowledgment. This bit can only be written by firmware when DPSM is inactive (DPSMACT = 0)."]
    #[inline(always)]
    pub fn bootacken(&mut self) -> BootackenW<DctrlSpec> {
        BootackenW::new(self, 12)
    }
    #[doc = "Bit 13 - FIFO reset, will flush any remaining data. This bit can only be written by firmware when IDMAEN= 0 and DPSM is active (DPSMACT = 1). This bit will only take effect when a transfer error or transfer hold occurs."]
    #[inline(always)]
    pub fn fiforst(&mut self) -> FiforstW<DctrlSpec> {
        FiforstW::new(self, 13)
    }
}
#[doc = "The SDMMC_DCTRL register control the data path state machine (DPSM).\n\nYou can [`read`](crate::Reg::read) this register and get [`dctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DctrlSpec;
impl crate::RegisterSpec for DctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dctrl::R`](R) reader structure"]
impl crate::Readable for DctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`dctrl::W`](W) writer structure"]
impl crate::Writable for DctrlSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCTRL to value 0"]
impl crate::Resettable for DctrlSpec {}
