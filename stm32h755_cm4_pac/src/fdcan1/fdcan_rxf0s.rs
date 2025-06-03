#[doc = "Register `FDCAN_RXF0S` reader"]
pub type R = crate::R<FdcanRxf0sSpec>;
#[doc = "Register `FDCAN_RXF0S` writer"]
pub type W = crate::W<FdcanRxf0sSpec>;
#[doc = "Field `F0FL` reader - Rx FIFO 0 Fill Level"]
pub type F0flR = crate::FieldReader;
#[doc = "Field `F0FL` writer - Rx FIFO 0 Fill Level"]
pub type F0flW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F0G` reader - Rx FIFO 0 Get Index"]
pub type F0gR = crate::FieldReader;
#[doc = "Field `F0G` writer - Rx FIFO 0 Get Index"]
pub type F0gW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `F0P` reader - Rx FIFO 0 Put Index"]
pub type F0pR = crate::FieldReader;
#[doc = "Field `F0P` writer - Rx FIFO 0 Put Index"]
pub type F0pW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `F0F` reader - Rx FIFO 0 Full"]
pub type F0fR = crate::BitReader;
#[doc = "Field `F0F` writer - Rx FIFO 0 Full"]
pub type F0fW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RF0L` reader - Rx FIFO 0 Message Lost"]
pub type Rf0lR = crate::BitReader;
#[doc = "Field `RF0L` writer - Rx FIFO 0 Message Lost"]
pub type Rf0lW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:6 - Rx FIFO 0 Fill Level"]
    #[inline(always)]
    pub fn f0fl(&self) -> F0flR {
        F0flR::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - Rx FIFO 0 Get Index"]
    #[inline(always)]
    pub fn f0g(&self) -> F0gR {
        F0gR::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 0 Put Index"]
    #[inline(always)]
    pub fn f0p(&self) -> F0pR {
        F0pR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bit 24 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn f0f(&self) -> F0fR {
        F0fR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&self) -> Rf0lR {
        Rf0lR::new(((self.bits >> 25) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:6 - Rx FIFO 0 Fill Level"]
    #[inline(always)]
    pub fn f0fl(&mut self) -> F0flW<FdcanRxf0sSpec> {
        F0flW::new(self, 0)
    }
    #[doc = "Bits 8:13 - Rx FIFO 0 Get Index"]
    #[inline(always)]
    pub fn f0g(&mut self) -> F0gW<FdcanRxf0sSpec> {
        F0gW::new(self, 8)
    }
    #[doc = "Bits 16:21 - Rx FIFO 0 Put Index"]
    #[inline(always)]
    pub fn f0p(&mut self) -> F0pW<FdcanRxf0sSpec> {
        F0pW::new(self, 16)
    }
    #[doc = "Bit 24 - Rx FIFO 0 Full"]
    #[inline(always)]
    pub fn f0f(&mut self) -> F0fW<FdcanRxf0sSpec> {
        F0fW::new(self, 24)
    }
    #[doc = "Bit 25 - Rx FIFO 0 Message Lost"]
    #[inline(always)]
    pub fn rf0l(&mut self) -> Rf0lW<FdcanRxf0sSpec> {
        Rf0lW::new(self, 25)
    }
}
#[doc = "FDCAN Rx FIFO 0 Status Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0s::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf0s::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanRxf0sSpec;
impl crate::RegisterSpec for FdcanRxf0sSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxf0s::R`](R) reader structure"]
impl crate::Readable for FdcanRxf0sSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rxf0s::W`](W) writer structure"]
impl crate::Writable for FdcanRxf0sSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_RXF0S to value 0"]
impl crate::Resettable for FdcanRxf0sSpec {}
