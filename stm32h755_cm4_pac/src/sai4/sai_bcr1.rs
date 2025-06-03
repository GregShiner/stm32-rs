#[doc = "Register `SAI_BCR1` reader"]
pub type R = crate::R<SaiBcr1Spec>;
#[doc = "Register `SAI_BCR1` writer"]
pub type W = crate::W<SaiBcr1Spec>;
#[doc = "Field `MODE` reader - SAIx audio block mode immediately"]
pub type ModeR = crate::FieldReader;
#[doc = "Field `MODE` writer - SAIx audio block mode immediately"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRTCFG` reader - Protocol configuration. These bits are set and cleared by software. These bits have to be configured when the audio block is disabled."]
pub type PrtcfgR = crate::FieldReader;
#[doc = "Field `PRTCFG` writer - Protocol configuration. These bits are set and cleared by software. These bits have to be configured when the audio block is disabled."]
pub type PrtcfgW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DS` reader - Data size. These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG\\[1:0\\]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP\\[1:0\\] bits, DS\\[1:0\\] are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled."]
pub type DsR = crate::FieldReader;
#[doc = "Field `DS` writer - Data size. These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG\\[1:0\\]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP\\[1:0\\] bits, DS\\[1:0\\] are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled."]
pub type DsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LSBFIRST` reader - Least significant bit first. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC97 audio protocol since AC97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first."]
pub type LsbfirstR = crate::BitReader;
#[doc = "Field `LSBFIRST` writer - Least significant bit first. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC97 audio protocol since AC97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first."]
pub type LsbfirstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CKSTR` reader - Clock strobing edge. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol."]
pub type CkstrR = crate::BitReader;
#[doc = "Field `CKSTR` writer - Clock strobing edge. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol."]
pub type CkstrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCEN` reader - Synchronization enable. These bits are set and cleared by software. They must be configured when the audio sub-block is disabled. Note: The audio sub-block should be configured as asynchronous when SPDIF mode is enabled."]
pub type SyncenR = crate::FieldReader;
#[doc = "Field `SYNCEN` writer - Synchronization enable. These bits are set and cleared by software. They must be configured when the audio sub-block is disabled. Note: The audio sub-block should be configured as asynchronous when SPDIF mode is enabled."]
pub type SyncenW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MONO` reader - Mono mode. This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to Section: Mono/stereo mode for more details."]
pub type MonoR = crate::BitReader;
#[doc = "Field `MONO` writer - Mono mode. This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to Section: Mono/stereo mode for more details."]
pub type MonoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTDRIV` reader - Output drive. This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration."]
pub type OutdrivR = crate::BitReader;
#[doc = "Field `OUTDRIV` writer - Output drive. This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration."]
pub type OutdrivW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAIXEN` reader - Audio block enable where x is A or B. This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command will not be taken into account. This bit allows to control the state of SAIx audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When SAIx block is configured in master mode, the clock must be present on the input of SAIx before setting SAIXEN bit."]
pub type SaixenR = crate::BitReader;
#[doc = "Field `SAIXEN` writer - Audio block enable where x is A or B. This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command will not be taken into account. This bit allows to control the state of SAIx audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When SAIx block is configured in master mode, the clock must be present on the input of SAIx before setting SAIXEN bit."]
pub type SaixenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMAEN` reader - DMA enable. This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE\\[1:0\\] bits must be configured before setting DMAEN to avoid a DMA request in receiver mode."]
pub type DmaenR = crate::BitReader;
#[doc = "Field `DMAEN` writer - DMA enable. This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE\\[1:0\\] bits must be configured before setting DMAEN to avoid a DMA request in receiver mode."]
pub type DmaenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `NOMCK` reader - No divider"]
pub type NomckR = crate::BitReader;
#[doc = "Field `NOMCK` writer - No divider"]
pub type NomckW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MCKDIV` reader - Master clock divider. These bits are set and cleared by software. These bits are meaningless when the audio block operates in slave mode. They have to be configured when the audio block is disabled. Others: the master clock frequency is calculated accordingly to the following formula:"]
pub type MckdivR = crate::FieldReader;
#[doc = "Field `MCKDIV` writer - Master clock divider. These bits are set and cleared by software. These bits are meaningless when the audio block operates in slave mode. They have to be configured when the audio block is disabled. Others: the master clock frequency is calculated accordingly to the following formula:"]
pub type MckdivW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `OSR` reader - Oversampling ratio for master clock"]
pub type OsrR = crate::BitReader;
#[doc = "Field `OSR` writer - Oversampling ratio for master clock"]
pub type OsrW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - SAIx audio block mode immediately"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Protocol configuration. These bits are set and cleared by software. These bits have to be configured when the audio block is disabled."]
    #[inline(always)]
    pub fn prtcfg(&self) -> PrtcfgR {
        PrtcfgR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 5:7 - Data size. These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG\\[1:0\\]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP\\[1:0\\] bits, DS\\[1:0\\] are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled."]
    #[inline(always)]
    pub fn ds(&self) -> DsR {
        DsR::new(((self.bits >> 5) & 7) as u8)
    }
    #[doc = "Bit 8 - Least significant bit first. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC97 audio protocol since AC97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first."]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LsbfirstR {
        LsbfirstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Clock strobing edge. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol."]
    #[inline(always)]
    pub fn ckstr(&self) -> CkstrR {
        CkstrR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:11 - Synchronization enable. These bits are set and cleared by software. They must be configured when the audio sub-block is disabled. Note: The audio sub-block should be configured as asynchronous when SPDIF mode is enabled."]
    #[inline(always)]
    pub fn syncen(&self) -> SyncenR {
        SyncenR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bit 12 - Mono mode. This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to Section: Mono/stereo mode for more details."]
    #[inline(always)]
    pub fn mono(&self) -> MonoR {
        MonoR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Output drive. This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration."]
    #[inline(always)]
    pub fn outdriv(&self) -> OutdrivR {
        OutdrivR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 16 - Audio block enable where x is A or B. This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command will not be taken into account. This bit allows to control the state of SAIx audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When SAIx block is configured in master mode, the clock must be present on the input of SAIx before setting SAIXEN bit."]
    #[inline(always)]
    pub fn saixen(&self) -> SaixenR {
        SaixenR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - DMA enable. This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE\\[1:0\\] bits must be configured before setting DMAEN to avoid a DMA request in receiver mode."]
    #[inline(always)]
    pub fn dmaen(&self) -> DmaenR {
        DmaenR::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 19 - No divider"]
    #[inline(always)]
    pub fn nomck(&self) -> NomckR {
        NomckR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:23 - Master clock divider. These bits are set and cleared by software. These bits are meaningless when the audio block operates in slave mode. They have to be configured when the audio block is disabled. Others: the master clock frequency is calculated accordingly to the following formula:"]
    #[inline(always)]
    pub fn mckdiv(&self) -> MckdivR {
        MckdivR::new(((self.bits >> 20) & 0x0f) as u8)
    }
    #[doc = "Bit 26 - Oversampling ratio for master clock"]
    #[inline(always)]
    pub fn osr(&self) -> OsrR {
        OsrR::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - SAIx audio block mode immediately"]
    #[inline(always)]
    pub fn mode(&mut self) -> ModeW<SaiBcr1Spec> {
        ModeW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Protocol configuration. These bits are set and cleared by software. These bits have to be configured when the audio block is disabled."]
    #[inline(always)]
    pub fn prtcfg(&mut self) -> PrtcfgW<SaiBcr1Spec> {
        PrtcfgW::new(self, 2)
    }
    #[doc = "Bits 5:7 - Data size. These bits are set and cleared by software. These bits are ignored when the SPDIF protocols are selected (bit PRTCFG\\[1:0\\]), because the frame and the data size are fixed in such case. When the companding mode is selected through COMP\\[1:0\\] bits, DS\\[1:0\\] are ignored since the data size is fixed to 8 bits by the algorithm. These bits must be configured when the audio block is disabled."]
    #[inline(always)]
    pub fn ds(&mut self) -> DsW<SaiBcr1Spec> {
        DsW::new(self, 5)
    }
    #[doc = "Bit 8 - Least significant bit first. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in AC97 audio protocol since AC97 data are always transferred with the MSB first. This bit has no meaning in SPDIF audio protocol since in SPDIF data are always transferred with LSB first."]
    #[inline(always)]
    pub fn lsbfirst(&mut self) -> LsbfirstW<SaiBcr1Spec> {
        LsbfirstW::new(self, 8)
    }
    #[doc = "Bit 9 - Clock strobing edge. This bit is set and cleared by software. It must be configured when the audio block is disabled. This bit has no meaning in SPDIF audio protocol."]
    #[inline(always)]
    pub fn ckstr(&mut self) -> CkstrW<SaiBcr1Spec> {
        CkstrW::new(self, 9)
    }
    #[doc = "Bits 10:11 - Synchronization enable. These bits are set and cleared by software. They must be configured when the audio sub-block is disabled. Note: The audio sub-block should be configured as asynchronous when SPDIF mode is enabled."]
    #[inline(always)]
    pub fn syncen(&mut self) -> SyncenW<SaiBcr1Spec> {
        SyncenW::new(self, 10)
    }
    #[doc = "Bit 12 - Mono mode. This bit is set and cleared by software. It is meaningful only when the number of slots is equal to 2. When the mono mode is selected, slot 0 data are duplicated on slot 1 when the audio block operates as a transmitter. In reception mode, the slot1 is discarded and only the data received from slot 0 are stored. Refer to Section: Mono/stereo mode for more details."]
    #[inline(always)]
    pub fn mono(&mut self) -> MonoW<SaiBcr1Spec> {
        MonoW::new(self, 12)
    }
    #[doc = "Bit 13 - Output drive. This bit is set and cleared by software. Note: This bit has to be set before enabling the audio block and after the audio block configuration."]
    #[inline(always)]
    pub fn outdriv(&mut self) -> OutdrivW<SaiBcr1Spec> {
        OutdrivW::new(self, 13)
    }
    #[doc = "Bit 16 - Audio block enable where x is A or B. This bit is set by software. To switch off the audio block, the application software must program this bit to 0 and poll the bit till it reads back 0, meaning that the block is completely disabled. Before setting this bit to 1, check that it is set to 0, otherwise the enable command will not be taken into account. This bit allows to control the state of SAIx audio block. If it is disabled when an audio frame transfer is ongoing, the ongoing transfer completes and the cell is fully disabled at the end of this audio frame transfer. Note: When SAIx block is configured in master mode, the clock must be present on the input of SAIx before setting SAIXEN bit."]
    #[inline(always)]
    pub fn saixen(&mut self) -> SaixenW<SaiBcr1Spec> {
        SaixenW::new(self, 16)
    }
    #[doc = "Bit 17 - DMA enable. This bit is set and cleared by software. Note: Since the audio block defaults to operate as a transmitter after reset, the MODE\\[1:0\\] bits must be configured before setting DMAEN to avoid a DMA request in receiver mode."]
    #[inline(always)]
    pub fn dmaen(&mut self) -> DmaenW<SaiBcr1Spec> {
        DmaenW::new(self, 17)
    }
    #[doc = "Bit 19 - No divider"]
    #[inline(always)]
    pub fn nomck(&mut self) -> NomckW<SaiBcr1Spec> {
        NomckW::new(self, 19)
    }
    #[doc = "Bits 20:23 - Master clock divider. These bits are set and cleared by software. These bits are meaningless when the audio block operates in slave mode. They have to be configured when the audio block is disabled. Others: the master clock frequency is calculated accordingly to the following formula:"]
    #[inline(always)]
    pub fn mckdiv(&mut self) -> MckdivW<SaiBcr1Spec> {
        MckdivW::new(self, 20)
    }
    #[doc = "Bit 26 - Oversampling ratio for master clock"]
    #[inline(always)]
    pub fn osr(&mut self) -> OsrW<SaiBcr1Spec> {
        OsrW::new(self, 26)
    }
}
#[doc = "Configuration register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`sai_bcr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai_bcr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaiBcr1Spec;
impl crate::RegisterSpec for SaiBcr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sai_bcr1::R`](R) reader structure"]
impl crate::Readable for SaiBcr1Spec {}
#[doc = "`write(|w| ..)` method takes [`sai_bcr1::W`](W) writer structure"]
impl crate::Writable for SaiBcr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAI_BCR1 to value 0x40"]
impl crate::Resettable for SaiBcr1Spec {
    const RESET_VALUE: u32 = 0x40;
}
