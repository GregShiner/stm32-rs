#[doc = "Register `FDCAN_RXF1C` reader"]
pub type R = crate::R<FdcanRxf1cSpec>;
#[doc = "Register `FDCAN_RXF1C` writer"]
pub type W = crate::W<FdcanRxf1cSpec>;
#[doc = "Field `F1SA` reader - Rx FIFO 1 Start Address"]
pub type F1saR = crate::FieldReader<u16>;
#[doc = "Field `F1SA` writer - Rx FIFO 1 Start Address"]
pub type F1saW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `F1S` reader - Rx FIFO 1 Size"]
pub type F1sR = crate::FieldReader;
#[doc = "Field `F1S` writer - Rx FIFO 1 Size"]
pub type F1sW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `F1WM` reader - Rx FIFO 1 Watermark"]
pub type F1wmR = crate::FieldReader;
#[doc = "Field `F1WM` writer - Rx FIFO 1 Watermark"]
pub type F1wmW<'a, REG> = crate::FieldWriter<'a, REG, 7>;
impl R {
    #[doc = "Bits 2:15 - Rx FIFO 1 Start Address"]
    #[inline(always)]
    pub fn f1sa(&self) -> F1saR {
        F1saR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:22 - Rx FIFO 1 Size"]
    #[inline(always)]
    pub fn f1s(&self) -> F1sR {
        F1sR::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Rx FIFO 1 Watermark"]
    #[inline(always)]
    pub fn f1wm(&self) -> F1wmR {
        F1wmR::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Rx FIFO 1 Start Address"]
    #[inline(always)]
    pub fn f1sa(&mut self) -> F1saW<FdcanRxf1cSpec> {
        F1saW::new(self, 2)
    }
    #[doc = "Bits 16:22 - Rx FIFO 1 Size"]
    #[inline(always)]
    pub fn f1s(&mut self) -> F1sW<FdcanRxf1cSpec> {
        F1sW::new(self, 16)
    }
    #[doc = "Bits 24:30 - Rx FIFO 1 Watermark"]
    #[inline(always)]
    pub fn f1wm(&mut self) -> F1wmW<FdcanRxf1cSpec> {
        F1wmW::new(self, 24)
    }
}
#[doc = "FDCAN Rx FIFO 1 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxf1c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf1c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanRxf1cSpec;
impl crate::RegisterSpec for FdcanRxf1cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxf1c::R`](R) reader structure"]
impl crate::Readable for FdcanRxf1cSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rxf1c::W`](W) writer structure"]
impl crate::Writable for FdcanRxf1cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_RXF1C to value 0"]
impl crate::Resettable for FdcanRxf1cSpec {}
