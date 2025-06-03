#[doc = "Register `MASKR` reader"]
pub type R = crate::R<MaskrSpec>;
#[doc = "Register `MASKR` writer"]
pub type W = crate::W<MaskrSpec>;
#[doc = "Field `CCRCFAILIE` reader - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
pub type CcrcfailieR = crate::BitReader;
#[doc = "Field `CCRCFAILIE` writer - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
pub type CcrcfailieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DCRCFAILIE` reader - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
pub type DcrcfailieR = crate::BitReader;
#[doc = "Field `DCRCFAILIE` writer - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
pub type DcrcfailieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTIMEOUTIE` reader - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
pub type CtimeoutieR = crate::BitReader;
#[doc = "Field `CTIMEOUTIE` writer - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
pub type CtimeoutieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DTIMEOUTIE` reader - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
pub type DtimeoutieR = crate::BitReader;
#[doc = "Field `DTIMEOUTIE` writer - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
pub type DtimeoutieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUNDERRIE` reader - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
pub type TxunderrieR = crate::BitReader;
#[doc = "Field `TXUNDERRIE` writer - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
pub type TxunderrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVERRIE` reader - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
pub type RxoverrieR = crate::BitReader;
#[doc = "Field `RXOVERRIE` writer - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
pub type RxoverrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDRENDIE` reader - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
pub type CmdrendieR = crate::BitReader;
#[doc = "Field `CMDRENDIE` writer - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
pub type CmdrendieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CMDSENTIE` reader - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
pub type CmdsentieR = crate::BitReader;
#[doc = "Field `CMDSENTIE` writer - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
pub type CmdsentieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DATAENDIE` reader - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
pub type DataendieR = crate::BitReader;
#[doc = "Field `DATAENDIE` writer - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
pub type DataendieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DHOLDIE` reader - Data hold interrupt enable Set and cleared by software to enable/disable the interrupt generated when sending new data is hold in the DPSM Wait_S state."]
pub type DholdieR = crate::BitReader;
#[doc = "Field `DHOLDIE` writer - Data hold interrupt enable Set and cleared by software to enable/disable the interrupt generated when sending new data is hold in the DPSM Wait_S state."]
pub type DholdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DBCKENDIE` reader - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
pub type DbckendieR = crate::BitReader;
#[doc = "Field `DBCKENDIE` writer - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
pub type DbckendieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DABORTIE` reader - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
pub type DabortieR = crate::BitReader;
#[doc = "Field `DABORTIE` writer - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
pub type DabortieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOHEIE` reader - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
pub type TxfifoheieR = crate::BitReader;
#[doc = "Field `TXFIFOHEIE` writer - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
pub type TxfifoheieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOHFIE` reader - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
pub type RxfifohfieR = crate::BitReader;
#[doc = "Field `RXFIFOHFIE` writer - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
pub type RxfifohfieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXFIFOFIE` reader - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
pub type RxfifofieR = crate::BitReader;
#[doc = "Field `RXFIFOFIE` writer - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
pub type RxfifofieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXFIFOEIE` reader - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
pub type TxfifoeieR = crate::BitReader;
#[doc = "Field `TXFIFOEIE` writer - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
pub type TxfifoeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUSYD0ENDIE` reader - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
pub type Busyd0endieR = crate::BitReader;
#[doc = "Field `BUSYD0ENDIE` writer - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
pub type Busyd0endieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIOITIE` reader - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
pub type SdioitieR = crate::BitReader;
#[doc = "Field `SDIOITIE` writer - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
pub type SdioitieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKFAILIE` reader - Acknowledgment Fail interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment Fail."]
pub type AckfailieR = crate::BitReader;
#[doc = "Field `ACKFAILIE` writer - Acknowledgment Fail interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment Fail."]
pub type AckfailieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACKTIMEOUTIE` reader - Acknowledgment timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment timeout."]
pub type AcktimeoutieR = crate::BitReader;
#[doc = "Field `ACKTIMEOUTIE` writer - Acknowledgment timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment timeout."]
pub type AcktimeoutieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `VSWENDIE` reader - Voltage switch critical timing section completion interrupt enable Set and cleared by software to enable/disable the interrupt generated when voltage switch critical timing section completion."]
pub type VswendieR = crate::BitReader;
#[doc = "Field `VSWENDIE` writer - Voltage switch critical timing section completion interrupt enable Set and cleared by software to enable/disable the interrupt generated when voltage switch critical timing section completion."]
pub type VswendieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSTOPIE` reader - Voltage Switch clock stopped interrupt enable Set and cleared by software to enable/disable interrupt caused by Voltage Switch clock stopped."]
pub type CkstopieR = crate::BitReader;
#[doc = "Field `CKSTOPIE` writer - Voltage Switch clock stopped interrupt enable Set and cleared by software to enable/disable interrupt caused by Voltage Switch clock stopped."]
pub type CkstopieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IDMABTCIE` reader - IDMA buffer transfer complete interrupt enable Set and cleared by software to enable/disable the interrupt generated when the IDMA has transferred all data belonging to a memory buffer."]
pub type IdmabtcieR = crate::BitReader;
#[doc = "Field `IDMABTCIE` writer - IDMA buffer transfer complete interrupt enable Set and cleared by software to enable/disable the interrupt generated when the IDMA has transferred all data belonging to a memory buffer."]
pub type IdmabtcieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
    #[inline(always)]
    pub fn ccrcfailie(&self) -> CcrcfailieR {
        CcrcfailieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
    #[inline(always)]
    pub fn dcrcfailie(&self) -> DcrcfailieR {
        DcrcfailieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
    #[inline(always)]
    pub fn ctimeoutie(&self) -> CtimeoutieR {
        CtimeoutieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
    #[inline(always)]
    pub fn dtimeoutie(&self) -> DtimeoutieR {
        DtimeoutieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
    #[inline(always)]
    pub fn txunderrie(&self) -> TxunderrieR {
        TxunderrieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
    #[inline(always)]
    pub fn rxoverrie(&self) -> RxoverrieR {
        RxoverrieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
    #[inline(always)]
    pub fn cmdrendie(&self) -> CmdrendieR {
        CmdrendieR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
    #[inline(always)]
    pub fn cmdsentie(&self) -> CmdsentieR {
        CmdsentieR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
    #[inline(always)]
    pub fn dataendie(&self) -> DataendieR {
        DataendieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Data hold interrupt enable Set and cleared by software to enable/disable the interrupt generated when sending new data is hold in the DPSM Wait_S state."]
    #[inline(always)]
    pub fn dholdie(&self) -> DholdieR {
        DholdieR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
    #[inline(always)]
    pub fn dbckendie(&self) -> DbckendieR {
        DbckendieR::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
    #[inline(always)]
    pub fn dabortie(&self) -> DabortieR {
        DabortieR::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 14 - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
    #[inline(always)]
    pub fn txfifoheie(&self) -> TxfifoheieR {
        TxfifoheieR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
    #[inline(always)]
    pub fn rxfifohfie(&self) -> RxfifohfieR {
        RxfifohfieR::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 17 - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
    #[inline(always)]
    pub fn rxfifofie(&self) -> RxfifofieR {
        RxfifofieR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
    #[inline(always)]
    pub fn txfifoeie(&self) -> TxfifoeieR {
        TxfifoeieR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 21 - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
    #[inline(always)]
    pub fn busyd0endie(&self) -> Busyd0endieR {
        Busyd0endieR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
    #[inline(always)]
    pub fn sdioitie(&self) -> SdioitieR {
        SdioitieR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Acknowledgment Fail interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment Fail."]
    #[inline(always)]
    pub fn ackfailie(&self) -> AckfailieR {
        AckfailieR::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Acknowledgment timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment timeout."]
    #[inline(always)]
    pub fn acktimeoutie(&self) -> AcktimeoutieR {
        AcktimeoutieR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Voltage switch critical timing section completion interrupt enable Set and cleared by software to enable/disable the interrupt generated when voltage switch critical timing section completion."]
    #[inline(always)]
    pub fn vswendie(&self) -> VswendieR {
        VswendieR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Voltage Switch clock stopped interrupt enable Set and cleared by software to enable/disable interrupt caused by Voltage Switch clock stopped."]
    #[inline(always)]
    pub fn ckstopie(&self) -> CkstopieR {
        CkstopieR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 28 - IDMA buffer transfer complete interrupt enable Set and cleared by software to enable/disable the interrupt generated when the IDMA has transferred all data belonging to a memory buffer."]
    #[inline(always)]
    pub fn idmabtcie(&self) -> IdmabtcieR {
        IdmabtcieR::new(((self.bits >> 28) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Command CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by command CRC failure."]
    #[inline(always)]
    pub fn ccrcfailie(&mut self) -> CcrcfailieW<MaskrSpec> {
        CcrcfailieW::new(self, 0)
    }
    #[doc = "Bit 1 - Data CRC fail interrupt enable Set and cleared by software to enable/disable interrupt caused by data CRC failure."]
    #[inline(always)]
    pub fn dcrcfailie(&mut self) -> DcrcfailieW<MaskrSpec> {
        DcrcfailieW::new(self, 1)
    }
    #[doc = "Bit 2 - Command timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by command timeout."]
    #[inline(always)]
    pub fn ctimeoutie(&mut self) -> CtimeoutieW<MaskrSpec> {
        CtimeoutieW::new(self, 2)
    }
    #[doc = "Bit 3 - Data timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by data timeout."]
    #[inline(always)]
    pub fn dtimeoutie(&mut self) -> DtimeoutieW<MaskrSpec> {
        DtimeoutieW::new(self, 3)
    }
    #[doc = "Bit 4 - Tx FIFO underrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO underrun error."]
    #[inline(always)]
    pub fn txunderrie(&mut self) -> TxunderrieW<MaskrSpec> {
        TxunderrieW::new(self, 4)
    }
    #[doc = "Bit 5 - Rx FIFO overrun error interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO overrun error."]
    #[inline(always)]
    pub fn rxoverrie(&mut self) -> RxoverrieW<MaskrSpec> {
        RxoverrieW::new(self, 5)
    }
    #[doc = "Bit 6 - Command response received interrupt enable Set and cleared by software to enable/disable interrupt caused by receiving command response."]
    #[inline(always)]
    pub fn cmdrendie(&mut self) -> CmdrendieW<MaskrSpec> {
        CmdrendieW::new(self, 6)
    }
    #[doc = "Bit 7 - Command sent interrupt enable Set and cleared by software to enable/disable interrupt caused by sending command."]
    #[inline(always)]
    pub fn cmdsentie(&mut self) -> CmdsentieW<MaskrSpec> {
        CmdsentieW::new(self, 7)
    }
    #[doc = "Bit 8 - Data end interrupt enable Set and cleared by software to enable/disable interrupt caused by data end."]
    #[inline(always)]
    pub fn dataendie(&mut self) -> DataendieW<MaskrSpec> {
        DataendieW::new(self, 8)
    }
    #[doc = "Bit 9 - Data hold interrupt enable Set and cleared by software to enable/disable the interrupt generated when sending new data is hold in the DPSM Wait_S state."]
    #[inline(always)]
    pub fn dholdie(&mut self) -> DholdieW<MaskrSpec> {
        DholdieW::new(self, 9)
    }
    #[doc = "Bit 10 - Data block end interrupt enable Set and cleared by software to enable/disable interrupt caused by data block end."]
    #[inline(always)]
    pub fn dbckendie(&mut self) -> DbckendieW<MaskrSpec> {
        DbckendieW::new(self, 10)
    }
    #[doc = "Bit 11 - Data transfer aborted interrupt enable Set and cleared by software to enable/disable interrupt caused by a data transfer being aborted."]
    #[inline(always)]
    pub fn dabortie(&mut self) -> DabortieW<MaskrSpec> {
        DabortieW::new(self, 11)
    }
    #[doc = "Bit 14 - Tx FIFO half empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO half empty."]
    #[inline(always)]
    pub fn txfifoheie(&mut self) -> TxfifoheieW<MaskrSpec> {
        TxfifoheieW::new(self, 14)
    }
    #[doc = "Bit 15 - Rx FIFO half full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO half full."]
    #[inline(always)]
    pub fn rxfifohfie(&mut self) -> RxfifohfieW<MaskrSpec> {
        RxfifohfieW::new(self, 15)
    }
    #[doc = "Bit 17 - Rx FIFO full interrupt enable Set and cleared by software to enable/disable interrupt caused by Rx FIFO full."]
    #[inline(always)]
    pub fn rxfifofie(&mut self) -> RxfifofieW<MaskrSpec> {
        RxfifofieW::new(self, 17)
    }
    #[doc = "Bit 18 - Tx FIFO empty interrupt enable Set and cleared by software to enable/disable interrupt caused by Tx FIFO empty."]
    #[inline(always)]
    pub fn txfifoeie(&mut self) -> TxfifoeieW<MaskrSpec> {
        TxfifoeieW::new(self, 18)
    }
    #[doc = "Bit 21 - BUSYD0END interrupt enable Set and cleared by software to enable/disable the interrupt generated when SDMMC_D0 signal changes from busy to NOT busy following a CMD response."]
    #[inline(always)]
    pub fn busyd0endie(&mut self) -> Busyd0endieW<MaskrSpec> {
        Busyd0endieW::new(self, 21)
    }
    #[doc = "Bit 22 - SDIO mode interrupt received interrupt enable Set and cleared by software to enable/disable the interrupt generated when receiving the SDIO mode interrupt."]
    #[inline(always)]
    pub fn sdioitie(&mut self) -> SdioitieW<MaskrSpec> {
        SdioitieW::new(self, 22)
    }
    #[doc = "Bit 23 - Acknowledgment Fail interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment Fail."]
    #[inline(always)]
    pub fn ackfailie(&mut self) -> AckfailieW<MaskrSpec> {
        AckfailieW::new(self, 23)
    }
    #[doc = "Bit 24 - Acknowledgment timeout interrupt enable Set and cleared by software to enable/disable interrupt caused by acknowledgment timeout."]
    #[inline(always)]
    pub fn acktimeoutie(&mut self) -> AcktimeoutieW<MaskrSpec> {
        AcktimeoutieW::new(self, 24)
    }
    #[doc = "Bit 25 - Voltage switch critical timing section completion interrupt enable Set and cleared by software to enable/disable the interrupt generated when voltage switch critical timing section completion."]
    #[inline(always)]
    pub fn vswendie(&mut self) -> VswendieW<MaskrSpec> {
        VswendieW::new(self, 25)
    }
    #[doc = "Bit 26 - Voltage Switch clock stopped interrupt enable Set and cleared by software to enable/disable interrupt caused by Voltage Switch clock stopped."]
    #[inline(always)]
    pub fn ckstopie(&mut self) -> CkstopieW<MaskrSpec> {
        CkstopieW::new(self, 26)
    }
    #[doc = "Bit 28 - IDMA buffer transfer complete interrupt enable Set and cleared by software to enable/disable the interrupt generated when the IDMA has transferred all data belonging to a memory buffer."]
    #[inline(always)]
    pub fn idmabtcie(&mut self) -> IdmabtcieW<MaskrSpec> {
        IdmabtcieW::new(self, 28)
    }
}
#[doc = "The interrupt mask register determines which status flags generate an interrupt request by setting the corresponding bit to 1.\n\nYou can [`read`](crate::Reg::read) this register and get [`maskr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`maskr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MaskrSpec;
impl crate::RegisterSpec for MaskrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`maskr::R`](R) reader structure"]
impl crate::Readable for MaskrSpec {}
#[doc = "`write(|w| ..)` method takes [`maskr::W`](W) writer structure"]
impl crate::Writable for MaskrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MASKR to value 0"]
impl crate::Resettable for MaskrSpec {}
