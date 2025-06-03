#[doc = "Register `FDCAN_TSCC` reader"]
pub type R = crate::R<FdcanTsccSpec>;
#[doc = "Register `FDCAN_TSCC` writer"]
pub type W = crate::W<FdcanTsccSpec>;
#[doc = "Field `TSS` reader - Timestamp Select"]
pub type TssR = crate::FieldReader;
#[doc = "Field `TSS` writer - Timestamp Select"]
pub type TssW<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TCP` reader - Timestamp Counter Prescaler"]
pub type TcpR = crate::FieldReader;
#[doc = "Field `TCP` writer - Timestamp Counter Prescaler"]
pub type TcpW<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1 - Timestamp Select"]
    #[inline(always)]
    pub fn tss(&self) -> TssR {
        TssR::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler"]
    #[inline(always)]
    pub fn tcp(&self) -> TcpR {
        TcpR::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Timestamp Select"]
    #[inline(always)]
    pub fn tss(&mut self) -> TssW<FdcanTsccSpec> {
        TssW::new(self, 0)
    }
    #[doc = "Bits 16:19 - Timestamp Counter Prescaler"]
    #[inline(always)]
    pub fn tcp(&mut self) -> TcpW<FdcanTsccSpec> {
        TcpW::new(self, 16)
    }
}
#[doc = "FDCAN Timestamp Counter Configuration Register\n\nYou can [`read`](crate::Reg::read) this register and get [`fdcan_tscc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdcan_tscc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FdcanTsccSpec;
impl crate::RegisterSpec for FdcanTsccSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fdcan_tscc::R`](R) reader structure"]
impl crate::Readable for FdcanTsccSpec {}
#[doc = "`write(|w| ..)` method takes [`fdcan_tscc::W`](W) writer structure"]
impl crate::Writable for FdcanTsccSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FDCAN_TSCC to value 0"]
impl crate::Resettable for FdcanTsccSpec {}
