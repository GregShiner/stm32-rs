#[doc = "Register `FDCAN_RXF0A` reader"]
pub type R = crate::R<FdcanRxf0aSpec>;
#[doc = "Register `FDCAN_RXF0A` writer"]
pub type W = crate::W<FdcanRxf0aSpec>;
#[doc = "Field `FA01` reader - Rx FIFO 0 Acknowledge Index"]
pub type Fa01R = crate::FieldReader;
#[doc = "Field `FA01` writer - Rx FIFO 0 Acknowledge Index"]
pub type Fa01W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 0:5 - Rx FIFO 0 Acknowledge Index"]
    #[inline(always)]
    pub fn fa01(&self) -> Fa01R {
        Fa01R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Rx FIFO 0 Acknowledge Index"]
    #[inline(always)]
    pub fn fa01(&mut self) -> Fa01W<FdcanRxf0aSpec> {
        Fa01W::new(self, 0)
    }
}
#[doc = "CAN Rx FIFO 0 Acknowledge Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxf0a::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxf0a::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanRxf0aSpec;
impl crate::RegisterSpec for FdcanRxf0aSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxf0a::R`](R) reader structure"]
impl crate::Readable for FdcanRxf0aSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rxf0a::W`](W) writer structure"]
impl crate::Writable for FdcanRxf0aSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_RXF0A to value 0"]
impl crate::Resettable for FdcanRxf0aSpec {}
