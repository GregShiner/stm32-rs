#[doc = "Register `MDMA_C8BNDTR` reader"]
pub type R = crate::R<MdmaC8bndtrSpec>;
#[doc = "Register `MDMA_C8BNDTR` writer"]
pub type W = crate::W<MdmaC8bndtrSpec>;
#[doc = "Field `BNDT` reader - block number of data to transfer"]
pub type BndtR = crate::FieldReader<u32>;
#[doc = "Field `BNDT` writer - block number of data to transfer"]
pub type BndtW<'a, REG> = crate::FieldWriter<'a, REG, 17, u32>;
#[doc = "Field `BRSUM` reader - Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0."]
pub type BrsumR = crate::BitReader;
#[doc = "Field `BRSUM` writer - Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0."]
pub type BrsumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRDUM` reader - Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0."]
pub type BrdumR = crate::BitReader;
#[doc = "Field `BRDUM` writer - Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0."]
pub type BrdumW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BRC` reader - Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0."]
pub type BrcR = crate::FieldReader<u16>;
#[doc = "Field `BRC` writer - Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0."]
pub type BrcW<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:16 - block number of data to transfer"]
    #[inline(always)]
    pub fn bndt(&self) -> BndtR {
        BndtR::new(self.bits & 0x0001_ffff)
    }
    #[doc = "Bit 18 - Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn brsum(&self) -> BrsumR {
        BrsumR::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn brdum(&self) -> BrdumR {
        BrdumR::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:31 - Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn brc(&self) -> BrcR {
        BrcR::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:16 - block number of data to transfer"]
    #[inline(always)]
    pub fn bndt(&mut self) -> BndtW<MdmaC8bndtrSpec> {
        BndtW::new(self, 0)
    }
    #[doc = "Bit 18 - Block Repeat Source address Update Mode These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn brsum(&mut self) -> BrsumW<MdmaC8bndtrSpec> {
        BrsumW::new(self, 18)
    }
    #[doc = "Bit 19 - Block Repeat Destination address Update Mode These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn brdum(&mut self) -> BrdumW<MdmaC8bndtrSpec> {
        BrdumW::new(self, 19)
    }
    #[doc = "Bits 20:31 - Block Repeat Count This field contains the number of repetitions of the current block (0 to 4095). When the channel is enabled, this register is read-only, indicating the remaining number of blocks, excluding the current one. This register decrements after each complete block transfer. Once the last block transfer has completed, this register can either stay at zero or be reloaded automatically from memory (in Linked List mode - i.e. Link Address valid). These bits are protected and can be written only if EN is 0."]
    #[inline(always)]
    pub fn brc(&mut self) -> BrcW<MdmaC8bndtrSpec> {
        BrcW::new(self, 20)
    }
}
#[doc = "MDMA Channel x block number of data register\n\nYou can [`read`](crate::Reg::read) this register and get [`mdma_c8bndtr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdma_c8bndtr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MdmaC8bndtrSpec;
impl crate::RegisterSpec for MdmaC8bndtrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mdma_c8bndtr::R`](R) reader structure"]
impl crate::Readable for MdmaC8bndtrSpec {}
#[doc = "`write(|w| ..)` method takes [`mdma_c8bndtr::W`](W) writer structure"]
impl crate::Writable for MdmaC8bndtrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MDMA_C8BNDTR to value 0"]
impl crate::Resettable for MdmaC8bndtrSpec {}
