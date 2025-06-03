#[doc = "Register `APB1HRSTR` reader"]
pub type R = crate::R<Apb1hrstrSpec>;
#[doc = "Register `APB1HRSTR` writer"]
pub type W = crate::W<Apb1hrstrSpec>;
#[doc = "Field `CRSRST` reader - Clock Recovery System reset"]
pub type CrsrstR = crate::BitReader;
#[doc = "Field `CRSRST` writer - Clock Recovery System reset"]
pub type CrsrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SWPRST` reader - SWPMI block reset"]
pub type SwprstR = crate::BitReader;
#[doc = "Field `SWPRST` writer - SWPMI block reset"]
pub type SwprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OPAMPRST` reader - OPAMP block reset"]
pub type OpamprstR = crate::BitReader;
#[doc = "Field `OPAMPRST` writer - OPAMP block reset"]
pub type OpamprstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MDIOSRST` reader - MDIOS block reset"]
pub type MdiosrstR = crate::BitReader;
#[doc = "Field `MDIOSRST` writer - MDIOS block reset"]
pub type MdiosrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FDCANRST` reader - FDCAN block reset"]
pub type FdcanrstR = crate::BitReader;
#[doc = "Field `FDCANRST` writer - FDCAN block reset"]
pub type FdcanrstW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Clock Recovery System reset"]
    #[inline(always)]
    pub fn crsrst(&self) -> CrsrstR {
        CrsrstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SWPMI block reset"]
    #[inline(always)]
    pub fn swprst(&self) -> SwprstR {
        SwprstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - OPAMP block reset"]
    #[inline(always)]
    pub fn opamprst(&self) -> OpamprstR {
        OpamprstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - MDIOS block reset"]
    #[inline(always)]
    pub fn mdiosrst(&self) -> MdiosrstR {
        MdiosrstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 8 - FDCAN block reset"]
    #[inline(always)]
    pub fn fdcanrst(&self) -> FdcanrstR {
        FdcanrstR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Clock Recovery System reset"]
    #[inline(always)]
    pub fn crsrst(&mut self) -> CrsrstW<Apb1hrstrSpec> {
        CrsrstW::new(self, 1)
    }
    #[doc = "Bit 2 - SWPMI block reset"]
    #[inline(always)]
    pub fn swprst(&mut self) -> SwprstW<Apb1hrstrSpec> {
        SwprstW::new(self, 2)
    }
    #[doc = "Bit 4 - OPAMP block reset"]
    #[inline(always)]
    pub fn opamprst(&mut self) -> OpamprstW<Apb1hrstrSpec> {
        OpamprstW::new(self, 4)
    }
    #[doc = "Bit 5 - MDIOS block reset"]
    #[inline(always)]
    pub fn mdiosrst(&mut self) -> MdiosrstW<Apb1hrstrSpec> {
        MdiosrstW::new(self, 5)
    }
    #[doc = "Bit 8 - FDCAN block reset"]
    #[inline(always)]
    pub fn fdcanrst(&mut self) -> FdcanrstW<Apb1hrstrSpec> {
        FdcanrstW::new(self, 8)
    }
}
#[doc = "RCC APB1 Peripheral Reset Register\n\nYou can [`read`](crate::Reg::read) this register and get [`apb1hrstr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apb1hrstr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Apb1hrstrSpec;
impl crate::RegisterSpec for Apb1hrstrSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb1hrstr::R`](R) reader structure"]
impl crate::Readable for Apb1hrstrSpec {}
#[doc = "`write(|w| ..)` method takes [`apb1hrstr::W`](W) writer structure"]
impl crate::Writable for Apb1hrstrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets APB1HRSTR to value 0"]
impl crate::Resettable for Apb1hrstrSpec {}
