#[doc = "Register `ISR` reader"]
pub type R = crate::R<IsrSpec>;
#[doc = "Register `ISR` writer"]
pub type W = crate::W<IsrSpec>;
#[doc = "Field `FLT1` reader - Fault 1 Interrupt Flag"]
pub type Flt1R = crate::BitReader;
#[doc = "Field `FLT2` reader - Fault 2 Interrupt Flag"]
pub type Flt2R = crate::BitReader;
#[doc = "Field `FLT3` reader - Fault 3 Interrupt Flag"]
pub type Flt3R = crate::BitReader;
#[doc = "Field `FLT4` reader - Fault 4 Interrupt Flag"]
pub type Flt4R = crate::BitReader;
#[doc = "Field `FLT5` reader - Fault 5 Interrupt Flag"]
pub type Flt5R = crate::BitReader;
#[doc = "Field `SYSFLT` reader - System Fault Interrupt Flag"]
pub type SysfltR = crate::BitReader;
#[doc = "Field `SYSFLT` writer - System Fault Interrupt Flag"]
pub type SysfltW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DLLRDY` reader - DLL Ready Interrupt Flag"]
pub type DllrdyR = crate::BitReader;
#[doc = "Field `BMPER` reader - Burst mode Period Interrupt Flag"]
pub type BmperR = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Fault 1 Interrupt Flag"]
    #[inline(always)]
    pub fn flt1(&self) -> Flt1R {
        Flt1R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Fault 2 Interrupt Flag"]
    #[inline(always)]
    pub fn flt2(&self) -> Flt2R {
        Flt2R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Fault 3 Interrupt Flag"]
    #[inline(always)]
    pub fn flt3(&self) -> Flt3R {
        Flt3R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Fault 4 Interrupt Flag"]
    #[inline(always)]
    pub fn flt4(&self) -> Flt4R {
        Flt4R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Fault 5 Interrupt Flag"]
    #[inline(always)]
    pub fn flt5(&self) -> Flt5R {
        Flt5R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - System Fault Interrupt Flag"]
    #[inline(always)]
    pub fn sysflt(&self) -> SysfltR {
        SysfltR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 16 - DLL Ready Interrupt Flag"]
    #[inline(always)]
    pub fn dllrdy(&self) -> DllrdyR {
        DllrdyR::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Burst mode Period Interrupt Flag"]
    #[inline(always)]
    pub fn bmper(&self) -> BmperR {
        BmperR::new(((self.bits >> 17) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 5 - System Fault Interrupt Flag"]
    #[inline(always)]
    pub fn sysflt(&mut self) -> SysfltW<IsrSpec> {
        SysfltW::new(self, 5)
    }
}
#[doc = "Interrupt Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`isr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IsrSpec;
impl crate::RegisterSpec for IsrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isr::R`](R) reader structure"]
impl crate::Readable for IsrSpec {}
#[doc = "`write(|w| ..)` method takes [`isr::W`](W) writer structure"]
impl crate::Writable for IsrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ISR to value 0"]
impl crate::Resettable for IsrSpec {}
