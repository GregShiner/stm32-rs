#[doc = "Register `FDCAN_RXF0C` reader"]
pub type R = crate::R<FdcanRxf0cSpec>;
#[doc = "Register `FDCAN_RXF0C` writer"]
pub type W = crate::W<FdcanRxf0cSpec>;
#[doc = "Field `F0SA` reader - Rx FIFO 0 Start Address"]
pub type F0saR = crate::FieldReader<u16>;
#[doc = "Field `F0SA` writer - Rx FIFO 0 Start Address"]
pub type F0saW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `F0S` reader - Rx FIFO 0 Size"]
pub type F0sR = crate::FieldReader;
#[doc = "Field `F0S` writer - Rx FIFO 0 Size"]
pub type F0sW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `F0WM` reader - FIFO 0 Watermark"]
pub type F0wmR = crate::FieldReader;
#[doc = "Field `F0WM` writer - FIFO 0 Watermark"]
pub type F0wmW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 2:15 - Rx FIFO 0 Start Address"]
    #[inline(always)]
    pub fn f0sa(&self) -> F0saR {
        F0saR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:23 - Rx FIFO 0 Size"]
    #[inline(always)]
    pub fn f0s(&self) -> F0sR {
        F0sR::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - FIFO 0 Watermark"]
    #[inline(always)]
    pub fn f0wm(&self) -> F0wmR {
        F0wmR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Rx FIFO 0 Start Address"]
    #[inline(always)]
    pub fn f0sa(&mut self) -> F0saW<FdcanRxf0cSpec> {
        F0saW::new(self, 2)
    }
    #[doc = "Bits 16:23 - Rx FIFO 0 Size"]
    #[inline(always)]
    pub fn f0s(&mut self) -> F0sW<FdcanRxf0cSpec> {
        F0sW::new(self, 16)
    }
    #[doc = "Bits 24:31 - FIFO 0 Watermark"]
    #[inline(always)]
    pub fn f0wm(&mut self) -> F0wmW<FdcanRxf0cSpec> {
        F0wmW::new(self, 24)
    }
}
#[doc = "FDCAN Rx FIFO 0 Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf0c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanRxf0cSpec;
impl crate::RegisterSpec for FdcanRxf0cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxf0c::R`](R) reader structure"]
impl crate::Readable for FdcanRxf0cSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rxf0c::W`](W) writer structure"]
impl crate::Writable for FdcanRxf0cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_RXF0C to value 0"]
impl crate::Resettable for FdcanRxf0cSpec {}
