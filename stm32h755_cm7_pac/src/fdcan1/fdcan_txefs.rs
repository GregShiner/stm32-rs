#[doc = "Register `FDCAN_TXEFS` reader"]
pub type R = crate::R<FdcanTxefsSpec>;
#[doc = "Register `FDCAN_TXEFS` writer"]
pub type W = crate::W<FdcanTxefsSpec>;
#[doc = "Field `EFFL` reader - Event FIFO Fill Level"]
pub type EfflR = crate::FieldReader;
#[doc = "Field `EFFL` writer - Event FIFO Fill Level"]
pub type EfflW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `EFGI` reader - Event FIFO Get Index."]
pub type EfgiR = crate::FieldReader;
#[doc = "Field `EFGI` writer - Event FIFO Get Index."]
pub type EfgiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EFPI` reader - Event FIFO put index."]
pub type EfpiR = crate::FieldReader;
#[doc = "Field `EFPI` writer - Event FIFO put index."]
pub type EfpiW<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `EFF` reader - Event FIFO Full."]
pub type EffR = crate::BitReader;
#[doc = "Field `EFF` writer - Event FIFO Full."]
pub type EffW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEFL` reader - Tx Event FIFO Element Lost."]
pub type TeflR = crate::BitReader;
#[doc = "Field `TEFL` writer - Tx Event FIFO Element Lost."]
pub type TeflW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - Event FIFO Fill Level"]
    #[inline(always)]
    pub fn effl(&self) -> EfflR {
        EfflR::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 8:12 - Event FIFO Get Index."]
    #[inline(always)]
    pub fn efgi(&self) -> EfgiR {
        EfgiR::new(((self.bits >> 8) & 0x1f) as u8)
    }
    #[doc = "Bits 16:20 - Event FIFO put index."]
    #[inline(always)]
    pub fn efpi(&self) -> EfpiR {
        EfpiR::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bit 24 - Event FIFO Full."]
    #[inline(always)]
    pub fn eff(&self) -> EffR {
        EffR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Tx Event FIFO Element Lost."]
    #[inline(always)]
    pub fn tefl(&self) -> TeflR {
        TeflR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - Event FIFO Fill Level"]
    #[inline(always)]
    pub fn effl(&mut self) -> EfflW<FdcanTxefsSpec> {
        EfflW::new(self, 0)
    }
    #[doc = "Bits 8:12 - Event FIFO Get Index."]
    #[inline(always)]
    pub fn efgi(&mut self) -> EfgiW<FdcanTxefsSpec> {
        EfgiW::new(self, 8)
    }
    #[doc = "Bits 16:20 - Event FIFO put index."]
    #[inline(always)]
    pub fn efpi(&mut self) -> EfpiW<FdcanTxefsSpec> {
        EfpiW::new(self, 16)
    }
    #[doc = "Bit 24 - Event FIFO Full."]
    #[inline(always)]
    pub fn eff(&mut self) -> EffW<FdcanTxefsSpec> {
        EffW::new(self, 24)
    }
    #[doc = "Bit 25 - Tx Event FIFO Element Lost."]
    #[inline(always)]
    pub fn tefl(&mut self) -> TeflW<FdcanTxefsSpec> {
        TeflW::new(self, 25)
    }
}
#[doc = "FDCAN Tx Event FIFO Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txefs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txefs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxefsSpec;
impl crate::RegisterSpec for FdcanTxefsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txefs::R`](R) reader structure"]
impl crate::Readable for FdcanTxefsSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txefs::W`](W) writer structure"]
impl crate::Writable for FdcanTxefsSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TXEFS to value 0"]
impl crate::Resettable for FdcanTxefsSpec {}
