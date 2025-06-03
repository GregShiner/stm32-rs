#[doc = "Register `FDCAN_RXF1S` reader"]
pub type R = crate::R<FdcanRxf1sSpec>;
#[doc = "Register `FDCAN_RXF1S` writer"]
pub type W = crate::W<FdcanRxf1sSpec>;
#[doc = "Field `F1FL` reader - Rx FIFO 1 Fill Level"]
pub type F1flR = crate::FieldReader;
#[doc = "Field `F1FL` writer - Rx FIFO 1 Fill Level"]
pub type F1flW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F1GI` reader - Rx FIFO 1 Get Index"]
pub type F1giR = crate::FieldReader;
#[doc = "Field `F1GI` writer - Rx FIFO 1 Get Index"]
pub type F1giW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F1PI` reader - Rx FIFO 1 Put Index"]
pub type F1piR = crate::FieldReader;
#[doc = "Field `F1PI` writer - Rx FIFO 1 Put Index"]
pub type F1piW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F1F` reader - Rx FIFO 1 Full"]
pub type F1fR = crate::BitReader;
#[doc = "Field `F1F` writer - Rx FIFO 1 Full"]
pub type F1fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF1L` reader - Rx FIFO 1 Message Lost"]
pub type Rf1lR = crate::BitReader;
#[doc = "Field `RF1L` writer - Rx FIFO 1 Message Lost"]
pub type Rf1lW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DMS` reader - Debug Message Status"]
pub type DmsR = crate::FieldReader;
#[doc = "Field `DMS` writer - Debug Message Status"]
pub type DmsW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 1 Fill Level"]
    #[inline(always)]
    pub fn f1fl(&self) -> F1flR {
        F1flR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Rx FIFO 1 Get Index"]
    #[inline(always)]
    pub fn f1gi(&self) -> F1giR {
        F1giR::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Rx FIFO 1 Put Index"]
    #[inline(always)]
    pub fn f1pi(&self) -> F1piR {
        F1piR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 1 Full"]
    #[inline(always)]
    pub fn f1f(&self) -> F1fR {
        F1fR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&self) -> Rf1lR {
        Rf1lR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 30:31 - Debug Message Status"]
    #[inline(always)]
    pub fn dms(&self) -> DmsR {
        DmsR::new(((self.bits >> 30) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Rx FIFO 1 Fill Level"]
    #[inline(always)]
    pub fn f1fl(&mut self) -> F1flW<FdcanRxf1sSpec> {
        F1flW::new(self, 0)
    }
    #[doc = "Bits 8:14 - Rx FIFO 1 Get Index"]
    #[inline(always)]
    pub fn f1gi(&mut self) -> F1giW<FdcanRxf1sSpec> {
        F1giW::new(self, 8)
    }
    #[doc = "Bits 16:22 - Rx FIFO 1 Put Index"]
    #[inline(always)]
    pub fn f1pi(&mut self) -> F1piW<FdcanRxf1sSpec> {
        F1piW::new(self, 16)
    }
    #[doc = "Bit 24 - Rx FIFO 1 Full"]
    #[inline(always)]
    pub fn f1f(&mut self) -> F1fW<FdcanRxf1sSpec> {
        F1fW::new(self, 24)
    }
    #[doc = "Bit 25 - Rx FIFO 1 Message Lost"]
    #[inline(always)]
    pub fn rf1l(&mut self) -> Rf1lW<FdcanRxf1sSpec> {
        Rf1lW::new(self, 25)
    }
    #[doc = "Bits 30:31 - Debug Message Status"]
    #[inline(always)]
    pub fn dms(&mut self) -> DmsW<FdcanRxf1sSpec> {
        DmsW::new(self, 30)
    }
}
#[doc = "FDCAN Rx FIFO 1 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxf1s::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf1s::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanRxf1sSpec;
impl crate::RegisterSpec for FdcanRxf1sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxf1s::R`](R) reader structure"]
impl crate::Readable for FdcanRxf1sSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rxf1s::W`](W) writer structure"]
impl crate::Writable for FdcanRxf1sSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_RXF1S to value 0"]
impl crate::Resettable for FdcanRxf1sSpec {}
