#[doc = "Register `SR` reader"]
pub type R = crate::R<SrSpec>;
#[doc = "Field `TEF` reader - Transfer error flag This bit is set in indirect mode when an invalid address is being accessed in indirect mode. It is cleared by writing 1 to CTEF."]
pub type TefR = crate::BitReader;
#[doc = "Field `TCF` reader - Transfer complete flag This bit is set in indirect mode when the programmed number of data has been transferred or in any mode when the transfer has been aborted.It is cleared by writing 1 to CTCF."]
pub type TcfR = crate::BitReader;
#[doc = "Field `FTF` reader - FIFO threshold flag In indirect mode, this bit is set when the FIFO threshold has been reached, or if there is any data left in the FIFO after reads from the Flash memory are complete. It is cleared automatically as soon as threshold condition is no longer true. In automatic polling mode this bit is set every time the status register is read, and the bit is cleared when the data register is read."]
pub type FtfR = crate::BitReader;
#[doc = "Field `SMF` reader - Status match flag This bit is set in automatic polling mode when the unmasked received data matches the corresponding bits in the match register (QUADSPI_PSMAR). It is cleared by writing 1 to CSMF."]
pub type SmfR = crate::BitReader;
#[doc = "Field `TOF` reader - Timeout flag This bit is set when timeout occurs. It is cleared by writing 1 to CTOF."]
pub type TofR = crate::BitReader;
#[doc = "Field `BUSY` reader - Busy This bit is set when an operation is on going. This bit clears automatically when the operation with the Flash memory is finished and the FIFO is empty."]
pub type BusyR = crate::BitReader;
#[doc = "Field `FLEVEL` reader - FIFO level This field gives the number of valid bytes which are being held in the FIFO. FLEVEL = 0 when the FIFO is empty, and 16 when it is full. In memory-mapped mode and in automatic status polling mode, FLEVEL is zero."]
pub type FlevelR = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transfer error flag This bit is set in indirect mode when an invalid address is being accessed in indirect mode. It is cleared by writing 1 to CTEF."]
    #[inline(always)]
    pub fn tef(&self) -> TefR {
        TefR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete flag This bit is set in indirect mode when the programmed number of data has been transferred or in any mode when the transfer has been aborted.It is cleared by writing 1 to CTCF."]
    #[inline(always)]
    pub fn tcf(&self) -> TcfR {
        TcfR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FIFO threshold flag In indirect mode, this bit is set when the FIFO threshold has been reached, or if there is any data left in the FIFO after reads from the Flash memory are complete. It is cleared automatically as soon as threshold condition is no longer true. In automatic polling mode this bit is set every time the status register is read, and the bit is cleared when the data register is read."]
    #[inline(always)]
    pub fn ftf(&self) -> FtfR {
        FtfR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Status match flag This bit is set in automatic polling mode when the unmasked received data matches the corresponding bits in the match register (QUADSPI_PSMAR). It is cleared by writing 1 to CSMF."]
    #[inline(always)]
    pub fn smf(&self) -> SmfR {
        SmfR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Timeout flag This bit is set when timeout occurs. It is cleared by writing 1 to CTOF."]
    #[inline(always)]
    pub fn tof(&self) -> TofR {
        TofR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Busy This bit is set when an operation is on going. This bit clears automatically when the operation with the Flash memory is finished and the FIFO is empty."]
    #[inline(always)]
    pub fn busy(&self) -> BusyR {
        BusyR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 8:13 - FIFO level This field gives the number of valid bytes which are being held in the FIFO. FLEVEL = 0 when the FIFO is empty, and 16 when it is full. In memory-mapped mode and in automatic status polling mode, FLEVEL is zero."]
    #[inline(always)]
    pub fn flevel(&self) -> FlevelR {
        FlevelR::new(((self.bits >> 8) & 0x3f) as u8)
    }
}
#[doc = "QUADSPI status register\n\nYou can [`read`](crate::Reg::read) this register and get [`sr::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SrSpec;
impl crate::RegisterSpec for SrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sr::R`](R) reader structure"]
impl crate::Readable for SrSpec {}
#[doc = "`reset()` method sets SR to value 0"]
impl crate::Resettable for SrSpec {}
