#[doc = "Register `APB1HENR` reader"]
pub type R = crate::R<Apb1henrSpec>;
#[doc = "Register `APB1HENR` writer"]
pub type W = crate::W<Apb1henrSpec>;
#[doc = "Field `CRSEN` reader - Clock Recovery System peripheral clock enable"]
pub type CrsenR = crate::BitReader;
#[doc = "Field `CRSEN` writer - Clock Recovery System peripheral clock enable"]
pub type CrsenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWPEN` reader - SWPMI Peripheral Clocks Enable"]
pub type SwpenR = crate::BitReader;
#[doc = "Field `SWPEN` writer - SWPMI Peripheral Clocks Enable"]
pub type SwpenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMPEN` reader - OPAMP peripheral clock enable"]
pub type OpampenR = crate::BitReader;
#[doc = "Field `OPAMPEN` writer - OPAMP peripheral clock enable"]
pub type OpampenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDIOSEN` reader - MDIOS peripheral clock enable"]
pub type MdiosenR = crate::BitReader;
#[doc = "Field `MDIOSEN` writer - MDIOS peripheral clock enable"]
pub type MdiosenW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDCANEN` reader - FDCAN Peripheral Clocks Enable"]
pub type FdcanenR = crate::BitReader;
#[doc = "Field `FDCANEN` writer - FDCAN Peripheral Clocks Enable"]
pub type FdcanenW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Clock Recovery System peripheral clock enable"]
    #[inline(always)]
    pub fn crsen(&self) -> CrsenR {
        CrsenR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SWPMI Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn swpen(&self) -> SwpenR {
        SwpenR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable"]
    #[inline(always)]
    pub fn opampen(&self) -> OpampenR {
        OpampenR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable"]
    #[inline(always)]
    pub fn mdiosen(&self) -> MdiosenR {
        MdiosenR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - FDCAN Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn fdcanen(&self) -> FdcanenR {
        FdcanenR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clock Recovery System peripheral clock enable"]
    #[inline(always)]
    pub fn crsen(&mut self) -> CrsenW<Apb1henrSpec> {
        CrsenW::new(self, 1)
    }
    #[doc = "Bit 2 - SWPMI Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn swpen(&mut self) -> SwpenW<Apb1henrSpec> {
        SwpenW::new(self, 2)
    }
    #[doc = "Bit 4 - OPAMP peripheral clock enable"]
    #[inline(always)]
    pub fn opampen(&mut self) -> OpampenW<Apb1henrSpec> {
        OpampenW::new(self, 4)
    }
    #[doc = "Bit 5 - MDIOS peripheral clock enable"]
    #[inline(always)]
    pub fn mdiosen(&mut self) -> MdiosenW<Apb1henrSpec> {
        MdiosenW::new(self, 5)
    }
    #[doc = "Bit 8 - FDCAN Peripheral Clocks Enable"]
    #[inline(always)]
    pub fn fdcanen(&mut self) -> FdcanenW<Apb1henrSpec> {
        FdcanenW::new(self, 8)
    }
}
#[doc = "RCC APB1 Clock Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1henr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1henr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1henrSpec;
impl crate::RegisterSpec for Apb1henrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1henr::R`](R) reader structure"]
impl crate::Readable for Apb1henrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1henr::W`](W) writer structure"]
impl crate::Writable for Apb1henrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1HENR to value 0"]
impl crate::Resettable for Apb1henrSpec {}
