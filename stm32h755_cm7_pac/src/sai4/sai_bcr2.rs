#[doc = "Register `SAI_BCR2` reader"]
pub type R = crate::R<SaiBcr2Spec>;
#[doc = "Register `SAI_BCR2` writer"]
pub type W = crate::W<SaiBcr2Spec>;
#[doc = "Field `FTH` reader - FIFO threshold. This bit is set and cleared by software."]
pub type FthR = crate::FieldReader;
#[doc = "Field `FTH` writer - FIFO threshold. This bit is set and cleared by software."]
pub type FthW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `FFLUSH` writer - FIFO flush. This bit is set by software. It is always read as 0. This bit should be configured when the SAI is disabled."]
pub type FflushW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRIS` reader - Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to Section: Output data line management on an inactive slot for more details."]
pub type TrisR = crate::BitReader;
#[doc = "Field `TRIS` writer - Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to Section: Output data line management on an inactive slot for more details."]
pub type TrisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUTE` reader - Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to Section: Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
pub type MuteR = crate::BitReader;
#[doc = "Field `MUTE` writer - Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to Section: Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
pub type MuteW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUTEVAL` reader - Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIXEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to Section: Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
pub type MutevalR = crate::BitReader;
#[doc = "Field `MUTEVAL` writer - Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIXEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to Section: Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
pub type MutevalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MUTECNT` reader - Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET will be set and an interrupt will be generated if bit MUTEDETIE is set. Refer to Section: Mute mode for more details."]
pub type MutecntR = crate::FieldReader;
#[doc = "Field `MUTECNT` writer - Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET will be set and an interrupt will be generated if bit MUTEDETIE is set. Refer to Section: Mute mode for more details."]
pub type MutecntW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `CPL` reader - Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is -Law algorithm or A-Law algorithm."]
pub type CplR = crate::BitReader;
#[doc = "Field `CPL` writer - Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is -Law algorithm or A-Law algorithm."]
pub type CplW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `COMP` reader - Companding mode. These bits are set and cleared by software. The -Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that will be used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\\[0\\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to Section: Companding mode for more details. Note: Companding mode is applicable only when TDM is selected."]
pub type CompR = crate::FieldReader;
#[doc = "Field `COMP` writer - Companding mode. These bits are set and cleared by software. The -Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that will be used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\\[0\\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to Section: Companding mode for more details. Note: Companding mode is applicable only when TDM is selected."]
pub type CompW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:2 - FIFO threshold. This bit is set and cleared by software."]
    #[inline(always)]
    pub fn fth(&self) -> FthR {
        FthR::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 4 - Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to Section: Output data line management on an inactive slot for more details."]
    #[inline(always)]
    pub fn tris(&self) -> TrisR {
        TrisR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to Section: Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
    #[inline(always)]
    pub fn mute(&self) -> MuteR {
        MuteR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIXEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to Section: Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
    #[inline(always)]
    pub fn muteval(&self) -> MutevalR {
        MutevalR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 7:12 - Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET will be set and an interrupt will be generated if bit MUTEDETIE is set. Refer to Section: Mute mode for more details."]
    #[inline(always)]
    pub fn mutecnt(&self) -> MutecntR {
        MutecntR::new(((self.bits >> 7) & 0x3f) as u8)
    }
    #[doc = "Bit 13 - Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is -Law algorithm or A-Law algorithm."]
    #[inline(always)]
    pub fn cpl(&self) -> CplR {
        CplR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Companding mode. These bits are set and cleared by software. The -Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that will be used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\\[0\\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to Section: Companding mode for more details. Note: Companding mode is applicable only when TDM is selected."]
    #[inline(always)]
    pub fn comp(&self) -> CompR {
        CompR::new(((self.bits >> 14) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - FIFO threshold. This bit is set and cleared by software."]
    #[inline(always)]
    pub fn fth(&mut self) -> FthW<SaiBcr2Spec> {
        FthW::new(self, 0)
    }
    #[doc = "Bit 3 - FIFO flush. This bit is set by software. It is always read as 0. This bit should be configured when the SAI is disabled."]
    #[inline(always)]
    pub fn fflush(&mut self) -> FflushW<SaiBcr2Spec> {
        FflushW::new(self, 3)
    }
    #[doc = "Bit 4 - Tristate management on data line. This bit is set and cleared by software. It is meaningful only if the audio block is configured as a transmitter. This bit is not used when the audio block is configured in SPDIF mode. It should be configured when SAI is disabled. Refer to Section: Output data line management on an inactive slot for more details."]
    #[inline(always)]
    pub fn tris(&mut self) -> TrisW<SaiBcr2Spec> {
        TrisW::new(self, 4)
    }
    #[doc = "Bit 5 - Mute. This bit is set and cleared by software. It is meaningful only when the audio block operates as a transmitter. The MUTE value is linked to value of MUTEVAL if the number of slots is lower or equal to 2, or equal to 0 if it is greater than 2. Refer to Section: Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
    #[inline(always)]
    pub fn mute(&mut self) -> MuteW<SaiBcr2Spec> {
        MuteW::new(self, 5)
    }
    #[doc = "Bit 6 - Mute value. This bit is set and cleared by software.It must be written before enabling the audio block: SAIXEN. This bit is meaningful only when the audio block operates as a transmitter, the number of slots is lower or equal to 2 and the MUTE bit is set. If more slots are declared, the bit value sent during the transmission in mute mode is equal to 0, whatever the value of MUTEVAL. if the number of slot is lower or equal to 2 and MUTEVAL = 1, the MUTE value transmitted for each slot is the one sent during the previous frame. Refer to Section: Mute mode for more details. Note: This bit is meaningless and should not be used for SPDIF audio blocks."]
    #[inline(always)]
    pub fn muteval(&mut self) -> MutevalW<SaiBcr2Spec> {
        MutevalW::new(self, 6)
    }
    #[doc = "Bits 7:12 - Mute counter. These bits are set and cleared by software. They are used only in reception mode. The value set in these bits is compared to the number of consecutive mute frames detected in reception. When the number of mute frames is equal to this value, the flag MUTEDET will be set and an interrupt will be generated if bit MUTEDETIE is set. Refer to Section: Mute mode for more details."]
    #[inline(always)]
    pub fn mutecnt(&mut self) -> MutecntW<SaiBcr2Spec> {
        MutecntW::new(self, 7)
    }
    #[doc = "Bit 13 - Complement bit. This bit is set and cleared by software. It defines the type of complement to be used for companding mode Note: This bit has effect only when the companding mode is -Law algorithm or A-Law algorithm."]
    #[inline(always)]
    pub fn cpl(&mut self) -> CplW<SaiBcr2Spec> {
        CplW::new(self, 13)
    }
    #[doc = "Bits 14:15 - Companding mode. These bits are set and cleared by software. The -Law and the A-Law log are a part of the CCITT G.711 recommendation, the type of complement that will be used depends on CPL bit. The data expansion or data compression are determined by the state of bit MODE\\[0\\]. The data compression is applied if the audio block is configured as a transmitter. The data expansion is automatically applied when the audio block is configured as a receiver. Refer to Section: Companding mode for more details. Note: Companding mode is applicable only when TDM is selected."]
    #[inline(always)]
    pub fn comp(&mut self) -> CompW<SaiBcr2Spec> {
        CompW::new(self, 14)
    }
}
#[doc = "Configuration register 2\n\nYou can [`read`](crate::Reg::read) this register and get [`sai_bcr2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sai_bcr2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SaiBcr2Spec;
impl crate::RegisterSpec for SaiBcr2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sai_bcr2::R`](R) reader structure"]
impl crate::Readable for SaiBcr2Spec {}
#[doc = "`write(|w| ..)` method takes [`sai_bcr2::W`](W) writer structure"]
impl crate::Writable for SaiBcr2Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAI_BCR2 to value 0"]
impl crate::Resettable for SaiBcr2Spec {}
