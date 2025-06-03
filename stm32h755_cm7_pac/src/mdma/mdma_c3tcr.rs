#[doc = "Register `MDMA_C3TCR` reader"]
pub type R = crate::R<MdmaC3tcrSpec>;
#[doc = "Register `MDMA_C3TCR` writer"]
pub type W = crate::W<MdmaC3tcrSpec>;
#[doc = "Field `SINC` reader - Source increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When source is AHB (SBUS=1), SINC = 00 is forbidden. In Linked List Mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\] + 0x00)."]
pub type SincR = crate::FieldReader;
#[doc = "Field `SINC` writer - Source increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When source is AHB (SBUS=1), SINC = 00 is forbidden. In Linked List Mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\] + 0x00)."]
pub type SincW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DINC` reader - Destination increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When destination is AHB (DBUS=1), DINC = 00 is forbidden."]
pub type DincR = crate::FieldReader;
#[doc = "Field `DINC` writer - Destination increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When destination is AHB (DBUS=1), DINC = 00 is forbidden."]
pub type DincW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SSIZE` reader - Source data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If SINCOS &lt; SSIZE and SINC &#8800; 00, the result will be unpredictable. Note: SSIZE = 11 (double-word) is forbidden when source is TCM/AHB bus (SBUS=1)."]
pub type SsizeR = crate::FieldReader;
#[doc = "Field `SSIZE` writer - Source data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If SINCOS &lt; SSIZE and SINC &#8800; 00, the result will be unpredictable. Note: SSIZE = 11 (double-word) is forbidden when source is TCM/AHB bus (SBUS=1)."]
pub type SsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DSIZE` reader - Destination data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0. Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If DINCOS &lt; DSIZE and DINC &#8800; 00, the result will be unpredictable. Note: DSIZE = 11 (double-word) is forbidden when destination is TCM/AHB bus (DBUS=1)."]
pub type DsizeR = crate::FieldReader;
#[doc = "Field `DSIZE` writer - Destination data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0. Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If DINCOS &lt; DSIZE and DINC &#8800; 00, the result will be unpredictable. Note: DSIZE = 11 (double-word) is forbidden when destination is TCM/AHB bus (DBUS=1)."]
pub type DsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SINCOS` reader - source increment offset size"]
pub type SincosR = crate::FieldReader;
#[doc = "Field `SINCOS` writer - source increment offset size"]
pub type SincosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DINCOS` reader - Destination increment offset"]
pub type DincosR = crate::FieldReader;
#[doc = "Field `DINCOS` writer - Destination increment offset"]
pub type DincosW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SBURST` reader - source burst transfer configuration"]
pub type SburstR = crate::FieldReader;
#[doc = "Field `SBURST` writer - source burst transfer configuration"]
pub type SburstW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DBURST` reader - Destination burst transfer configuration"]
pub type DburstR = crate::FieldReader;
#[doc = "Field `DBURST` writer - Destination burst transfer configuration"]
pub type DburstW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TLEN` reader - buffer transfer lengh"]
pub type TlenR = crate::FieldReader;
#[doc = "Field `TLEN` writer - buffer transfer lengh"]
pub type TlenW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `PKE` reader - PacK Enable These bit is set and cleared by software. If the Source Size is smaller than the destination, it will be padded according to the PAM value. If the Source data size is larger than the destination one, it will be truncated. The alignment will be done according to the PAM\\[0\\] value. This bit is protected and can be written only if EN is 0"]
pub type PkeR = crate::BitReader;
#[doc = "Field `PKE` writer - PacK Enable These bit is set and cleared by software. If the Source Size is smaller than the destination, it will be padded according to the PAM value. If the Source data size is larger than the destination one, it will be truncated. The alignment will be done according to the PAM\\[0\\] value. This bit is protected and can be written only if EN is 0"]
pub type PkeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PAM` reader - Padding/Alignement Mode These bits are set and cleared by software. Case 1: Source data size smaller than destination data size - 3 options are valid. Case 2: Source data size larger than destination data size. The remainder part is discarded. When PKE = 1 or DSIZE=SSIZE, these bits are ignored. These bits are protected and can be written only if EN is 0"]
pub type PamR = crate::FieldReader;
#[doc = "Field `PAM` writer - Padding/Alignement Mode These bits are set and cleared by software. Case 1: Source data size smaller than destination data size - 3 options are valid. Case 2: Source data size larger than destination data size. The remainder part is discarded. When PKE = 1 or DSIZE=SSIZE, these bits are ignored. These bits are protected and can be written only if EN is 0"]
pub type PamW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TRGM` reader - Trigger Mode These bits are set and cleared by software. Note: If TRGM is 11 for the current block, all the values loaded at the end of the current block through the linked list mechanism must keep the same value (TRGM=11) and the same SWRM value, otherwise the result is undefined. These bits are protected and can be written only if EN is 0."]
pub type TrgmR = crate::FieldReader;
#[doc = "Field `TRGM` writer - Trigger Mode These bits are set and cleared by software. Note: If TRGM is 11 for the current block, all the values loaded at the end of the current block through the linked list mechanism must keep the same value (TRGM=11) and the same SWRM value, otherwise the result is undefined. These bits are protected and can be written only if EN is 0."]
pub type TrgmW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `SWRM` reader - SW Request Mode This bit is set and cleared by software. If a HW or SW request is currently active, the bit change will be delayed until the current transfer is completed. If the CxMAR contains a valid address, the CxMDR value will also be written @ CxMAR address. This bit is protected and can be written only if EN is 0."]
pub type SwrmR = crate::BitReader;
#[doc = "Field `SWRM` writer - SW Request Mode This bit is set and cleared by software. If a HW or SW request is currently active, the bit change will be delayed until the current transfer is completed. If the CxMAR contains a valid address, the CxMDR value will also be written @ CxMAR address. This bit is protected and can be written only if EN is 0."]
pub type SwrmW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BWM` reader - Bufferable Write Mode This bit is set and cleared by software. This bit is protected and can be written only if EN is 0. Note: All MDMA destination accesses are non-cacheable."]
pub type BwmR = crate::BitReader;
#[doc = "Field `BWM` writer - Bufferable Write Mode This bit is set and cleared by software. This bit is protected and can be written only if EN is 0. Note: All MDMA destination accesses are non-cacheable."]
pub type BwmW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Source increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When source is AHB (SBUS=1), SINC = 00 is forbidden. In Linked List Mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\] + 0x00)."]
    #[inline(always)]
    pub fn sinc(&self) -> SincR {
        SincR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Destination increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When destination is AHB (DBUS=1), DINC = 00 is forbidden."]
    #[inline(always)]
    pub fn dinc(&self) -> DincR {
        DincR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - Source data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If SINCOS &lt; SSIZE and SINC &#8800; 00, the result will be unpredictable. Note: SSIZE = 11 (double-word) is forbidden when source is TCM/AHB bus (SBUS=1)."]
    #[inline(always)]
    pub fn ssize(&self) -> SsizeR {
        SsizeR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - Destination data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0. Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If DINCOS &lt; DSIZE and DINC &#8800; 00, the result will be unpredictable. Note: DSIZE = 11 (double-word) is forbidden when destination is TCM/AHB bus (DBUS=1)."]
    #[inline(always)]
    pub fn dsize(&self) -> DsizeR {
        DsizeR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - source increment offset size"]
    #[inline(always)]
    pub fn sincos(&self) -> SincosR {
        SincosR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Destination increment offset"]
    #[inline(always)]
    pub fn dincos(&self) -> DincosR {
        DincosR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:14 - source burst transfer configuration"]
    #[inline(always)]
    pub fn sburst(&self) -> SburstR {
        SburstR::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - Destination burst transfer configuration"]
    #[inline(always)]
    pub fn dburst(&self) -> DburstR {
        DburstR::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:24 - buffer transfer lengh"]
    #[inline(always)]
    pub fn tlen(&self) -> TlenR {
        TlenR::new(((self.bits >> 18) & 0x7f) as u8)
    }
    #[doc = "Bit 25 - PacK Enable These bit is set and cleared by software. If the Source Size is smaller than the destination, it will be padded according to the PAM value. If the Source data size is larger than the destination one, it will be truncated. The alignment will be done according to the PAM\\[0\\] value. This bit is protected and can be written only if EN is 0"]
    #[inline(always)]
    pub fn pke(&self) -> PkeR {
        PkeR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:27 - Padding/Alignement Mode These bits are set and cleared by software. Case 1: Source data size smaller than destination data size - 3 options are valid. Case 2: Source data size larger than destination data size. The remainder part is discarded. When PKE = 1 or DSIZE=SSIZE, these bits are ignored. These bits are protected and can be written only if EN is 0"]
    #[inline(always)]
    pub fn pam(&self) -> PamR {
        PamR::new(((self.bits >> 26) & 3) as u8)
    }
    #[doc = "Bits 28:29 - Trigger Mode These bits are set and cleared by software. Note: If TRGM is 11 for the current block, all the values loaded at the end of the current block through the linked list mechanism must keep the same value (TRGM=11) and the same SWRM value, otherwise the result is undefined. These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn trgm(&self) -> TrgmR {
        TrgmR::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - SW Request Mode This bit is set and cleared by software. If a HW or SW request is currently active, the bit change will be delayed until the current transfer is completed. If the CxMAR contains a valid address, the CxMDR value will also be written @ CxMAR address. This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn swrm(&self) -> SwrmR {
        SwrmR::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Bufferable Write Mode This bit is set and cleared by software. This bit is protected and can be written only if EN is 0. Note: All MDMA destination accesses are non-cacheable."]
    #[inline(always)]
    pub fn bwm(&self) -> BwmR {
        BwmR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - Source increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When source is AHB (SBUS=1), SINC = 00 is forbidden. In Linked List Mode, at the end of a block (single or last block in repeated block transfer mode), this register will be loaded from memory (from address given by current LAR\\[31:0\\] + 0x00)."]
    #[inline(always)]
    pub fn sinc(&mut self) -> SincW<MdmaC3tcrSpec> {
        SincW::new(self, 0)
    }
    #[doc = "Bits 2:3 - Destination increment mode These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: When destination is AHB (DBUS=1), DINC = 00 is forbidden."]
    #[inline(always)]
    pub fn dinc(&mut self) -> DincW<MdmaC3tcrSpec> {
        DincW::new(self, 2)
    }
    #[doc = "Bits 4:5 - Source data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0 Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If SINCOS &lt; SSIZE and SINC &#8800; 00, the result will be unpredictable. Note: SSIZE = 11 (double-word) is forbidden when source is TCM/AHB bus (SBUS=1)."]
    #[inline(always)]
    pub fn ssize(&mut self) -> SsizeW<MdmaC3tcrSpec> {
        SsizeW::new(self, 4)
    }
    #[doc = "Bits 6:7 - Destination data size These bits are set and cleared by software. These bits are protected and can be written only if EN is 0. Note: If a value of 11 is programmed for the TCM access/AHB port, a transfer error will occur (TEIF bit set) If DINCOS &lt; DSIZE and DINC &#8800; 00, the result will be unpredictable. Note: DSIZE = 11 (double-word) is forbidden when destination is TCM/AHB bus (DBUS=1)."]
    #[inline(always)]
    pub fn dsize(&mut self) -> DsizeW<MdmaC3tcrSpec> {
        DsizeW::new(self, 6)
    }
    #[doc = "Bits 8:9 - source increment offset size"]
    #[inline(always)]
    pub fn sincos(&mut self) -> SincosW<MdmaC3tcrSpec> {
        SincosW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Destination increment offset"]
    #[inline(always)]
    pub fn dincos(&mut self) -> DincosW<MdmaC3tcrSpec> {
        DincosW::new(self, 10)
    }
    #[doc = "Bits 12:14 - source burst transfer configuration"]
    #[inline(always)]
    pub fn sburst(&mut self) -> SburstW<MdmaC3tcrSpec> {
        SburstW::new(self, 12)
    }
    #[doc = "Bits 15:17 - Destination burst transfer configuration"]
    #[inline(always)]
    pub fn dburst(&mut self) -> DburstW<MdmaC3tcrSpec> {
        DburstW::new(self, 15)
    }
    #[doc = "Bits 18:24 - buffer transfer lengh"]
    #[inline(always)]
    pub fn tlen(&mut self) -> TlenW<MdmaC3tcrSpec> {
        TlenW::new(self, 18)
    }
    #[doc = "Bit 25 - PacK Enable These bit is set and cleared by software. If the Source Size is smaller than the destination, it will be padded according to the PAM value. If the Source data size is larger than the destination one, it will be truncated. The alignment will be done according to the PAM\\[0\\] value. This bit is protected and can be written only if EN is 0"]
    #[inline(always)]
    pub fn pke(&mut self) -> PkeW<MdmaC3tcrSpec> {
        PkeW::new(self, 25)
    }
    #[doc = "Bits 26:27 - Padding/Alignement Mode These bits are set and cleared by software. Case 1: Source data size smaller than destination data size - 3 options are valid. Case 2: Source data size larger than destination data size. The remainder part is discarded. When PKE = 1 or DSIZE=SSIZE, these bits are ignored. These bits are protected and can be written only if EN is 0"]
    #[inline(always)]
    pub fn pam(&mut self) -> PamW<MdmaC3tcrSpec> {
        PamW::new(self, 26)
    }
    #[doc = "Bits 28:29 - Trigger Mode These bits are set and cleared by software. Note: If TRGM is 11 for the current block, all the values loaded at the end of the current block through the linked list mechanism must keep the same value (TRGM=11) and the same SWRM value, otherwise the result is undefined. These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn trgm(&mut self) -> TrgmW<MdmaC3tcrSpec> {
        TrgmW::new(self, 28)
    }
    #[doc = "Bit 30 - SW Request Mode This bit is set and cleared by software. If a HW or SW request is currently active, the bit change will be delayed until the current transfer is completed. If the CxMAR contains a valid address, the CxMDR value will also be written @ CxMAR address. This bit is protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn swrm(&mut self) -> SwrmW<MdmaC3tcrSpec> {
        SwrmW::new(self, 30)
    }
    #[doc = "Bit 31 - Bufferable Write Mode This bit is set and cleared by software. This bit is protected and can be written only if EN is 0. Note: All MDMA destination accesses are non-cacheable."]
    #[inline(always)]
    pub fn bwm(&mut self) -> BwmW<MdmaC3tcrSpec> {
        BwmW::new(self, 31)
    }
}
#[doc = "This register is used to configure the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c3tcr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c3tcr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC3tcrSpec;
impl crate::RegisterSpec for MdmaC3tcrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c3tcr::R`](R) reader structure"]
impl crate::Readable for MdmaC3tcrSpec {}
#[doc = "`write(|w| ..)` method takes [`mdma_c3tcr::W`](W) writer structure"]
impl crate::Writable for MdmaC3tcrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C3TCR to value 0"]
impl crate::Resettable for MdmaC3tcrSpec {}
