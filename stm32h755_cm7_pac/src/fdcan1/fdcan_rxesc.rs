#[doc = "Register `FDCAN_RXESC` reader"]
pub type R = crate::R<FdcanRxescSpec>;
#[doc = "Register `FDCAN_RXESC` writer"]
pub type W = crate::W<FdcanRxescSpec>;
#[doc = "Field `F0DS` reader - Rx FIFO 1 Data Field Size:"]
pub type F0dsR = crate::FieldReader;
#[doc = "Field `F0DS` writer - Rx FIFO 1 Data Field Size:"]
pub type F0dsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `F1DS` reader - Rx FIFO 0 Data Field Size:"]
pub type F1dsR = crate::FieldReader;
#[doc = "Field `F1DS` writer - Rx FIFO 0 Data Field Size:"]
pub type F1dsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `RBDS` reader - Rx Buffer Data Field Size:"]
pub type RbdsR = crate::FieldReader;
#[doc = "Field `RBDS` writer - Rx Buffer Data Field Size:"]
pub type RbdsW<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - Rx FIFO 1 Data Field Size:"]
    #[inline(always)]
    pub fn f0ds(&self) -> F0dsR {
        F0dsR::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:6 - Rx FIFO 0 Data Field Size:"]
    #[inline(always)]
    pub fn f1ds(&self) -> F1dsR {
        F1dsR::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 8:10 - Rx Buffer Data Field Size:"]
    #[inline(always)]
    pub fn rbds(&self) -> RbdsR {
        RbdsR::new(((self.bits >> 8) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Rx FIFO 1 Data Field Size:"]
    #[inline(always)]
    pub fn f0ds(&mut self) -> F0dsW<FdcanRxescSpec> {
        F0dsW::new(self, 0)
    }
    #[doc = "Bits 4:6 - Rx FIFO 0 Data Field Size:"]
    #[inline(always)]
    pub fn f1ds(&mut self) -> F1dsW<FdcanRxescSpec> {
        F1dsW::new(self, 4)
    }
    #[doc = "Bits 8:10 - Rx Buffer Data Field Size:"]
    #[inline(always)]
    pub fn rbds(&mut self) -> RbdsW<FdcanRxescSpec> {
        RbdsW::new(self, 8)
    }
}
#[doc = "FDCAN Rx Buffer Element Size Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_rxesc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_rxesc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanRxescSpec;
impl crate::RegisterSpec for FdcanRxescSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_rxesc::R`](R) reader structure"]
impl crate::Readable for FdcanRxescSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_rxesc::W`](W) writer structure"]
impl crate::Writable for FdcanRxescSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_RXESC to value 0"]
impl crate::Resettable for FdcanRxescSpec {}
