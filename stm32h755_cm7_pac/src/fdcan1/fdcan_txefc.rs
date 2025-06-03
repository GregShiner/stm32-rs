#[doc = "Register `FDCAN_TXEFC` reader"]
pub type R = crate::R<FdcanTxefcSpec>;
#[doc = "Register `FDCAN_TXEFC` writer"]
pub type W = crate::W<FdcanTxefcSpec>;
#[doc = "Field `EFSA` reader - Event FIFO Start Address"]
pub type EfsaR = crate::FieldReader<u16>;
#[doc = "Field `EFSA` writer - Event FIFO Start Address"]
pub type EfsaW<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
#[doc = "Field `EFS` reader - Event FIFO Size"]
pub type EfsR = crate::FieldReader;
#[doc = "Field `EFS` writer - Event FIFO Size"]
pub type EfsW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `EFWM` reader - Event FIFO Watermark"]
pub type EfwmR = crate::FieldReader;
#[doc = "Field `EFWM` writer - Event FIFO Watermark"]
pub type EfwmW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 2:15 - Event FIFO Start Address"]
    #[inline(always)]
    pub fn efsa(&self) -> EfsaR {
        EfsaR::new(((self.bits >> 2) & 0x3fff) as u16)
    }
    #[doc = "Bits 16:21 - Event FIFO Size"]
    #[inline(always)]
    pub fn efs(&self) -> EfsR {
        EfsR::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:29 - Event FIFO Watermark"]
    #[inline(always)]
    pub fn efwm(&self) -> EfwmR {
        EfwmR::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 2:15 - Event FIFO Start Address"]
    #[inline(always)]
    pub fn efsa(&mut self) -> EfsaW<FdcanTxefcSpec> {
        EfsaW::new(self, 2)
    }
    #[doc = "Bits 16:21 - Event FIFO Size"]
    #[inline(always)]
    pub fn efs(&mut self) -> EfsW<FdcanTxefcSpec> {
        EfsW::new(self, 16)
    }
    #[doc = "Bits 24:29 - Event FIFO Watermark"]
    #[inline(always)]
    pub fn efwm(&mut self) -> EfwmW<FdcanTxefcSpec> {
        EfwmW::new(self, 24)
    }
}
#[doc = "FDCAN Tx Event FIFO Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_txefc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_txefc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTxefcSpec;
impl crate::RegisterSpec for FdcanTxefcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_txefc::R`](R) reader structure"]
impl crate::Readable for FdcanTxefcSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_txefc::W`](W) writer structure"]
impl crate::Writable for FdcanTxefcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TXEFC to value 0"]
impl crate::Resettable for FdcanTxefcSpec {}
