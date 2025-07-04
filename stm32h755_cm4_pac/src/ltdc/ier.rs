#[doc = "Register `IER` reader"]
pub type R = crate::R<IerSpec>;
#[doc = "Register `IER` writer"]
pub type W = crate::W<IerSpec>;
#[doc = "Field `LIE` reader - Line Interrupt Enable"]
pub type LieR = crate::BitReader;
#[doc = "Field `LIE` writer - Line Interrupt Enable"]
pub type LieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FUIE` reader - FIFO Underrun Interrupt Enable"]
pub type FuieR = crate::BitReader;
#[doc = "Field `FUIE` writer - FIFO Underrun Interrupt Enable"]
pub type FuieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TERRIE` reader - Transfer Error Interrupt Enable"]
pub type TerrieR = crate::BitReader;
#[doc = "Field `TERRIE` writer - Transfer Error Interrupt Enable"]
pub type TerrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RRIE` reader - Register Reload interrupt enable"]
pub type RrieR = crate::BitReader;
#[doc = "Field `RRIE` writer - Register Reload interrupt enable"]
pub type RrieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Line Interrupt Enable"]
    #[inline(always)]
    pub fn lie(&self) -> LieR {
        LieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FIFO Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn fuie(&self) -> FuieR {
        FuieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Transfer Error Interrupt Enable"]
    #[inline(always)]
    pub fn terrie(&self) -> TerrieR {
        TerrieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Register Reload interrupt enable"]
    #[inline(always)]
    pub fn rrie(&self) -> RrieR {
        RrieR::new(((self.bits >> 3) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Line Interrupt Enable"]
    #[inline(always)]
    pub fn lie(&mut self) -> LieW<IerSpec> {
        LieW::new(self, 0)
    }
    #[doc = "Bit 1 - FIFO Underrun Interrupt Enable"]
    #[inline(always)]
    pub fn fuie(&mut self) -> FuieW<IerSpec> {
        FuieW::new(self, 1)
    }
    #[doc = "Bit 2 - Transfer Error Interrupt Enable"]
    #[inline(always)]
    pub fn terrie(&mut self) -> TerrieW<IerSpec> {
        TerrieW::new(self, 2)
    }
    #[doc = "Bit 3 - Register Reload interrupt enable"]
    #[inline(always)]
    pub fn rrie(&mut self) -> RrieW<IerSpec> {
        RrieW::new(self, 3)
    }
}
#[doc = "Interrupt Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ier::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ier::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IerSpec;
impl crate::RegisterSpec for IerSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ier::R`](R) reader structure"]
impl crate::Readable for IerSpec {}
#[doc = "`write(|w| ..)` method takes [`ier::W`](W) writer structure"]
impl crate::Writable for IerSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IER to value 0"]
impl crate::Resettable for IerSpec {}
