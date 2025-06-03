#[doc = "Register `APB1HLPENR` reader"]
pub type R = crate::R<Apb1hlpenrSpec>;
#[doc = "Register `APB1HLPENR` writer"]
pub type W = crate::W<Apb1hlpenrSpec>;
#[doc = "Field `CRSLPEN` reader - Clock Recovery System peripheral clock enable during CSleep mode"]
pub type CrslpenR = crate::BitReader;
#[doc = "Field `CRSLPEN` writer - Clock Recovery System peripheral clock enable during CSleep mode"]
pub type CrslpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWPLPEN` reader - SWPMI Peripheral Clocks Enable During CSleep Mode"]
pub type SwplpenR = crate::BitReader;
#[doc = "Field `SWPLPEN` writer - SWPMI Peripheral Clocks Enable During CSleep Mode"]
pub type SwplpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMPLPEN` reader - OPAMP peripheral clock enable during CSleep mode"]
pub type OpamplpenR = crate::BitReader;
#[doc = "Field `OPAMPLPEN` writer - OPAMP peripheral clock enable during CSleep mode"]
pub type OpamplpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDIOSLPEN` reader - MDIOS peripheral clock enable during CSleep mode"]
pub type MdioslpenR = crate::BitReader;
#[doc = "Field `MDIOSLPEN` writer - MDIOS peripheral clock enable during CSleep mode"]
pub type MdioslpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDCANLPEN` reader - FDCAN Peripheral Clocks Enable During CSleep Mode"]
pub type FdcanlpenR = crate::BitReader;
#[doc = "Field `FDCANLPEN` writer - FDCAN Peripheral Clocks Enable During CSleep Mode"]
pub type FdcanlpenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Clock Recovery System peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn crslpen(&self) -> CrslpenR {
        CrslpenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SWPMI Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn swplpen(&self) -> SwplpenR {
        SwplpenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn opamplpen(&self) -> OpamplpenR {
        OpamplpenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn mdioslpen(&self) -> MdioslpenR {
        MdioslpenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - FDCAN Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn fdcanlpen(&self) -> FdcanlpenR {
        FdcanlpenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clock Recovery System peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn crslpen(&mut self) -> CrslpenW<Apb1hlpenrSpec> {
        CrslpenW::new(self, 1)
    }
    #[doc = "Bit 2 - SWPMI Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn swplpen(&mut self) -> SwplpenW<Apb1hlpenrSpec> {
        SwplpenW::new(self, 2)
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn opamplpen(&mut self) -> OpamplpenW<Apb1hlpenrSpec> {
        OpamplpenW::new(self, 4)
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable during CSleep mode"]
    #[inline(always)]
    pub fn mdioslpen(&mut self) -> MdioslpenW<Apb1hlpenrSpec> {
        MdioslpenW::new(self, 5)
    }
    #[doc = "Bit 8 - FDCAN Peripheral Clocks Enable During CSleep Mode"]
    #[inline(always)]
    pub fn fdcanlpen(&mut self) -> FdcanlpenW<Apb1hlpenrSpec> {
        FdcanlpenW::new(self, 8)
    }
}
#[doc = "RCC APB1 High Sleep Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1hlpenr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hlpenr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1hlpenrSpec;
impl crate::RegisterSpec for Apb1hlpenrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1hlpenr::R`](R) reader structure"]
impl crate::Readable for Apb1hlpenrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1hlpenr::W`](W) writer structure"]
impl crate::Writable for Apb1hlpenrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1HLPENR to value 0"]
impl crate::Resettable for Apb1hlpenrSpec {}
