#[doc = "Register `MDMA_C2CR` reader"]
pub type R = crate::R<MdmaC2crSpec>;
#[doc = "Register `MDMA_C2CR` writer"]
pub type W = crate::W<MdmaC2crSpec>;
#[doc = "Field `EN` reader - channel enable"]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - channel enable"]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIE` reader - Transfer error interrupt enable This bit is set and cleared by software."]
pub type TeieR = crate::BitReader;
#[doc = "Field `TEIE` writer - Transfer error interrupt enable This bit is set and cleared by software."]
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CTCIE` reader - Channel Transfer Complete interrupt enable This bit is set and cleared by software."]
pub type CtcieR = crate::BitReader;
#[doc = "Field `CTCIE` writer - Channel Transfer Complete interrupt enable This bit is set and cleared by software."]
pub type CtcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRTIE` reader - Block Repeat transfer interrupt enable This bit is set and cleared by software."]
pub type BrtieR = crate::BitReader;
#[doc = "Field `BRTIE` writer - Block Repeat transfer interrupt enable This bit is set and cleared by software."]
pub type BrtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BTIE` reader - Block Transfer interrupt enable This bit is set and cleared by software."]
pub type BtieR = crate::BitReader;
#[doc = "Field `BTIE` writer - Block Transfer interrupt enable This bit is set and cleared by software."]
pub type BtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - buffer Transfer Complete interrupt enable This bit is set and cleared by software."]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - buffer Transfer Complete interrupt enable This bit is set and cleared by software."]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PL` reader - Priority level These bits are set and cleared by software. These bits are protected and can be written only if EN is 0."]
pub type PlR = crate::FieldReader;
#[doc = "Field `PL` writer - Priority level These bits are set and cleared by software. These bits are protected and can be written only if EN is 0."]
pub type PlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `BEX` reader - byte Endianness exchange"]
pub type BexR = crate::BitReader;
#[doc = "Field `BEX` writer - byte Endianness exchange"]
pub type BexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HEX` reader - Half word Endianes exchange"]
pub type HexR = crate::BitReader;
#[doc = "Field `HEX` writer - Half word Endianes exchange"]
pub type HexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `WEX` reader - Word Endianness exchange"]
pub type WexR = crate::BitReader;
#[doc = "Field `WEX` writer - Word Endianness exchange"]
pub type WexW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWRQ` writer - SW ReQuest Writing a 1 into this bit sets the CRQAx in MDMA_ISRy register, activating the request on Channel x Note: Either the whole CxCR register or the 8-bit/16-bit register @ Address offset: 0x4E + 0x40 chn may be used for SWRQ activation. In case of a SW request, acknowledge is not generated (neither HW signal, nor CxMAR write access)."]
pub type SwrqW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - channel enable"]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel Transfer Complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ctcie(&self) -> CtcieR {
        CtcieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Block Repeat transfer interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn brtie(&self) -> BrtieR {
        BrtieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Block Transfer interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn btie(&self) -> BtieR {
        BtieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - buffer Transfer Complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 6:7 - Priority level These bits are set and cleared by software. These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn pl(&self) -> PlR {
        PlR::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 12 - byte Endianness exchange"]
    #[inline(always)]
    pub fn bex(&self) -> BexR {
        BexR::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Half word Endianes exchange"]
    #[inline(always)]
    pub fn hex(&self) -> HexR {
        HexR::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Word Endianness exchange"]
    #[inline(always)]
    pub fn wex(&self) -> WexR {
        WexR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - channel enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<MdmaC2crSpec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn teie(&mut self) -> TeieW<MdmaC2crSpec> {
        TeieW::new(self, 1)
    }
    #[doc = "Bit 2 - Channel Transfer Complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn ctcie(&mut self) -> CtcieW<MdmaC2crSpec> {
        CtcieW::new(self, 2)
    }
    #[doc = "Bit 3 - Block Repeat transfer interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn brtie(&mut self) -> BrtieW<MdmaC2crSpec> {
        BrtieW::new(self, 3)
    }
    #[doc = "Bit 4 - Block Transfer interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn btie(&mut self) -> BtieW<MdmaC2crSpec> {
        BtieW::new(self, 4)
    }
    #[doc = "Bit 5 - buffer Transfer Complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<MdmaC2crSpec> {
        TcieW::new(self, 5)
    }
    #[doc = "Bits 6:7 - Priority level These bits are set and cleared by software. These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn pl(&mut self) -> PlW<MdmaC2crSpec> {
        PlW::new(self, 6)
    }
    #[doc = "Bit 12 - byte Endianness exchange"]
    #[inline(always)]
    pub fn bex(&mut self) -> BexW<MdmaC2crSpec> {
        BexW::new(self, 12)
    }
    #[doc = "Bit 13 - Half word Endianes exchange"]
    #[inline(always)]
    pub fn hex(&mut self) -> HexW<MdmaC2crSpec> {
        HexW::new(self, 13)
    }
    #[doc = "Bit 14 - Word Endianness exchange"]
    #[inline(always)]
    pub fn wex(&mut self) -> WexW<MdmaC2crSpec> {
        WexW::new(self, 14)
    }
    #[doc = "Bit 16 - SW ReQuest Writing a 1 into this bit sets the CRQAx in MDMA_ISRy register, activating the request on Channel x Note: Either the whole CxCR register or the 8-bit/16-bit register @ Address offset: 0x4E + 0x40 chn may be used for SWRQ activation. In case of a SW request, acknowledge is not generated (neither HW signal, nor CxMAR write access)."]
    #[inline(always)]
    pub fn swrq(&mut self) -> SwrqW<MdmaC2crSpec> {
        SwrqW::new(self, 16)
    }
}
#[doc = "This register is used to control the concerned channel.\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c2cr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c2cr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC2crSpec;
impl crate::RegisterSpec for MdmaC2crSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c2cr::R`](R) reader structure"]
impl crate::Readable for MdmaC2crSpec {}
#[doc = "`write(|w| ..)` method takes [`mdma_c2cr::W`](W) writer structure"]
impl crate::Writable for MdmaC2crSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C2CR to value 0"]
impl crate::Resettable for MdmaC2crSpec {}
