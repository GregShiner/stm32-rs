#[doc = "Register `IMR` reader"]
pub type R = crate::R<ImrSpec>;
#[doc = "Register `IMR` writer"]
pub type W = crate::W<ImrSpec>;
#[doc = "Field `RXNEIE` reader - RXNE interrupt enable"]
pub type RxneieR = crate::BitReader;
#[doc = "Field `RXNEIE` writer - RXNE interrupt enable"]
pub type RxneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CSRNEIE` reader - Control Buffer Ready Interrupt Enable"]
pub type CsrneieR = crate::BitReader;
#[doc = "Field `CSRNEIE` writer - Control Buffer Ready Interrupt Enable"]
pub type CsrneieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PERRIE` reader - Parity error interrupt enable"]
pub type PerrieR = crate::BitReader;
#[doc = "Field `PERRIE` writer - Parity error interrupt enable"]
pub type PerrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OVRIE` reader - Overrun error Interrupt Enable"]
pub type OvrieR = crate::BitReader;
#[doc = "Field `OVRIE` writer - Overrun error Interrupt Enable"]
pub type OvrieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SBLKIE` reader - Synchronization Block Detected Interrupt Enable"]
pub type SblkieR = crate::BitReader;
#[doc = "Field `SBLKIE` writer - Synchronization Block Detected Interrupt Enable"]
pub type SblkieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCDIE` reader - Synchronization Done"]
pub type SyncdieR = crate::BitReader;
#[doc = "Field `SYNCDIE` writer - Synchronization Done"]
pub type SyncdieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IFEIE` reader - Serial Interface Error Interrupt Enable"]
pub type IfeieR = crate::BitReader;
#[doc = "Field `IFEIE` writer - Serial Interface Error Interrupt Enable"]
pub type IfeieW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RxneieR {
        RxneieR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Control Buffer Ready Interrupt Enable"]
    #[inline(always)]
    pub fn csrneie(&self) -> CsrneieR {
        CsrneieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Parity error interrupt enable"]
    #[inline(always)]
    pub fn perrie(&self) -> PerrieR {
        PerrieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Overrun error Interrupt Enable"]
    #[inline(always)]
    pub fn ovrie(&self) -> OvrieR {
        OvrieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Synchronization Block Detected Interrupt Enable"]
    #[inline(always)]
    pub fn sblkie(&self) -> SblkieR {
        SblkieR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Synchronization Done"]
    #[inline(always)]
    pub fn syncdie(&self) -> SyncdieR {
        SyncdieR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Serial Interface Error Interrupt Enable"]
    #[inline(always)]
    pub fn ifeie(&self) -> IfeieR {
        IfeieR::new(((self.bits >> 6) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&mut self) -> RxneieW<ImrSpec> {
        RxneieW::new(self, 0)
    }
    #[doc = "Bit 1 - Control Buffer Ready Interrupt Enable"]
    #[inline(always)]
    pub fn csrneie(&mut self) -> CsrneieW<ImrSpec> {
        CsrneieW::new(self, 1)
    }
    #[doc = "Bit 2 - Parity error interrupt enable"]
    #[inline(always)]
    pub fn perrie(&mut self) -> PerrieW<ImrSpec> {
        PerrieW::new(self, 2)
    }
    #[doc = "Bit 3 - Overrun error Interrupt Enable"]
    #[inline(always)]
    pub fn ovrie(&mut self) -> OvrieW<ImrSpec> {
        OvrieW::new(self, 3)
    }
    #[doc = "Bit 4 - Synchronization Block Detected Interrupt Enable"]
    #[inline(always)]
    pub fn sblkie(&mut self) -> SblkieW<ImrSpec> {
        SblkieW::new(self, 4)
    }
    #[doc = "Bit 5 - Synchronization Done"]
    #[inline(always)]
    pub fn syncdie(&mut self) -> SyncdieW<ImrSpec> {
        SyncdieW::new(self, 5)
    }
    #[doc = "Bit 6 - Serial Interface Error Interrupt Enable"]
    #[inline(always)]
    pub fn ifeie(&mut self) -> IfeieW<ImrSpec> {
        IfeieW::new(self, 6)
    }
}
#[doc = "Interrupt mask register\n\nYou can [`read`](crate::Reg::read) this register and get [`imr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`imr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ImrSpec;
impl crate::RegisterSpec for ImrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`imr::R`](R) reader structure"]
impl crate::Readable for ImrSpec {}
#[doc = "`write(|w| ..)` method takes [`imr::W`](W) writer structure"]
impl crate::Writable for ImrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IMR to value 0"]
impl crate::Resettable for ImrSpec {}
