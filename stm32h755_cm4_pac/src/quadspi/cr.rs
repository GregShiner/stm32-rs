#[doc = "Register `CR` reader"]
pub type R = crate::R<CrSpec>;
#[doc = "Register `CR` writer"]
pub type W = crate::W<CrSpec>;
#[doc = "Field `EN` reader - Enable Enable the QUADSPI."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Enable Enable the QUADSPI."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ABORT` reader - Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is complete. This bit stops the current transfer. In polling mode or memory-mapped mode, this bit also reset the APM bit or the DM bit."]
pub type AbortR = crate::BitReader;
#[doc = "Field `ABORT` writer - Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is complete. This bit stops the current transfer. In polling mode or memory-mapped mode, this bit also reset the APM bit or the DM bit."]
pub type AbortW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMA enable In indirect mode, DMA can be used to input or output data via the QUADSPI_DR register. DMA transfers are initiated when the FIFO threshold flag, FTF, is set."]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA enable In indirect mode, DMA can be used to input or output data via the QUADSPI_DR register. DMA transfers are initiated when the FIFO threshold flag, FTF, is set."]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCEN` reader - Timeout counter enable This bit is valid only when memory-mapped mode (FMODE = 11) is selected. Activating this bit causes the chip select (nCS) to be released (and thus reduces consumption) if there has not been an access after a certain amount of time, where this time is defined by TIMEOUT\\[15:0\\] (QUADSPI_LPTR). Enable the timeout counter. By default, the QUADSPI never stops its prefetch operation, keeping the previous read operation active with nCS maintained low, even if no access to the Flash memory occurs for a long time. Since Flash memories tend to consume more when nCS is held low, the application might want to activate the timeout counter (TCEN = 1, QUADSPI_CR\\[3\\]) so that nCS is released after a period of TIMEOUT\\[15:0\\] (QUADSPI_LPTR) cycles have elapsed without an access since when the FIFO becomes full with prefetch data. This bit can be modified only when BUSY = 0."]
pub type TcenR = crate::BitReader;
#[doc = "Field `TCEN` writer - Timeout counter enable This bit is valid only when memory-mapped mode (FMODE = 11) is selected. Activating this bit causes the chip select (nCS) to be released (and thus reduces consumption) if there has not been an access after a certain amount of time, where this time is defined by TIMEOUT\\[15:0\\] (QUADSPI_LPTR). Enable the timeout counter. By default, the QUADSPI never stops its prefetch operation, keeping the previous read operation active with nCS maintained low, even if no access to the Flash memory occurs for a long time. Since Flash memories tend to consume more when nCS is held low, the application might want to activate the timeout counter (TCEN = 1, QUADSPI_CR\\[3\\]) so that nCS is released after a period of TIMEOUT\\[15:0\\] (QUADSPI_LPTR) cycles have elapsed without an access since when the FIFO becomes full with prefetch data. This bit can be modified only when BUSY = 0."]
pub type TcenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSHIFT` reader - Sample shift By default, the QUADSPI samples data 1/2 of a CLK cycle after the data is driven by the Flash memory. This bit allows the data is to be sampled later in order to account for external signal delays. Firmware must assure that SSHIFT = 0 when in DDR mode (when DDRM = 1). This field can be modified only when BUSY = 0."]
pub type SshiftR = crate::BitReader;
#[doc = "Field `SSHIFT` writer - Sample shift By default, the QUADSPI samples data 1/2 of a CLK cycle after the data is driven by the Flash memory. This bit allows the data is to be sampled later in order to account for external signal delays. Firmware must assure that SSHIFT = 0 when in DDR mode (when DDRM = 1). This field can be modified only when BUSY = 0."]
pub type SshiftW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DFM` reader - Dual-flash mode This bit activates dual-flash mode, where two external Flash memories are used simultaneously to double throughput and capacity. This bit can be modified only when BUSY = 0."]
pub type DfmR = crate::BitReader;
#[doc = "Field `DFM` writer - Dual-flash mode This bit activates dual-flash mode, where two external Flash memories are used simultaneously to double throughput and capacity. This bit can be modified only when BUSY = 0."]
pub type DfmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FSEL` reader - Flash memory selection This bit selects the Flash memory to be addressed in single flash mode (when DFM = 0). This bit can be modified only when BUSY = 0. This bit is ignored when DFM = 1."]
pub type FselR = crate::BitReader;
#[doc = "Field `FSEL` writer - Flash memory selection This bit selects the Flash memory to be addressed in single flash mode (when DFM = 0). This bit can be modified only when BUSY = 0. This bit is ignored when DFM = 1."]
pub type FselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTHRES` reader - FIFO threshold level Defines, in indirect mode, the threshold number of bytes in the FIFO that will cause the FIFO threshold flag (FTF, QUADSPI_SR\\[2\\]) to be set. In indirect write mode (FMODE = 00): ... In indirect read mode (FMODE = 01): ... If DMAEN = 1, then the DMA controller for the corresponding channel must be disabled before changing the FTHRES value."]
pub type FthresR = crate::FieldReader;
#[doc = "Field `FTHRES` writer - FIFO threshold level Defines, in indirect mode, the threshold number of bytes in the FIFO that will cause the FIFO threshold flag (FTF, QUADSPI_SR\\[2\\]) to be set. In indirect write mode (FMODE = 00): ... In indirect read mode (FMODE = 01): ... If DMAEN = 1, then the DMA controller for the corresponding channel must be disabled before changing the FTHRES value."]
pub type FthresW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `TEIE` reader - Transfer error interrupt enable This bit enables the transfer error interrupt."]
pub type TeieR = crate::BitReader;
#[doc = "Field `TEIE` writer - Transfer error interrupt enable This bit enables the transfer error interrupt."]
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable This bit enables the transfer complete interrupt."]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable This bit enables the transfer complete interrupt."]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FTIE` reader - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt."]
pub type FtieR = crate::BitReader;
#[doc = "Field `FTIE` writer - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt."]
pub type FtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SMIE` reader - Status match interrupt enable This bit enables the status match interrupt."]
pub type SmieR = crate::BitReader;
#[doc = "Field `SMIE` writer - Status match interrupt enable This bit enables the status match interrupt."]
pub type SmieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOIE` reader - TimeOut interrupt enable This bit enables the TimeOut interrupt."]
pub type ToieR = crate::BitReader;
#[doc = "Field `TOIE` writer - TimeOut interrupt enable This bit enables the TimeOut interrupt."]
pub type ToieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `APMS` reader - Automatic poll mode stop This bit determines if automatic polling is stopped after a match. This bit can be modified only when BUSY = 0."]
pub type ApmsR = crate::BitReader;
#[doc = "Field `APMS` writer - Automatic poll mode stop This bit determines if automatic polling is stopped after a match. This bit can be modified only when BUSY = 0."]
pub type ApmsW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PMM` reader - Polling match mode This bit indicates which method should be used for determining a match during automatic polling mode. This bit can be modified only when BUSY = 0."]
pub type PmmR = crate::BitReader;
#[doc = "Field `PMM` writer - Polling match mode This bit indicates which method should be used for determining a match during automatic polling mode. This bit can be modified only when BUSY = 0."]
pub type PmmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PRESCALER` reader - clock prescaler"]
pub type PrescalerR = crate::FieldReader;
#[doc = "Field `PRESCALER` writer - clock prescaler"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Enable Enable the QUADSPI."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is complete. This bit stops the current transfer. In polling mode or memory-mapped mode, this bit also reset the APM bit or the DM bit."]
    #[inline(always)]
    pub fn abort(&self) -> AbortR {
        AbortR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DMA enable In indirect mode, DMA can be used to input or output data via the QUADSPI_DR register. DMA transfers are initiated when the FIFO threshold flag, FTF, is set."]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Timeout counter enable This bit is valid only when memory-mapped mode (FMODE = 11) is selected. Activating this bit causes the chip select (nCS) to be released (and thus reduces consumption) if there has not been an access after a certain amount of time, where this time is defined by TIMEOUT\\[15:0\\] (QUADSPI_LPTR). Enable the timeout counter. By default, the QUADSPI never stops its prefetch operation, keeping the previous read operation active with nCS maintained low, even if no access to the Flash memory occurs for a long time. Since Flash memories tend to consume more when nCS is held low, the application might want to activate the timeout counter (TCEN = 1, QUADSPI_CR\\[3\\]) so that nCS is released after a period of TIMEOUT\\[15:0\\] (QUADSPI_LPTR) cycles have elapsed without an access since when the FIFO becomes full with prefetch data. This bit can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn tcen(&self) -> TcenR {
        TcenR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Sample shift By default, the QUADSPI samples data 1/2 of a CLK cycle after the data is driven by the Flash memory. This bit allows the data is to be sampled later in order to account for external signal delays. Firmware must assure that SSHIFT = 0 when in DDR mode (when DDRM = 1). This field can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn sshift(&self) -> SshiftR {
        SshiftR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - Dual-flash mode This bit activates dual-flash mode, where two external Flash memories are used simultaneously to double throughput and capacity. This bit can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn dfm(&self) -> DfmR {
        DfmR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Flash memory selection This bit selects the Flash memory to be addressed in single flash mode (when DFM = 0). This bit can be modified only when BUSY = 0. This bit is ignored when DFM = 1."]
    #[inline(always)]
    pub fn fsel(&self) -> FselR {
        FselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:12 - FIFO threshold level Defines, in indirect mode, the threshold number of bytes in the FIFO that will cause the FIFO threshold flag (FTF, QUADSPI_SR\\[2\\]) to be set. In indirect write mode (FMODE = 00): ... In indirect read mode (FMODE = 01): ... If DMAEN = 1, then the DMA controller for the corresponding channel must be disabled before changing the FTHRES value."]
    #[inline(always)]
    pub fn fthres(&self) -> FthresR {
        FthresR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bit 16 - Transfer error interrupt enable This bit enables the transfer error interrupt."]
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Transfer complete interrupt enable This bit enables the transfer complete interrupt."]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt."]
    #[inline(always)]
    pub fn ftie(&self) -> FtieR {
        FtieR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Status match interrupt enable This bit enables the status match interrupt."]
    #[inline(always)]
    pub fn smie(&self) -> SmieR {
        SmieR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - TimeOut interrupt enable This bit enables the TimeOut interrupt."]
    #[inline(always)]
    pub fn toie(&self) -> ToieR {
        ToieR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 22 - Automatic poll mode stop This bit determines if automatic polling is stopped after a match. This bit can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn apms(&self) -> ApmsR {
        ApmsR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Polling match mode This bit indicates which method should be used for determining a match during automatic polling mode. This bit can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn pmm(&self) -> PmmR {
        PmmR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:31 - clock prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Enable the QUADSPI."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<CrSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Abort request This bit aborts the on-going command sequence. It is automatically reset once the abort is complete. This bit stops the current transfer. In polling mode or memory-mapped mode, this bit also reset the APM bit or the DM bit."]
    #[inline(always)]
    pub fn abort(&mut self) -> AbortW<CrSpec> {
        AbortW::new(self, 1)
    }
    #[doc = "Bit 2 - DMA enable In indirect mode, DMA can be used to input or output data via the QUADSPI_DR register. DMA transfers are initiated when the FIFO threshold flag, FTF, is set."]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<CrSpec> {
        DmaenW::new(self, 2)
    }
    #[doc = "Bit 3 - Timeout counter enable This bit is valid only when memory-mapped mode (FMODE = 11) is selected. Activating this bit causes the chip select (nCS) to be released (and thus reduces consumption) if there has not been an access after a certain amount of time, where this time is defined by TIMEOUT\\[15:0\\] (QUADSPI_LPTR). Enable the timeout counter. By default, the QUADSPI never stops its prefetch operation, keeping the previous read operation active with nCS maintained low, even if no access to the Flash memory occurs for a long time. Since Flash memories tend to consume more when nCS is held low, the application might want to activate the timeout counter (TCEN = 1, QUADSPI_CR\\[3\\]) so that nCS is released after a period of TIMEOUT\\[15:0\\] (QUADSPI_LPTR) cycles have elapsed without an access since when the FIFO becomes full with prefetch data. This bit can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn tcen(&mut self) -> TcenW<CrSpec> {
        TcenW::new(self, 3)
    }
    #[doc = "Bit 4 - Sample shift By default, the QUADSPI samples data 1/2 of a CLK cycle after the data is driven by the Flash memory. This bit allows the data is to be sampled later in order to account for external signal delays. Firmware must assure that SSHIFT = 0 when in DDR mode (when DDRM = 1). This field can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn sshift(&mut self) -> SshiftW<CrSpec> {
        SshiftW::new(self, 4)
    }
    #[doc = "Bit 6 - Dual-flash mode This bit activates dual-flash mode, where two external Flash memories are used simultaneously to double throughput and capacity. This bit can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn dfm(&mut self) -> DfmW<CrSpec> {
        DfmW::new(self, 6)
    }
    #[doc = "Bit 7 - Flash memory selection This bit selects the Flash memory to be addressed in single flash mode (when DFM = 0). This bit can be modified only when BUSY = 0. This bit is ignored when DFM = 1."]
    #[inline(always)]
    pub fn fsel(&mut self) -> FselW<CrSpec> {
        FselW::new(self, 7)
    }
    #[doc = "Bits 8:12 - FIFO threshold level Defines, in indirect mode, the threshold number of bytes in the FIFO that will cause the FIFO threshold flag (FTF, QUADSPI_SR\\[2\\]) to be set. In indirect write mode (FMODE = 00): ... In indirect read mode (FMODE = 01): ... If DMAEN = 1, then the DMA controller for the corresponding channel must be disabled before changing the FTHRES value."]
    #[inline(always)]
    pub fn fthres(&mut self) -> FthresW<CrSpec> {
        FthresW::new(self, 8)
    }
    #[doc = "Bit 16 - Transfer error interrupt enable This bit enables the transfer error interrupt."]
    #[inline(always)]
    pub fn teie(&mut self) -> TeieW<CrSpec> {
        TeieW::new(self, 16)
    }
    #[doc = "Bit 17 - Transfer complete interrupt enable This bit enables the transfer complete interrupt."]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<CrSpec> {
        TcieW::new(self, 17)
    }
    #[doc = "Bit 18 - FIFO threshold interrupt enable This bit enables the FIFO threshold interrupt."]
    #[inline(always)]
    pub fn ftie(&mut self) -> FtieW<CrSpec> {
        FtieW::new(self, 18)
    }
    #[doc = "Bit 19 - Status match interrupt enable This bit enables the status match interrupt."]
    #[inline(always)]
    pub fn smie(&mut self) -> SmieW<CrSpec> {
        SmieW::new(self, 19)
    }
    #[doc = "Bit 20 - TimeOut interrupt enable This bit enables the TimeOut interrupt."]
    #[inline(always)]
    pub fn toie(&mut self) -> ToieW<CrSpec> {
        ToieW::new(self, 20)
    }
    #[doc = "Bit 22 - Automatic poll mode stop This bit determines if automatic polling is stopped after a match. This bit can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn apms(&mut self) -> ApmsW<CrSpec> {
        ApmsW::new(self, 22)
    }
    #[doc = "Bit 23 - Polling match mode This bit indicates which method should be used for determining a match during automatic polling mode. This bit can be modified only when BUSY = 0."]
    #[inline(always)]
    pub fn pmm(&mut self) -> PmmW<CrSpec> {
        PmmW::new(self, 23)
    }
    #[doc = "Bits 24:31 - clock prescaler"]
    #[inline(always)]
    pub fn prescaler(&mut self) -> PrescalerW<CrSpec> {
        PrescalerW::new(self, 24)
    }
}
#[doc = "QUADSPI control register\n\nYou can [`read`](crate::Reg::read) this register and get [`cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CrSpec;
impl crate::RegisterSpec for CrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr::R`](R) reader structure"]
impl crate::Readable for CrSpec {}
#[doc = "`write(|w| ..)` method takes [`cr::W`](W) writer structure"]
impl crate::Writable for CrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CrSpec {}
