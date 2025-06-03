#[doc = "Register `MTLQICSR` reader"]
pub type R = crate::R<MtlqicsrSpec>;
#[doc = "Register `MTLQICSR` writer"]
pub type W = crate::W<MtlqicsrSpec>;
#[doc = "Field `TXUNFIS` reader - TXUNFIS"]
pub type TxunfisR = crate::BitReader;
#[doc = "Field `TXUNFIS` writer - TXUNFIS"]
pub type TxunfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TXUIE` reader - TXUIE"]
pub type TxuieR = crate::BitReader;
#[doc = "Field `TXUIE` writer - TXUIE"]
pub type TxuieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOVFIS` reader - RXOVFIS"]
pub type RxovfisR = crate::BitReader;
#[doc = "Field `RXOVFIS` writer - RXOVFIS"]
pub type RxovfisW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RXOIE` reader - RXOIE"]
pub type RxoieR = crate::BitReader;
#[doc = "Field `RXOIE` writer - RXOIE"]
pub type RxoieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - TXUNFIS"]
    #[inline(always)]
    pub fn txunfis(&self) -> TxunfisR {
        TxunfisR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - TXUIE"]
    #[inline(always)]
    pub fn txuie(&self) -> TxuieR {
        TxuieR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 16 - RXOVFIS"]
    #[inline(always)]
    pub fn rxovfis(&self) -> RxovfisR {
        RxovfisR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 24 - RXOIE"]
    #[inline(always)]
    pub fn rxoie(&self) -> RxoieR {
        RxoieR::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - TXUNFIS"]
    #[inline(always)]
    pub fn txunfis(&mut self) -> TxunfisW<MtlqicsrSpec> {
        TxunfisW::new(self, 0)
    }
    #[doc = "Bit 8 - TXUIE"]
    #[inline(always)]
    pub fn txuie(&mut self) -> TxuieW<MtlqicsrSpec> {
        TxuieW::new(self, 8)
    }
    #[doc = "Bit 16 - RXOVFIS"]
    #[inline(always)]
    pub fn rxovfis(&mut self) -> RxovfisW<MtlqicsrSpec> {
        RxovfisW::new(self, 16)
    }
    #[doc = "Bit 24 - RXOIE"]
    #[inline(always)]
    pub fn rxoie(&mut self) -> RxoieW<MtlqicsrSpec> {
        RxoieW::new(self, 24)
    }
}
#[doc = "Queue interrupt control status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`mtlqicsr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mtlqicsr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MtlqicsrSpec;
impl crate::RegisterSpec for MtlqicsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mtlqicsr::R`](R) reader structure"]
impl crate::Readable for MtlqicsrSpec {}
#[doc = "`write(|w| ..)` method takes [`mtlqicsr::W`](W) writer structure"]
impl crate::Writable for MtlqicsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MTLQICSR to value 0"]
impl crate::Resettable for MtlqicsrSpec {}
