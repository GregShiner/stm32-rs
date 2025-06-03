#[doc = "Register `CCR1` reader"]
pub type R = crate::R<Ccr1Spec>;
#[doc = "Register `CCR1` writer"]
pub type W = crate::W<Ccr1Spec>;
#[doc = "Field `EN` reader - Channel enable This bit is set and cleared by software."]
pub type EnR = crate::BitReader;
#[doc = "Field `EN` writer - Channel enable This bit is set and cleared by software."]
pub type EnW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable This bit is set and cleared by software."]
pub type TcieR = crate::BitReader;
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable This bit is set and cleared by software."]
pub type TcieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HTIE` reader - Half transfer interrupt enable This bit is set and cleared by software."]
pub type HtieR = crate::BitReader;
#[doc = "Field `HTIE` writer - Half transfer interrupt enable This bit is set and cleared by software."]
pub type HtieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TEIE` reader - Transfer error interrupt enable This bit is set and cleared by software."]
pub type TeieR = crate::BitReader;
#[doc = "Field `TEIE` writer - Transfer error interrupt enable This bit is set and cleared by software."]
pub type TeieW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DIR` reader - Data transfer direction This bit is set and cleared by software."]
pub type DirR = crate::BitReader;
#[doc = "Field `DIR` writer - Data transfer direction This bit is set and cleared by software."]
pub type DirW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CIRC` reader - Circular mode This bit is set and cleared by software."]
pub type CircR = crate::BitReader;
#[doc = "Field `CIRC` writer - Circular mode This bit is set and cleared by software."]
pub type CircW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PINC` reader - Peripheral increment mode This bit is set and cleared by software."]
pub type PincR = crate::BitReader;
#[doc = "Field `PINC` writer - Peripheral increment mode This bit is set and cleared by software."]
pub type PincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MINC` reader - Memory increment mode This bit is set and cleared by software."]
pub type MincR = crate::BitReader;
#[doc = "Field `MINC` writer - Memory increment mode This bit is set and cleared by software."]
pub type MincW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PSIZE` reader - Peripheral size These bits are set and cleared by software."]
pub type PsizeR = crate::FieldReader;
#[doc = "Field `PSIZE` writer - Peripheral size These bits are set and cleared by software."]
pub type PsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MSIZE` reader - Memory size These bits are set and cleared by software."]
pub type MsizeR = crate::FieldReader;
#[doc = "Field `MSIZE` writer - Memory size These bits are set and cleared by software."]
pub type MsizeW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PL` reader - Channel priority level These bits are set and cleared by software."]
pub type PlR = crate::FieldReader;
#[doc = "Field `PL` writer - Channel priority level These bits are set and cleared by software."]
pub type PlW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `MEM2MEM` reader - Memory to memory mode This bit is set and cleared by software."]
pub type Mem2memR = crate::BitReader;
#[doc = "Field `MEM2MEM` writer - Memory to memory mode This bit is set and cleared by software."]
pub type Mem2memW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Channel enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn en(&self) -> EnR {
        EnR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tcie(&self) -> TcieR {
        TcieR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half transfer interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn htie(&self) -> HtieR {
        HtieR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn teie(&self) -> TeieR {
        TeieR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data transfer direction This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dir(&self) -> DirR {
        DirR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Circular mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn circ(&self) -> CircR {
        CircR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral increment mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn pinc(&self) -> PincR {
        PincR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Memory increment mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn minc(&self) -> MincR {
        MincR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Peripheral size These bits are set and cleared by software."]
    #[inline(always)]
    pub fn psize(&self) -> PsizeR {
        PsizeR::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Memory size These bits are set and cleared by software."]
    #[inline(always)]
    pub fn msize(&self) -> MsizeR {
        MsizeR::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Channel priority level These bits are set and cleared by software."]
    #[inline(always)]
    pub fn pl(&self) -> PlR {
        PlR::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Memory to memory mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn mem2mem(&self) -> Mem2memR {
        Mem2memR::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn en(&mut self) -> EnW<Ccr1Spec> {
        EnW::new(self, 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn tcie(&mut self) -> TcieW<Ccr1Spec> {
        TcieW::new(self, 1)
    }
    #[doc = "Bit 2 - Half transfer interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn htie(&mut self) -> HtieW<Ccr1Spec> {
        HtieW::new(self, 2)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable This bit is set and cleared by software."]
    #[inline(always)]
    pub fn teie(&mut self) -> TeieW<Ccr1Spec> {
        TeieW::new(self, 3)
    }
    #[doc = "Bit 4 - Data transfer direction This bit is set and cleared by software."]
    #[inline(always)]
    pub fn dir(&mut self) -> DirW<Ccr1Spec> {
        DirW::new(self, 4)
    }
    #[doc = "Bit 5 - Circular mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn circ(&mut self) -> CircW<Ccr1Spec> {
        CircW::new(self, 5)
    }
    #[doc = "Bit 6 - Peripheral increment mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn pinc(&mut self) -> PincW<Ccr1Spec> {
        PincW::new(self, 6)
    }
    #[doc = "Bit 7 - Memory increment mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn minc(&mut self) -> MincW<Ccr1Spec> {
        MincW::new(self, 7)
    }
    #[doc = "Bits 8:9 - Peripheral size These bits are set and cleared by software."]
    #[inline(always)]
    pub fn psize(&mut self) -> PsizeW<Ccr1Spec> {
        PsizeW::new(self, 8)
    }
    #[doc = "Bits 10:11 - Memory size These bits are set and cleared by software."]
    #[inline(always)]
    pub fn msize(&mut self) -> MsizeW<Ccr1Spec> {
        MsizeW::new(self, 10)
    }
    #[doc = "Bits 12:13 - Channel priority level These bits are set and cleared by software."]
    #[inline(always)]
    pub fn pl(&mut self) -> PlW<Ccr1Spec> {
        PlW::new(self, 12)
    }
    #[doc = "Bit 14 - Memory to memory mode This bit is set and cleared by software."]
    #[inline(always)]
    pub fn mem2mem(&mut self) -> Mem2memW<Ccr1Spec> {
        Mem2memW::new(self, 14)
    }
}
#[doc = "DMA channel x configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`ccr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Ccr1Spec;
impl crate::RegisterSpec for Ccr1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ccr1::R`](R) reader structure"]
impl crate::Readable for Ccr1Spec {}
#[doc = "`write(|w| ..)` method takes [`ccr1::W`](W) writer structure"]
impl crate::Writable for Ccr1Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CCR1 to value 0"]
impl crate::Resettable for Ccr1Spec {}
